#![feature(unsized_tuple_coercion)]
#![feature(concat_idents)]
#![feature(register_tool)]
#![feature(test)]
#![feature(path_file_prefix)]
#![register_tool(tarpaulin)]
#![allow(clippy::map_entry, clippy::module_inception, clippy::too_many_arguments)]

use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use inexor_rgf_core_builder as builder;
use inexor_rgf_core_di as di;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_core_reactive as reactive;

use crate::di::profiles;
use crate::di::Container;
use crate::di::Provider;
use crate::runtime::Runtime;

mod api;
mod config;
mod core_model;
mod graphql;
mod implementation;
mod plugin;
mod rest;
mod runtime;

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

pub async fn main() {
    if let Err(error) = log4rs::init_file("./config/logging.toml", Default::default()) {
        eprintln!("Failed to configure logger: {}", error);
    }

    {
        let mut container = di_container_get::<profiles::Default>();
        let container = &mut container;
        let runtime = Provider::<dyn Runtime>::create(container);

        // Runtime Lifecycle
        runtime.init();
        runtime.post_init();
        runtime.run().await;
        runtime.pre_shutdown();
        runtime.shutdown();
    } // Destruct the application
    tokio::time::sleep(Duration::from_millis(2000)).await;
}

pub fn di_container_get<T>() -> Container<T> {
    Container::<T>::new()
}

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
