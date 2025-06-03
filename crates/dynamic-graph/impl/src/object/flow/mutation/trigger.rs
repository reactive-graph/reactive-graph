use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_runtime_model::COMPONENT_ACTION;
use serde_json::json;

pub fn get_flow_type_trigger_field(flow_type: &FlowType) -> Option<Field> {
    if !flow_type.wrapper_entity_instance.is_a(&COMPONENT_ACTION) {
        return None;
    }
    let dy_ty = DynamicGraphTypeDefinition::from(&flow_type.ty);
    let trigger_field = Field::new(TRIGGER.property_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        FieldFuture::new(async move {
            let flow_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveFlow>>()?;
            for flow_instance in flow_instances {
                flow_instance.set(TRIGGER.property_name(), json!(true));
            }
            Ok(Some(FieldValue::list(
                flow_instances.iter().map(|flow_instance| FieldValue::owned_any(flow_instance.clone())),
            )))
        })
    });
    Some(trigger_field)
}
