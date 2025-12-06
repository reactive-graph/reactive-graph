use crate::field::flow::flow_id_field;
use crate::field::namespace_path_field;
use crate::field::property::property_container_property_fields;
use crate::interface_manager_impl::component_type_id_container_component_fields;
use crate::object_type_name::object_type_name;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::FIELD_NAME_JSON_SCHEMA;
use reactive_graph_dynamic_graph_api::FlowQueryObjectFactory;
use reactive_graph_dynamic_graph_api::INTERFACE_FLOW;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct FlowQueryObjectFactoryImpl {
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

impl FlowQueryObjectFactoryImpl {}

#[async_trait]
#[component_alias]
impl FlowQueryObjectFactory for FlowQueryObjectFactoryImpl {
    fn create_query_objects(&self) -> Vec<Object> {
        let mut query_objects = Vec::<Object>::new();
        for (_, flow_type) in self.flow_type_manager.get_all() {
            query_objects.push(self.create_query_object(flow_type));
        }
        query_objects
    }

    fn create_query_object(&self, flow_type: FlowType) -> Object {
        let object_type_name = object_type_name(&flow_type.ty, RootObjectType::Query);
        trace!("Create query object {object_type_name} for {}", &flow_type.ty);
        let mut object = Object::new(object_type_name)
            .description(&flow_type.description)
            .implement(INTERFACE_FLOW)
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

        let entity_ty = flow_type.wrapper_type();
        if let Some(entity_type) = self.entity_type_manager.get(&entity_ty) {
            // `ComponentTypeIdContainer`s implements the interfaces of all components
            // and add a component id field for each component
            object = component_type_id_container_component_fields(&entity_type, object);
            // PropertyTypeContainer adds property fields
            object = property_container_property_fields::<EntityType, ReactiveFlow>(&entity_type, object);
        }

        object
    }
}

#[async_trait]
impl Lifecycle for FlowQueryObjectFactoryImpl {}
