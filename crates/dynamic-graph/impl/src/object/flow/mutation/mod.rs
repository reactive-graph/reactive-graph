use crate::object::flow::mutation::delete::get_flow_delete_field;
use crate::object::flow::mutation::trigger::get_flow_type_trigger_field;
use crate::object::flow::mutation::update::get_flow_update_field;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::SchemaBuilder;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeId;

pub mod delete;
pub mod export;
pub mod trigger;
pub mod update;

pub fn register_flow_type_mutation_objects(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for (flow_ty, flow_type) in context.flow_type_manager.get_all() {
        schema = schema.register(get_flow_mutation_type(&flow_ty, &flow_type, context));
    }
    schema
}

pub fn get_flow_mutation_type(flow_ty: &FlowTypeId, flow_type: &FlowType, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(flow_ty);
    let mut object = Object::new(dy_ty.mutation_type_name());
    if let Some(update_field) = get_flow_update_field(flow_type, context) {
        object = object.field(update_field);
    }
    if let Some(trigger_field) = get_flow_type_trigger_field(flow_type) {
        object = object.field(trigger_field);
    }
    object = object.field(get_flow_delete_field());
    object
}
