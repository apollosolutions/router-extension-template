use apollo_router_core::plugin_utils;
use apollo_router_core::{RouterRequest, RouterResponse};
use futures::future::BoxFuture;
use tower::{Layer, Service};

#[derive(Clone)]
pub(crate) struct ValidateOperationName {}

pub(crate) struct ValidateOperationNameLayer {}

impl ValidateOperationNameLayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S> Layer<S> for ValidateOperationNameLayer
where
    S: Service<RouterRequest, Response = RouterResponse> + Send,
    <S as Service<RouterRequest>>::Future: Send + 'static,
{
    type Service = ValidateOperationNameService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ValidateOperationNameService { service: inner }
    }
}

pub(crate) struct ValidateOperationNameService<S>
where
    S: Service<RouterRequest, Response = RouterResponse> + Send,
    <S as Service<RouterRequest>>::Future: Send + 'static,
{
    service: S,
}

impl<S> Service<RouterRequest> for ValidateOperationNameService<S>
where
    S: Service<RouterRequest, Response = RouterResponse> + Send,
    <S as Service<RouterRequest>>::Future: Send,
{
    type Response = RouterResponse;

    type Error = S::Error;

    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: RouterRequest) -> Self::Future {
        let maybe_operation_name = req.context.request.body().operation_name.as_ref();

        if maybe_operation_name.is_none()
            || maybe_operation_name.expect("is_none() checked").is_empty()
        {
            tracing::info!("Missing Operation Name; Rejecting");
            let res = plugin_utils::RouterResponse::builder()
                .errors(vec![apollo_router_core::Error {
                    message: "Anonymous operations are not allowed; please supply a name and retry"
                        .to_string(),
                    locations: Default::default(),
                    path: Default::default(),
                    extensions: Default::default(),
                }])
                .context(req.context.into())
                .build()
                .into();
            Box::pin(async { Ok(res) })
        } else {
            tracing::info!(
                "Found Operation Name: `{}`; Allowing",
                maybe_operation_name.expect("checked")
            );
            Box::pin(self.service.call(req))
        }
    }
}
