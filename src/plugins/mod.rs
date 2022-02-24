pub mod hello;
mod metadata_validation;

pub use hello::Hello;
use metadata_validation::MetadataValidationPlugin;

use apollo_router_core::{register_plugin, Plugin};

pub fn register_plugins() {
    register_plugin!(
        "apollo.solutions",
        "metadata-validation",
        MetadataValidationPlugin
    );
    register_plugin!("apollo.solutions", "hello-world", Hello);
}
