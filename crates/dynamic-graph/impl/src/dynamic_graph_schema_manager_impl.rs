use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Instant;

use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaBuilder;
use async_graphql::dynamic::SchemaError;
use async_trait::async_trait;
use log::debug;
use log::error;
use log::info;
use log::trace;
use springtime_di::Component;
use springtime_di::component_alias;
use uuid::Uuid;

use crate::interface::component::get_interfaces;
use crate::object::entity::mutation::register_entity_type_mutation_objects;
use crate::object::entity::query::register_entity_type_query_objects;
use crate::object::flow::get_flow_mutation_types;
use crate::object::flow::get_flow_types;
use crate::object::relation::mutation::register_relation_type_mutation_objects;
use crate::object::relation::query::register_relation_type_query_objects;
use crate::root::get_mutation;
use crate::root::get_query;
use crate::scalar::get_scalars;
use crate::union::get_unions;
use reactive_graph_dynamic_graph_api::DynamicGraphSchemaManager;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_runtime_model::EventProperties::EVENT;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespaceManager;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_api::TypeSystemEventTypes;

static UUID_TYPE_SYSTEM_CHANGED_EVENT: Uuid = Uuid::from_u128(0x6ba7b8109e1511d150b900c04fe530c7);

fn create_dynamic_schema() -> Arc<RwLock<Option<Arc<Schema>>>> {
    Arc::new(RwLock::new(None))
}

fn create_dynamic_schema_modified() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(true))
}

#[derive(Component)]
pub struct DynamicGraphSchemaManagerImpl {
    type_system_event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,

    namespace_manager: Arc<dyn NamespaceManager + Send + Sync>,

    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,

    reactive_relation_manager: Arc<dyn ReactiveRelationManager + Send + Sync>,

    reactive_flow_manager: Arc<dyn ReactiveFlowManager + Send + Sync>,

    #[component(default = "create_dynamic_schema")]
    dynamic_schema: Arc<RwLock<Option<Arc<Schema>>>>,

    #[component(default = "create_dynamic_schema_modified")]
    type_system_modified_state: Arc<AtomicBool>,
}

async fn build_dynamic_schema(context: SchemaBuilderContext, schema: SchemaBuilder) -> Result<Schema, SchemaError> {
    let mut schema = get_scalars(schema);
    schema = get_interfaces(schema, &context);
    schema = get_unions(schema, &context);
    schema = register_entity_type_query_objects(schema, &context);
    schema = register_entity_type_mutation_objects(schema, &context);
    schema = register_relation_type_query_objects(schema, &context);
    schema = register_relation_type_mutation_objects(schema, &context);
    schema = get_flow_types(schema, &context);
    schema = get_flow_mutation_types(schema, &context);
    schema = get_query(schema, &context);
    schema = get_mutation(schema, &context);
    schema.finish()
}

fn build_dynamic_schema_sync(context: SchemaBuilderContext, schema: SchemaBuilder) -> Result<Schema, SchemaError> {
    let mut schema = get_scalars(schema);
    schema = get_interfaces(schema, &context);
    schema = get_unions(schema, &context);
    schema = register_entity_type_query_objects(schema, &context);
    schema = register_entity_type_mutation_objects(schema, &context);
    schema = register_relation_type_query_objects(schema, &context);
    schema = register_relation_type_mutation_objects(schema, &context);
    schema = get_flow_types(schema, &context);
    schema = get_flow_mutation_types(schema, &context);
    schema = get_query(schema, &context);
    schema = get_mutation(schema, &context);
    schema.finish()
}

impl DynamicGraphSchemaManagerImpl {
    async fn generate_dynamic_schema(&self) {
        let context = self.get_schema_builder_context();
        let schema = self.get_schema_builder();
        let dynamic_schema_lock = self.dynamic_schema.clone();
        let type_system_modified_state = self.type_system_modified_state.clone();
        tokio::spawn(async move {
            debug!("Start generating dynamic schema");
            let start = Instant::now();
            match build_dynamic_schema(context, schema).await {
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

    fn get_schema_builder_context(&self) -> SchemaBuilderContext {
        SchemaBuilderContext::new(
            self.namespace_manager.clone(),
            self.component_manager.clone(),
            self.entity_type_manager.clone(),
            self.relation_type_manager.clone(),
            self.flow_type_manager.clone(),
        )
    }

    fn get_schema_builder(&self) -> SchemaBuilder {
        Schema::build("Query", Some("Mutation"), None)
            .data(self.namespace_manager.clone())
            .data(self.component_manager.clone())
            .data(self.entity_type_manager.clone())
            .data(self.relation_type_manager.clone())
            .data(self.flow_type_manager.clone())
            .data(self.reactive_entity_manager.clone())
            .data(self.reactive_relation_manager.clone())
            .data(self.reactive_flow_manager.clone())
    }

    async fn create_dynamic_schema(&self) -> Result<Schema, SchemaError> {
        build_dynamic_schema(self.get_schema_builder_context(), self.get_schema_builder()).await
    }

    fn create_dynamic_schema_sync(&self) -> Result<Schema, SchemaError> {
        build_dynamic_schema_sync(self.get_schema_builder_context(), self.get_schema_builder())
    }

    async fn regenerate_dynamic_schema(&self) -> Result<(), SchemaError> {
        trace!("Regenerating dynamic schema");
        match self.create_dynamic_schema().await {
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
                &EVENT.property_name(),
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
            event_type_system_changed.remove_observer(&EVENT.property_name(), UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128());
        }
    }

    async fn shutdown(&self) {}
}
