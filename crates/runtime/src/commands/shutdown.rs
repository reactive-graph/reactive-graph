use std::sync::Arc;
use std::sync::RwLock;
use std::time;

use serde_json::json;
use tokio::task;

use crate::model::PropertyInstanceGetter;
use crate::model_command::builder::CommandBuilder;
use crate::model_command::entity::Command;
use crate::model_command::entity::CommandArg;
use crate::model_command::error::CommandBuilderError;
use crate::model_runtime::ShutdownProperties::DELAY;
use crate::model_runtime::ENTITY_TYPE_SHUTDOWN;

use crate::api::UUID_SHUTDOWN;
use crate::api::UUID_SHUTDOWN_TRIGGER;

pub(crate) fn shutdown_command(shutdown_state: Arc<RwLock<bool>>) -> Result<Command, CommandBuilderError> {
    CommandBuilder::new()
        .singleton(&ENTITY_TYPE_SHUTDOWN.clone())
        .help("Shutdown the application")
        .arguments()
        .argument(
            CommandArg::new(DELAY)
                .short('d')
                .long("delay")
                .help("Delay shutdown by N seconds")
                .required(false),
            json!(0),
        )
        .no_properties()
        .executor_with_handle(
            move |e| {
                let delay = e.as_u64(DELAY).unwrap_or(0);
                if delay > 0 {
                    let shutdown_in_seconds = time::Duration::from_secs(delay);
                    let shutdown_state_deferred = shutdown_state.clone();
                    task::spawn(async move {
                        tokio::time::sleep(shutdown_in_seconds).await;
                        let mut guard = shutdown_state_deferred.write().unwrap();
                        *guard = true;
                    });
                    json!(delay)
                } else {
                    let mut guard = shutdown_state.write().unwrap();
                    *guard = true;
                    json!(true)
                }
            },
            Some(UUID_SHUTDOWN_TRIGGER.as_u128()),
        )
        .id(UUID_SHUTDOWN)
        .build()
}
