#![feature(unsized_tuple_coercion)]
#![feature(in_band_lifetimes)]
#![feature(concat_idents)]

use crate::plugin::registry::PluginRegistry;
pub use inexor_rgf_core_plugins as plugins;
use std::alloc::System;

mod plugin;

#[global_allocator]
static ALLOCATOR: System = System;

fn main() {
    let logger_result = log4rs::init_file("config/logging.yml", Default::default());
    match logger_result {
        Err(error) => {
            println!("Failed to configure logger: {}", error);
        }
        _ => {}
    }

    let mut registry = PluginRegistry::new();
    unsafe {
        registry
            .load("/home/aschaeffer/CLionProjects/inexor-rgf-plugin-base/target/debug/libinexor_rgf_plugin_base.so")
            .expect("Failed to load BASE plugin");
        registry
            .load("/home/aschaeffer/CLionProjects/inexor-rgf-plugin-metadata/target/debug/libinexor_rgf_plugin_metadata.so")
            .expect("Failed to load METADATA plugin");

        registry.list();
        registry.init("base");
        registry.init("metadata");
        registry.post_init("base");
        registry.post_init("metadata");
        registry.pre_shutdown("metadata");
        registry.pre_shutdown("base");
        registry.shutdown("metadata");
        registry.shutdown("base");
    }
}
