#![feature(concat_idents)]
#![feature(test)]
#![allow(clippy::map_entry, clippy::module_inception, clippy::too_many_arguments)]

// async fn in traits + async closures
// TODO: #![feature(async_fn_in_trait)]
// https://rust-lang.github.io/async-fundamentals-initiative/index.html

// Let rust know to link to the implementations
extern crate reactive_graph_behaviour_service_impl;
extern crate reactive_graph_dynamic_graph_impl;
extern crate reactive_graph_graphql_impl;
extern crate reactive_graph_instance_system_impl;
extern crate reactive_graph_plugin_graphql_impl;
extern crate reactive_graph_plugin_service_impl;
extern crate reactive_graph_reactive_service_impl;
extern crate reactive_graph_runtime_graphql_impl;
extern crate reactive_graph_runtime_service_impl;
extern crate reactive_graph_runtime_web_impl;
extern crate reactive_graph_type_system_impl;

pub use builder::*;
pub use runtime_getter::*;
pub use runtime_impl::*;

pub mod builder;
pub mod runtime_getter;
pub mod runtime_impl;

// #[cfg(test)]
// mod tests;
