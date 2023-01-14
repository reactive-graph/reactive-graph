use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_trait::async_trait;
use log::error;
use log::info;
use log::trace;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::DynamicGraphSchemaManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::NamespaceManager;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::core_model::PROPERTY_EVENT;
use crate::di::*;
use crate::graphql::dynamic::get_entity_mutation_types;
use crate::graphql::dynamic::get_entity_types;
use crate::graphql::dynamic::get_interfaces;
use crate::graphql::dynamic::get_mutation;
use crate::graphql::dynamic::get_query;
use crate::graphql::dynamic::get_relation_types;
use crate::graphql::dynamic::get_scalars;
use crate::graphql::dynamic::get_unions;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::model::ReactivePropertyContainer;
use crate::plugins::SystemEventTypes;

static UUID_TYPE_SYSTEM_CHANGED_EVENT: Uuid = Uuid::from_u128(0x6ba7b8109e1511d150b900c04fe530c7);

#[wrapper]
pub struct DynamicSchemaContainer(RwLock<Option<Arc<Schema>>>);

#[provides]
fn create_dynamic_schema() -> DynamicSchemaContainer {
    DynamicSchemaContainer(RwLock::new(None))
}

#[wrapper]
pub struct TypeSystemModifiedStateContainer(Arc<RwLock<bool>>);

#[provides]
fn create_dynamic_schema_modified() -> TypeSystemModifiedStateContainer {
    TypeSystemModifiedStateContainer(Arc::new(RwLock::new(true)))
}

#[component]
pub struct DynamicGraphSchemaManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    namespace_manager: Wrc<dyn NamespaceManager>,

    entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    dynamic_schema: DynamicSchemaContainer,

    type_system_modified_state: TypeSystemModifiedStateContainer,
}

#[async_trait]
#[provides]
impl DynamicGraphSchemaManager for DynamicGraphSchemaManagerImpl {
    fn is_type_system_modified(&self) -> bool {
        *self.type_system_modified_state.0.read().unwrap().deref()
    }

    fn create_dynamic_schema(&self) -> Result<Schema, SchemaError> {
        let context = SchemaBuilderContext::new(
            self.namespace_manager.clone(),
            self.component_manager.clone(),
            self.entity_type_manager.clone(),
            self.relation_type_manager.clone(),
        );
        let mut schema = Schema::build("Query", Some("Mutation"), None)
            .data(self.namespace_manager.clone())
            .data(self.component_manager.clone())
            .data(self.entity_type_manager.clone())
            .data(self.relation_type_manager.clone())
            .data(self.entity_instance_manager.clone())
            .data(self.relation_instance_manager.clone());
        schema = get_scalars(schema);
        schema = get_interfaces(schema, &context);
        schema = get_unions(schema, &context);
        schema = get_entity_types(schema, &context);
        schema = get_entity_mutation_types(schema, &context);
        schema = get_relation_types(schema, &context);
        schema = get_query(schema, &context);
        schema = get_mutation(schema, &context);
        schema.finish()
    }

    fn regenerate_dynamic_schema(&self) {
        trace!("Regenerating dynamic schema");
        match self.create_dynamic_schema() {
            Ok(dynamic_schema) => {
                info!("Successfully regenerated schema");
                trace!("{}", dynamic_schema.sdl());
                let mut guard = self.dynamic_schema.0.write().unwrap();
                *guard = Some(Arc::new(dynamic_schema));
                let mut guard = self.type_system_modified_state.0.write().unwrap();
                *guard = false;
            }
            Err(e) => {
                error!("Failed to regenerate dynamic schema: {}", e);
            }
        }
    }

    fn regenerate_dynamic_schema_if_modified(&self) {
        if self.is_type_system_modified() {
            trace!("The type system has been modified. Regenerating the dynamic schema");
            self.regenerate_dynamic_schema();
        }
    }

    fn get_dynamic_schema(&self) -> Option<Arc<Schema>> {
        let guard = self.dynamic_schema.0.read().unwrap();
        match guard.deref() {
            None => {
                error!("Can't get dynamic schema!");
                None
            }
            Some(schema) => Some(schema.clone()),
        }
    }
}

impl Lifecycle for DynamicGraphSchemaManagerImpl {
    fn init(&self) {}

    fn post_init(&self) {
        self.regenerate_dynamic_schema();
        if let Some(event_type_system_changed) = self.event_manager.get_system_event_instance(SystemEventTypes::TypeSystemChanged) {
            let type_system_modified_state = self.type_system_modified_state.0.clone();
            event_type_system_changed.observe_with_handle(
                PROPERTY_EVENT,
                move |v| {
                    if v.is_boolean() && v.as_bool().unwrap() {
                        // The type system has changed -> regenerate the dynamic schema
                        let mut guard = type_system_modified_state.write().unwrap();
                        *guard = true;
                    }
                },
                UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128(),
            );
        }
    }

    fn pre_shutdown(&self) {
        if let Some(event_type_system_changed) = self.event_manager.get_system_event_instance(SystemEventTypes::TypeSystemChanged) {
            event_type_system_changed.remove_observer(PROPERTY_EVENT, UUID_TYPE_SYSTEM_CHANGED_EVENT.as_u128());
        }
    }

    fn shutdown(&self) {}
}
