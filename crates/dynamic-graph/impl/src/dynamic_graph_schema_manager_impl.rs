use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Instant;

use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_trait::async_trait;
use log::debug;
use log::error;
use log::info;
use log::trace;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaBuilder;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_model_core::reactive_graph::core::event::EventProperties::EVENT;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_api::TypeSystemEventTypes;
use springtime_di::Component;
use springtime_di::component_alias;
use uuid::Uuid;

static UUID_TYPE_SYSTEM_CHANGED_EVENT: Uuid = Uuid::from_u128(0x6ba7b8109e1511d150b900c04fe530c7);

fn create_dynamic_schema() -> Arc<RwLock<Option<Arc<Schema>>>> {
    Arc::new(RwLock::new(None))
}

fn create_dynamic_schema_modified() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(true))
}

#[derive(Component)]
pub struct DynamicGraphSchemaManagerImpl {
    dynamic_graph_schema_builder: Arc<dyn DynamicGraphSchemaBuilder + Send + Sync>,

    type_system_event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    #[component(default = "create_dynamic_schema")]
    dynamic_schema: Arc<RwLock<Option<Arc<Schema>>>>,

    #[component(default = "create_dynamic_schema_modified")]
    type_system_modified_state: Arc<AtomicBool>,
}

impl DynamicGraphSchemaManagerImpl {
    async fn generate_dynamic_schema(&self) {
        let dynamic_schema_lock = self.dynamic_schema.clone();
        let type_system_modified_state = self.type_system_modified_state.clone();
        let dynamic_graph_schema_builder = self.dynamic_graph_schema_builder.clone();
        tokio::spawn(async move {
            debug!("Start generating dynamic schema");
            let start = Instant::now();
            match dynamic_graph_schema_builder.build_dynamic_schema() {
                Ok(dynamic_schema) => {
                    let mut guard = dynamic_schema_lock.write().unwrap();
                    *guard = Some(Arc::new(dynamic_schema));
                    type_system_modified_state.store(false, Ordering::Relaxed);
                    let duration = start.elapsed();
                    debug!("Successfully generated dynamic schema in {duration:?}");
                }
                Err(e) => {
                    error!("Failed to generate dynamic schema: {e}");
                }
            }
        });
    }
}

#[async_trait]
#[component_alias]
impl DynamicGraphSchemaManager for DynamicGraphSchemaManagerImpl {
    fn is_type_system_modified(&self) -> bool {
        self.type_system_modified_state.load(Ordering::Relaxed)
    }

    async fn regenerate_dynamic_schema(&self) -> Result<(), SchemaError> {
        trace!("Regenerating dynamic schema");
        match self.dynamic_graph_schema_builder.build_dynamic_schema() {
            Ok(dynamic_schema) => {
                info!("Successfully regenerated dynamic schema");
                trace!("{}", dynamic_schema.sdl());
                let mut guard = self.dynamic_schema.write().unwrap();
                *guard = Some(Arc::new(dynamic_schema));
                self.type_system_modified_state.store(false, Ordering::Relaxed);
                Ok(())
            }
            Err(e) => {
                error!("Failed to regenerate dynamic schema: {e}");
                Err(e)
            }
        }
    }

    async fn regenerate_dynamic_schema_if_modified(&self) -> Result<(), SchemaError> {
        if !self.is_type_system_modified() {
            return Ok(());
        }
        trace!("The type system has been modified. Regenerating the dynamic schema");
        self.regenerate_dynamic_schema().await
    }

    async fn get_dynamic_schema(&self) -> Result<Arc<Schema>, SchemaError> {
        self.regenerate_dynamic_schema_if_modified().await?;
        let guard = self.dynamic_schema.read().unwrap();
        match guard.deref() {
            Some(schema) => Ok(schema.clone()),
            None => {
                error!("Can't get dynamic schema!");
                Err(SchemaError("Dynamic schema is empty".to_string()))
            }
        }
    }
}

#[async_trait]
impl Lifecycle for DynamicGraphSchemaManagerImpl {
    async fn init(&self) {}

    async fn post_init(&self) {
        // Initially generate dynamic schema concurrently
        self.generate_dynamic_schema().await;

        // Listen on type system
        if let Some(event_type_system_changed) = self
            .type_system_event_manager
            .get_type_system_event_instance(TypeSystemEventTypes::TypeSystemChanged)
        {
            let type_system_modified_state = self.type_system_modified_state.clone();
            event_type_system_changed.observe_with_handle(
                EVENT.as_ref(),
                move |v| {
                    if v.is_boolean() && v.as_bool().unwrap() {
                        // The type system has changed -> regenerate the dynamic schema
                        info!("The type system has changed -> regenerate the dynamic schema");
                        type_system_modified_state.store(true, Ordering::Relaxed);
                    }
                },
                UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128(),
            );
        }
    }

    async fn pre_shutdown(&self) {
        if let Some(event_type_system_changed) = self
            .type_system_event_manager
            .get_type_system_event_instance(TypeSystemEventTypes::TypeSystemChanged)
        {
            event_type_system_changed.remove_observer(&EVENT.as_ref(), UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128());
        }
    }

    async fn shutdown(&self) {}
}
