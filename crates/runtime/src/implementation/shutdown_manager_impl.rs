use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use std::time;

use async_trait::async_trait;
use inexor_rgf_core_model::PropertyInstanceGetter;
use inexor_rgf_core_model::PropertyTypeDefinition;
use inexor_rgf_model_command::builder::CommandBuilder;
use inexor_rgf_model_command::entity::CommandArg;
use serde_json::json;
use tokio::task;

use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ShutdownManager;
use crate::api::UUID_SHUTDOWN;
use crate::api::UUID_SHUTDOWN_TRIGGER;
use crate::builder::EntityTypeBuilder;
use crate::di::*;
use crate::model::DataType;
use crate::model::ReactivePropertyContainer;
use crate::model_command::component::CommandProperties::COMMAND_NAME;
use crate::model_command::component::COMPONENT_COMMAND;
use crate::model_runtime::ShutdownProperties::DELAY;
use crate::model_runtime::COMPONENT_ACTION;
use crate::model_runtime::COMPONENT_LABELED;
use crate::model_runtime::ENTITY_TYPE_SHUTDOWN;
use crate::model_runtime::PROPERTY_TRIGGER;

#[wrapper]
pub struct ShutdownStateContainer(Arc<RwLock<bool>>);

#[provides]
fn create_shutdown_state() -> ShutdownStateContainer {
    ShutdownStateContainer(Arc::new(RwLock::new(false)))
}

#[component]
pub struct ShutdownManagerImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    shutdown_state: ShutdownStateContainer,
}

#[async_trait]
#[provides]
impl ShutdownManager for ShutdownManagerImpl {
    fn do_shutdown(&self) {
        let mut guard = self.shutdown_state.0.write().unwrap();
        *guard = true;
    }

    fn is_shutdown(&self) -> bool {
        *self.shutdown_state.0.read().unwrap().deref()
    }
}

#[async_trait]
impl Lifecycle for ShutdownManagerImpl {
    async fn init(&self) {
        let entity_type = EntityTypeBuilder::new(ENTITY_TYPE_SHUTDOWN.clone())
            .property(&DELAY.property_name(), DataType::Number)
            .component(&COMPONENT_LABELED.clone())
            .component(&COMPONENT_ACTION.clone())
            .property(PROPERTY_TRIGGER, DataType::Bool)
            .component(&COMPONENT_COMMAND.clone())
            .property(COMMAND_NAME, DataType::String)
            .build();
        let _ = self.entity_type_manager.register(entity_type);

        let shutdown_state = self.shutdown_state.0.clone();
        let handle_id = Some(UUID_SHUTDOWN_TRIGGER.as_u128());
        if let Ok(shutdown_command) = CommandBuilder::new()
            .singleton(&ENTITY_TYPE_SHUTDOWN.clone())
            .help("Shutdown the application")
            .arguments()
            .argument(
                CommandArg::new(DELAY)
                    .short('d')
                    .long("delay")
                    .help("Delay shutdown by M seconds")
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
                handle_id,
            )
            .id(UUID_SHUTDOWN)
            .build()
        {
            let _ = self
                .reactive_entity_instance_manager
                .register_reactive_instance(shutdown_command.get_instance());
        }
    }

    async fn shutdown(&self) {
        // Disconnect reactive streams of the shutdown handler
        if let Some(shutdown_handler) = self.reactive_entity_instance_manager.get(UUID_SHUTDOWN) {
            shutdown_handler.remove_all_observers();
        }
    }
}
