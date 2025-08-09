use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_runtime_model::COMPONENT_ACTION;
use serde_json::json;

pub fn entity_trigger_field(entity_type: &EntityType) -> Option<Field> {
    if !entity_type.is_a(&COMPONENT_ACTION) {
        return None;
    }
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    Some(
        Field::new(TRIGGER.property_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
            FieldFuture::new(async move {
                let reactive_entities = ctx.parent_value.try_downcast_ref::<Vec<ReactiveEntity>>()?;
                for reactive_entity in reactive_entities {
                    reactive_entity.set(TRIGGER.property_name(), json!(true));
                }
                Ok(Some(FieldValue::list(
                    reactive_entities.iter().map(|reactive_entity| FieldValue::owned_any(reactive_entity.clone())),
                )))
            })
        })
        .description("Triggers the action"),
    )
}
