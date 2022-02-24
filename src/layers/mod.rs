mod hello;
use apollo_router_core::{register_layer, ConfigurableLayer};
use hello::Hello;

pub fn register_layers() {
    register_layer!("apollo.solutions", "hello-world", Hello);
}
