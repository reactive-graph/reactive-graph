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

#[macro_use]
extern crate query_interface;

use std::future::Future;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use inexor_rgf_behaviour as behaviour;
use inexor_rgf_core_config as config;
use inexor_rgf_core_di as di;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_reactive as reactive;
use inexor_rgf_model_command as model_command;
use inexor_rgf_model_dynamic_graph as model_dynamic_graph;
use inexor_rgf_model_flow as model_flow;
use inexor_rgf_model_runtime as model_runtime;

#[cfg(test)]
use inexor_rgf_test_utils as test_utils;

use crate::di::profiles;
use crate::di::Container;
use crate::di::Provider;
pub use crate::runtime::builder::RuntimeBuilder;
pub use crate::runtime::Runtime;

mod api;
mod commands;
mod error;
mod graphql;
mod implementation;
mod plugin;
mod rest;
pub mod runtime;

pub fn get_runtime() -> Arc<dyn Runtime> {
    let mut container = di_container_get::<profiles::Default>();
    let container = &mut container;
    Arc::new(Provider::<dyn Runtime>::create(container))
}

pub fn get_rw_runtime() -> Arc<RwLock<dyn Runtime>> {
    let mut container = di_container_get::<profiles::Default>();
    let container = &mut container;
    Arc::new(RwLock::new(Provider::<dyn Runtime>::create(container)))
}

pub async fn main<F1, F2, C1, C2>(pre_config: C1, post_config: C2)
where
    F1: Future<Output = ()>,
    F2: Future<Output = ()>,
    C1: FnOnce(Arc<dyn Runtime>) -> F1,
    C2: FnOnce(Arc<dyn Runtime>) -> F2,
{
    {
        let runtime = get_runtime();
        // Runtime Configuration Phase
        pre_config(runtime.clone()).await;
        runtime.config().await;
        post_config(runtime.clone()).await;
        // Runtime Lifecycle
        runtime.init().await;
        runtime.post_init().await;
        runtime.run().await;
        runtime.pre_shutdown().await;
        runtime.shutdown().await;
    } // Destruct the whole runtime
    tokio::time::sleep(Duration::from_millis(2000)).await;
}

pub fn di_container_get<T>() -> Container<T> {
    Container::<T>::new()
}

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
