use crate::field::delete::FIELD_NAME_DELETE;
use crate::field::flow::flow_id_field;
use crate::field::json::id_to_field_value;
use crate::field::namespace_path_field;
use crate::field::property::property_container_update_field_arguments;
use crate::field::property::property_container_update_properties;
use crate::field::property::property_container_validate_input_fields;
use crate::field::trigger::create_trigger_field;
use crate::field::update::FIELD_NAME_UPDATE;
use crate::object_type_name::object_type_name;
use crate::object_type_name::object_type_ref;
use crate::type_ref::TYPE_REF_LIST_OF_IDS;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::FIELD_NAME_JSON_SCHEMA;
use reactive_graph_dynamic_graph_api::FlowMutationObjectFactory;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct FlowMutationObjectFactoryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl FlowMutationObjectFactory for FlowMutationObjectFactoryImpl {
    fn create_mutation_objects(&self) -> Vec<Object> {
        let mut objects = Vec::new();
        for (_, flow_type) in self.flow_type_manager.get_all() {
            objects.push(self.create_mutation_object(flow_type));
        }
        objects
    }

    fn create_mutation_object(&self, flow_type: FlowType) -> Object {
        let object_type_name = object_type_name(&flow_type.ty, RootObjectType::Mutation);
        trace!("Create mutation object {object_type_name} for {}", &flow_type.ty);
        let mut object = Object::new(object_type_name)
            .description(&flow_type.description)
            // Namespace path field
            .field(namespace_path_field(flow_type.namespace()))
            // ID field
            .field(flow_id_field());
        if let Some(field) = self
            .json_schema_field_factory
            .get_json_schema_field(FIELD_NAME_JSON_SCHEMA, &flow_type.type_definition())
        {
            object = object.field(field);
        }
        if let Some(update_field) = self.create_update_field(&flow_type) {
            object = object.field(update_field);
        }
        if let Some(trigger_field) = self.create_trigger_field(&flow_type) {
            object = object.field(trigger_field);
        }
        object = object.field(self.create_delete_field());
        object
    }

    fn create_update_field(&self, flow_type: &FlowType) -> Option<Field> {
        let entity_ty = flow_type.wrapper_type();
        let entity_type = self.entity_type_manager.get(&entity_ty)?;
        let entity_type_inner = entity_type.clone();
        let update_field = Field::new(FIELD_NAME_UPDATE, object_type_ref(&flow_type.ty, RootObjectType::Query), move |ctx| {
            let entity_type = entity_type_inner.clone();
            FieldFuture::new(async move {
                let reactive_flow = ctx.parent_value.try_downcast_ref::<ReactiveFlow>()?;
                // First validate all input fields for mutability and correct datatype
                property_container_validate_input_fields(&entity_type, &ctx)?;
                // Set properties
                property_container_update_properties(&entity_type, reactive_flow, &ctx)?;
                Ok(Some(FieldValue::owned_any(reactive_flow.clone())))
            })
        })
        .description("Updates the entity instance");
        property_container_update_field_arguments(&entity_type, update_field)
    }

    fn create_trigger_field(&self, flow_type: &FlowType) -> Option<Field> {
        self.entity_type_manager
            .get(&flow_type.wrapper_entity_instance.ty)
            .and_then(|entity_type| create_trigger_field::<ReactiveFlow, EntityType>(&entity_type))
    }

    fn create_delete_field(&self) -> Field {
        Field::new(FIELD_NAME_DELETE, TYPE_REF_LIST_OF_IDS.clone(), move |ctx| {
            FieldFuture::new(async move {
                let reactive_flow_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
                let reactive_flow = ctx.parent_value.try_downcast_ref::<ReactiveFlow>()?;
                let id = reactive_flow.id;
                Ok(if reactive_flow_manager.delete(id) {
                    Some(id_to_field_value(id))
                } else {
                    None
                })
            })
        })
    }
}

#[async_trait]
impl Lifecycle for FlowMutationObjectFactoryImpl {}
