use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_graph::RelationType;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_runtime_model::COMPONENT_ACTION;
use serde_json::json;

pub fn relation_trigger_field(relation_type: &RelationType) -> Option<Field> {
    if !relation_type.is_a(&COMPONENT_ACTION) {
        return None;
    }
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let trigger_field = Field::new(TRIGGER.property_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        FieldFuture::new(async move {
            let relation_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveRelation>>()?;
            for relation_instance in relation_instances {
                relation_instance.set(TRIGGER.property_name(), json!(true));
            }
            Ok(Some(FieldValue::list(
                relation_instances
                    .iter()
                    .map(|relation_instance| FieldValue::owned_any(relation_instance.clone())),
            )))
        })
    });
    Some(trigger_field)
}
