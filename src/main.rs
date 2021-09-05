#![feature(unsized_tuple_coercion)]
#![feature(in_band_lifetimes)]
#![feature(concat_idents)]

use std::alloc::System;

use inexor_rgf_core_behaviour as behaviour;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_core_reactive as reactive;

use crate::application::Application;
use crate::di::di_container;
use std::thread;
use std::time::Duration;
use waiter_di::{profiles, Provider};

mod api;
mod application;
mod builder;
mod di;
mod graphql;
mod implementation;
mod plugin;
mod rest;

#[global_allocator]
static ALLOCATOR: System = System;

#[async_std::main]
async fn main() {
    let logger_result = log4rs::init_file("config/logging.yml", Default::default());
    match logger_result {
        Err(error) => {
            println!("Failed to configure logger: {}", error);
        }
        _ => {}
    }

    {
        let mut container = di_container::get::<profiles::Default>();
        let container = &mut container;
        let mut application = Provider::<dyn Application>::create(container);

        application.init();
        application.run().await;

        // let main = async_std::future::ready(application.run);
        // let server = application.clone().serve();
        // let main = application.run();
        // main.await;
        // let futures = vec![main, server];
        // join!(futures).await;
        application.shutdown();
    } // Destruct the application
    thread::sleep(Duration::from_millis(2000));
}
