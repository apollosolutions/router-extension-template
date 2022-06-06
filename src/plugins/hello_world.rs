use apollo_router_core::plugin::Plugin;
use apollo_router_core::{
    register_plugin, ExecutionRequest, ExecutionResponse, QueryPlannerRequest,
    QueryPlannerResponse, RouterRequest, RouterResponse, SubgraphRequest, SubgraphResponse,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tower::util::BoxService;
use tower::BoxError;

#[derive(Debug)]
struct HelloWorld {
    #[allow(dead_code)]
    configuration: Conf,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
struct Conf {
    // Put your plugin configuration here. It will automatically be deserialized from JSON.
    // Always put some sort of config here, even if it is just a bool to say that the plugin is enabled,
    // otherwise the yaml to enable the plugin will be confusing.
    message: String,
}
// This is a bare bones plugin that can be duplicated when creating your own.
#[async_trait::async_trait]
impl Plugin for HelloWorld {
    type Config = Conf;

    async fn new(configuration: Self::Config) -> Result<Self, BoxError> {
        tracing::info!("{}", configuration.message);
        Ok(HelloWorld { configuration })
    }

    // Delete this function if you are not customizing it.
    fn router_service(
        &mut self,
        service: BoxService<RouterRequest, RouterResponse, BoxError>,
    ) -> BoxService<RouterRequest, RouterResponse, BoxError> {
        // Always use service builder to compose your plugins.
        // It provides off the shelf building blocks for your plugin.
        //
        // ServiceBuilder::new()
        //             .service(service)
        //             .boxed()

        // Returning the original service means that we didn't add any extra functionality for at this point in the lifecycle.
        service
    }

    // Delete this function if you are not customizing it.
    fn query_planning_service(
        &mut self,
        service: BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError>,
    ) -> BoxService<QueryPlannerRequest, QueryPlannerResponse, BoxError> {
        service
    }

    // Delete this function if you are not customizing it.
    fn execution_service(
        &mut self,
        service: BoxService<ExecutionRequest, ExecutionResponse, BoxError>,
    ) -> BoxService<ExecutionRequest, ExecutionResponse, BoxError> {
        service
    }

    // Delete this function if you are not customizing it.
    fn subgraph_service(
        &mut self,
        _name: &str,
        service: BoxService<SubgraphRequest, SubgraphResponse, BoxError>,
    ) -> BoxService<SubgraphRequest, SubgraphResponse, BoxError> {
        service
    }
}

// This macro allows us to use it in our plugin registry!
// register_plugin takes a group name, and a plugin name.
register_plugin!("router_extension_template", "hello_world", HelloWorld);

#[cfg(test)]
mod tests {
    use super::{Conf, HelloWorld};

    use apollo_router_core::utils::test::IntoSchema::Canned;
    use apollo_router_core::utils::test::PluginTestHarness;
    use apollo_router_core::{Plugin, ResponseBody};
    use tower::BoxError;

    #[tokio::test]
    async fn plugin_registered() {
        apollo_router_core::plugins()
            .get("router_extension_template.hello_world")
            .expect("Plugin not found")
            .create_instance(&serde_json::json!({"message" : "Starting my plugin"}))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn basic_test() -> Result<(), BoxError> {
        // Define a configuration to use with our plugin
        let conf = Conf {
            message: "Starting my plugin".to_string(),
        };

        // Build an instance of our plugin to use in the test harness
        let plugin = HelloWorld::new(conf).await.expect("created plugin");

        // Create the test harness. You can add mocks for individual services, or use prebuilt canned services.
        let mut test_harness = PluginTestHarness::builder()
            .plugin(plugin)
            .schema(Canned)
            .build()
            .await?;

        // Send a request
        let result = test_harness.call_canned().await?;
        if let ResponseBody::GraphQL(graphql) = result.response.body() {
            assert!(graphql.data.is_some());
        } else {
            panic!("expected graphql response")
        }

        Ok(())
    }
}

