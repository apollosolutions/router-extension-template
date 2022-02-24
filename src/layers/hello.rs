use apollo_router_core::ConfigurableLayer;
use schemars::JsonSchema;
use serde::Deserialize;
use tower::layer::layer_fn;
use tower::{BoxError, Layer};

pub(crate) struct Hello {}

#[derive(Default, Deserialize, JsonSchema)]
pub(crate) struct Conf {
    name: String,
}

impl<S> Layer<S> for Hello {
    type Service = S;
    fn layer(&self, inner: S) -> Self::Service {
        layer_fn(|s| s).layer(inner)
    }
}

impl ConfigurableLayer for Hello {
    type Config = Conf;

    fn new(configuration: Self::Config) -> Result<Self, BoxError> {
        tracing::info!("Hello Layer Init for Name '{}'!", configuration.name);
        Ok(Self {})
    }
}
