use apollo_router_core::{
    ExecutionRequest, ExecutionResponse, Plugin, QueryPlannerRequest, QueryPlannerResponse,
    RouterRequest, RouterResponse, SubgraphRequest, SubgraphResponse,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use tower::util::BoxService;
use tower::BoxError;

#[derive(Debug)]
pub struct HelloError;

impl fmt::Display for HelloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HelloError")
    }
}

impl Error for HelloError {}

#[derive(Debug)]
pub struct Hello {
    name: String,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct Configuration {
    name: String,
}

#[async_trait::async_trait]
impl Plugin for Hello {
    type Config = Configuration;

    fn new(configuration: Self::Config) -> Result<Self, BoxError> {
        tracing::info!("Hello Plugin Init for Name '{}'!", configuration.name);
        Ok(Hello {
            name: configuration.name,
        })
    }

    async fn startup(&mut self) -> Result<(), BoxError> {
        tracing::info!("starting: {}: {}", stringify!(Hello), self.name);
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), BoxError> {
        tracing::info!("shutting down: {}: {}", stringify!(Hello), self.name);
        Ok(())
    }

    fn router_service(
        &mut self,
        service: BoxService<RouterRequest, RouterResponse, BoxError>,
    ) -> BoxService<RouterRequest, RouterResponse, BoxError> {
        return service;
    }

    fn query_planning_service(
        &mut self,
        service: BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError>,
    ) -> BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError> {
        return service;
    }

    fn execution_service(
        &mut self,
        service: BoxService<ExecutionRequest, ExecutionResponse, BoxError>,
    ) -> BoxService<ExecutionRequest, ExecutionResponse, BoxError> {
        return service;
    }

    fn subgraph_service(
        &mut self,
        _name: &str,
        service: BoxService<SubgraphRequest, SubgraphResponse, BoxError>,
    ) -> BoxService<SubgraphRequest, SubgraphResponse, BoxError> {
        return service;
    }
}
