use crate::field::delete::FIELD_NAME_DELETE;
use crate::field::export::create_export_field;
use crate::field::namespace_path_field;
use crate::field::property::property_container_update_field_arguments;
use crate::field::property::property_container_update_properties;
use crate::field::property::property_container_validate_input_fields;
use crate::field::relation::relation_id_field;
use crate::field::relation::relation_instance_id_field;
use crate::field::trigger::create_trigger_field;
use crate::field::update::FIELD_NAME_UPDATE;
use crate::object_type_name::object_type_name;
use crate::object_type_name::object_type_ref;
use crate::type_ref::TYPE_REF_ID;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::FIELD_NAME_JSON_SCHEMA;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RelationMutationObjectFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct RelationMutationObjectFactoryImpl {
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RelationMutationObjectFactory for RelationMutationObjectFactoryImpl {
    fn create_mutation_objects(&self) -> Vec<Object> {
        let mut objects = Vec::new();
        for (_, relation_type) in self.relation_type_manager.get_all() {
            objects.push(self.create_mutation_object(relation_type));
        }
        objects
    }

    fn create_mutation_object(&self, relation_type: RelationType) -> Object {
        let object_type_name = object_type_name(&relation_type.ty, RootObjectType::Mutation);
        trace!("Create mutation object for entity type {object_type_name}");
        let mut object = Object::new(object_type_name)
            .description(&relation_type.description)
            // Namespace path field
            .field(namespace_path_field(relation_type.namespace()))
            // Relation ID field
            .field(relation_id_field())
            // Instance ID field
            .field(relation_instance_id_field());
        if let Some(field) = self
            .json_schema_field_factory
            .get_json_schema_field(FIELD_NAME_JSON_SCHEMA, &relation_type.type_definition())
        {
            object = object.field(field);
        }
        if let Some(update_field) = self.create_update_field(&relation_type) {
            object = object.field(update_field);
        }
        if let Some(trigger_field) = self.create_trigger_field(&relation_type) {
            object = object.field(trigger_field);
        }
        object = object.field(self.create_export_field());
        object = object.field(self.create_delete_field());
        object
    }

    fn create_update_field(&self, relation_type: &RelationType) -> Option<Field> {
        let relation_type_inner = relation_type.clone();
        let update_field = Field::new(FIELD_NAME_UPDATE, object_type_ref(&relation_type.ty, RootObjectType::Query), move |ctx| {
            let relation_type = relation_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_relation = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
                // First validate all input fields for mutability and correct datatype
                property_container_validate_input_fields(&relation_type, &ctx)?;
                // Set properties
                property_container_update_properties(&relation_type, reactive_relation, &ctx)?;
                Ok(Some(FieldValue::owned_any(reactive_relation.clone())))
            })
        })
        .description("Updates the entity instance");
        property_container_update_field_arguments(relation_type, update_field)
    }

    fn create_trigger_field(&self, relation_type: &RelationType) -> Option<Field> {
        create_trigger_field::<ReactiveRelation, RelationType>(&relation_type)
    }

    fn create_export_field(&self) -> Field {
        create_export_field::<ReactiveRelation>()
    }

    fn create_delete_field(&self) -> Field {
        Field::new(FIELD_NAME_DELETE, TYPE_REF_ID.clone(), move |ctx| {
            FieldFuture::new(async move {
                let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
                let reactive_relation = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
                let id = reactive_relation.id();
                Ok(if relation_instance_manager.delete(&id) {
                    Some(FieldValue::value(ID(id.to_string())))
                } else {
                    None
                })
            })
        })
    }
}

#[async_trait]
impl Lifecycle for RelationMutationObjectFactoryImpl {}
