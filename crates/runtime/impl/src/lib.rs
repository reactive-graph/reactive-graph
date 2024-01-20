#![feature(unsized_tuple_coercion)]
#![feature(concat_idents)]
#![feature(register_tool)]
#![feature(test)]
#![feature(path_file_prefix)]
#![feature(result_option_inspect)]
#![register_tool(tarpaulin)]
#![allow(clippy::map_entry, clippy::module_inception, clippy::too_many_arguments)]

// async fn in traits + async closures
// TODO: #![feature(async_fn_in_trait)]
// https://rust-lang.github.io/async-fundamentals-initiative/index.html

// Let rust know to link to the implementations
extern crate inexor_rgf_behaviour_service_impl;
extern crate inexor_rgf_dynamic_graph_impl;
extern crate inexor_rgf_graphql_impl;
extern crate inexor_rgf_instance_system_impl;
extern crate inexor_rgf_plugin_graphql_impl;
extern crate inexor_rgf_plugin_service_impl;
extern crate inexor_rgf_reactive_service_impl;
extern crate inexor_rgf_runtime_graphql_impl;
extern crate inexor_rgf_runtime_service_impl;
extern crate inexor_rgf_runtime_web_impl;
extern crate inexor_rgf_type_system_impl;

pub use builder::*;
pub use runtime_getter::*;
pub use runtime_impl::*;

pub mod builder;
pub mod runtime_getter;
pub mod runtime_impl;

// #[cfg(test)]
// mod tests;
