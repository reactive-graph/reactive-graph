use crate::object_type_name::object_type_ref;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use log::info;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_model_core::reactive_graph::core::action::ACTION;
use reactive_graph_model_core::reactive_graph::core::action::ActionProperties::TRIGGER;
use serde_json::json;

pub fn create_trigger_field<I: PropertyInstanceSetter + Clone + Send + Sync + 'static, T: ComponentTypeIdContainer + NamespacedTypeGetter>(
    type_: &T,
) -> Option<Field> {
    if !type_.is_a(&ACTION) {
        return None;
    }
    let ty = type_.namespaced_type();
    let ty_inner = ty.clone();
    Some(
        Field::new(TRIGGER.as_ref(), object_type_ref(ty, RootObjectType::Query), move |ctx| {
            let ty_inner = ty_inner.clone();
            FieldFuture::new(async move {
                info!("trigger {ty_inner} enter");
                let reactive_instance = ctx.parent_value.try_downcast_ref::<I>()?;
                info!("trigger {ty_inner} resolved reactive instances");
                reactive_instance.set(TRIGGER.as_ref(), json!(true));
                Ok(Some(FieldValue::owned_any(reactive_instance.clone())))
            })
        })
        .description("Triggers the action"),
    )
}
