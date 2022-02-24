use apollo_router_core::{Plugin, RouterRequest, RouterResponse};
use schemars::JsonSchema;
use serde::Deserialize;
use tower::util::BoxService;
use tower::{BoxError, Layer, ServiceExt};

use crate::services::require_operation_name::ValidateOperationNameLayer;

#[derive(Debug)]
pub struct MetadataValidationPluginError;

impl std::fmt::Display for MetadataValidationPluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encountered Unknown Error Validating Metadata")
    }
}

impl std::error::Error for MetadataValidationPluginError {}

#[derive(Debug)]
pub struct MetadataValidationPlugin {
    name: String,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct MetadataValidationPluginConfiguration {
    name: String,
}

#[async_trait::async_trait]
impl Plugin for MetadataValidationPlugin {
    type Config = MetadataValidationPluginConfiguration;

    fn new(config: Self::Config) -> Result<Self, tower::BoxError> {
        tracing::info!("Initializing: MetadataValidationPlugin -> {}", config.name);
        Ok(MetadataValidationPlugin { name: config.name })
    }

    async fn startup(&mut self) -> Result<(), BoxError> {
        tracing::info!(
            "Starting: {} -> {}",
            stringify!(MetadataValidationPlugin),
            self.name
        );
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), BoxError> {
        tracing::info!(
            "Terminating: {} -> {}",
            stringify!(MetadataValidationPlugin),
            self.name
        );
        Ok(())
    }

    fn router_service(
        &mut self,
        service: BoxService<RouterRequest, RouterResponse, BoxError>,
    ) -> BoxService<RouterRequest, RouterResponse, BoxError> {
        ValidateOperationNameLayer::new().layer(service).boxed()
    }
}
