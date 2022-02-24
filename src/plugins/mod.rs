pub mod hello;
pub use hello::Hello;

use apollo_router_core::{register_plugin, Plugin};

pub fn register_plugins() {
    register_plugin!("apollo.solutions", "hello-world", Hello);
}
