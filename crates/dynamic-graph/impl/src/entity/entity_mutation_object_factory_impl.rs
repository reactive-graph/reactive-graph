use crate::field::delete::FIELD_NAME_DELETE;
use crate::field::entity::entity_id_field;
use crate::field::export::create_export_field;
use crate::field::json::id_to_field_value;
use crate::field::namespace_path_field;
use crate::field::property::property_container_update_field_arguments;
use crate::field::property::property_container_update_properties;
use crate::field::property::property_container_validate_input_fields;
use crate::field::trigger::create_trigger_field;
use crate::field::update::FIELD_NAME_UPDATE;
use crate::object_type_name::object_type_name;
use crate::object_type_name::object_type_ref;
use crate::type_ref::TYPE_REF_ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::EntityMutationObjectFactory;
use reactive_graph_dynamic_graph_api::FIELD_NAME_JSON_SCHEMA;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_type_system_api::EntityTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct EntityMutationObjectFactoryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl EntityMutationObjectFactory for EntityMutationObjectFactoryImpl {
    fn create_mutation_objects(&self) -> Vec<Object> {
        let mut objects = Vec::new();
        for (_, entity_type) in self.entity_type_manager.get_all() {
            objects.push(self.create_mutation_object(entity_type));
        }
        objects
    }

    fn create_mutation_object(&self, entity_type: EntityType) -> Object {
        let object_type_name = object_type_name(&entity_type.ty, RootObjectType::Mutation);
        trace!("Create mutation object {object_type_name} for {}", &entity_type.ty);
        let mut object = Object::new(object_type_name)
            .description(&entity_type.description)
            // Namespace path field
            .field(namespace_path_field(entity_type.namespace()))
            // ID field
            .field(entity_id_field());
        if let Some(field) = self
            .json_schema_field_factory
            .get_json_schema_field(FIELD_NAME_JSON_SCHEMA, &entity_type.type_definition())
        {
            object = object.field(field);
        }
        if let Some(update_field) = self.create_update_field(&entity_type) {
            object = object.field(update_field);
        }
        if let Some(trigger_field) = self.create_trigger_field(&entity_type) {
            object = object.field(trigger_field);
        }
        object = object.field(self.create_export_field());
        object = object.field(self.create_delete_field());
        object
    }

    fn create_update_field(&self, entity_type: &EntityType) -> Option<Field> {
        let entity_type_inner = entity_type.clone();
        let update_field = Field::new(FIELD_NAME_UPDATE, object_type_ref(&entity_type.ty, RootObjectType::Query), move |ctx| {
            let entity_type = entity_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_entity = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
                // First validate all input fields for mutability and correct datatype
                property_container_validate_input_fields(&entity_type, &ctx)?;
                // Set properties
                property_container_update_properties(&entity_type, reactive_entity, &ctx)?;
                Ok(Some(FieldValue::owned_any(reactive_entity.clone())))
            })
        })
        .description("Updates the entity instance");
        property_container_update_field_arguments(entity_type, update_field)
    }

    fn create_trigger_field(&self, entity_type: &EntityType) -> Option<Field> {
        create_trigger_field::<ReactiveEntity, EntityType>(&entity_type)
    }

    fn create_export_field(&self) -> Field {
        create_export_field::<ReactiveEntity>()
    }

    fn create_delete_field(&self) -> Field {
        Field::new(FIELD_NAME_DELETE, TYPE_REF_ID.clone(), move |ctx| {
            FieldFuture::new(async move {
                let reactive_entity_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
                let reactive_entity = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
                let id = reactive_entity.id;
                Ok(if reactive_entity_manager.delete(id) {
                    Some(id_to_field_value(id))
                } else {
                    None
                })
            })
        })
        .description("Deletes the entity instance")
    }
}

#[async_trait]
impl Lifecycle for EntityMutationObjectFactoryImpl {}
