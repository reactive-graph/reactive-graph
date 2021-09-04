pub use inexor_rgf_core_plugins as plugins;
use std::alloc::System;
use crate::plugin::registry::PluginRegistry;

mod plugin;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let logger_result = log4rs::init_file("config/logging.yml", Default::default());
    match logger_result {
        Err(error) => {
            println!("Failed to configure logger: {}", error);
        },
        _ => {}
    }


    let mut registry = PluginRegistry::new();
    unsafe {
        registry
            .load("/home/aschaeffer/CLionProjects/inexor-rgf-plugin-base/target/debug/libinexor_rgf_plugin_base.so")
            .expect("Failed to load BASE plugin");

        registry.init("base");
    }
}
