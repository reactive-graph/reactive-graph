use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time;

use serde_json::json;
use tokio::task;

use reactive_graph_command_model::entity::Command;
use reactive_graph_command_model::entity::CommandArg;
use reactive_graph_command_model::CommandArgs;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ShutdownProperties::DELAY;
use reactive_graph_runtime_model::ENTITY_TYPE_SHUTDOWN;
use reactive_graph_runtime_service_api::UUID_SHUTDOWN;

pub(crate) fn shutdown_command(shutdown_state: Arc<AtomicBool>) -> Command {
    let args = CommandArgs::new().arg(
        CommandArg::new(DELAY)
            .short('d')
            .long("delay")
            .help("Delay shutdown by N seconds")
            .required(false),
    );
    let executor = Box::new(move |e: &ReactiveEntity| {
        let delay = e.as_u64(DELAY).unwrap_or(0);
        if delay > 0 {
            let shutdown_in_seconds = time::Duration::from_secs(delay);
            let shutdown_state_deferred = shutdown_state.clone();
            task::spawn(async move {
                tokio::time::sleep(shutdown_in_seconds).await;
                shutdown_state_deferred.store(true, Ordering::Relaxed);
                // let mut guard = shutdown_state_deferred.write().unwrap();
                // *guard = true;
            });
            json!(delay)
        } else {
            shutdown_state.store(true, Ordering::Relaxed);
            // let mut guard = shutdown_state.write().unwrap();
            // *guard = true;
            json!(true)
        }
    });
    Command::builder()
        .ty(ENTITY_TYPE_SHUTDOWN.deref())
        .id(UUID_SHUTDOWN)
        .namespace("core")
        .name("shutdown")
        .description("Shutdown the application")
        .help("Shutdown the application")
        .arguments(args)
        .executor(executor)
        .build()
}
