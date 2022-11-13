#![feature(unsized_tuple_coercion)]
#![feature(concat_idents)]
#![feature(register_tool)]
#![feature(test)]
#![register_tool(tarpaulin)]
#![allow(clippy::map_entry, clippy::module_inception, clippy::too_many_arguments)]

use std::alloc::System;

use inexor_rgf_core_builder as builder;
use inexor_rgf_core_di as di;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_core_reactive as reactive;

use crate::application::Application;
use crate::di::profiles;
use crate::di::Container;
use crate::di::Provider;
use std::thread;
use std::time::Duration;

mod api;
mod application;
mod config;
mod graphql;
mod implementation;
mod plugin;
mod rest;

#[global_allocator]
static ALLOCATOR: System = System;

#[async_std::main]
async fn main() {
    if let Err(error) = log4rs::init_file("config/logging.toml", Default::default()) {
        println!("Failed to configure logger: {}", error);
    }

    {
        let mut container = di_container_get::<profiles::Default>();
        let container = &mut container;
        let mut application = Provider::<dyn Application>::create(container);

        application.init();
        application.post_init();
        application.run().await;

        // let main = async_std::future::ready(application.run);
        // let server = application.clone().serve();
        // let main = application.run();
        // main.await;
        // let futures = vec![main, server];
        // join!(futures).await;
        application.pre_shutdown();
        application.shutdown();
    } // Destruct the application
    thread::sleep(Duration::from_millis(2000));
}

pub fn di_container_get<T>() -> Container<T> {
    Container::<T>::new()
}

#[cfg(test)]
#[tarpaulin::ignore]
pub mod tests;
