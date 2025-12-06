use crate::type_ref::TYPE_REF_ID;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use reactive_graph_dynamic_graph_api::INTERFACE_FLOW_FIELD_ID;
use reactive_graph_reactive_model_impl::ReactiveFlow;

pub fn flow_id_field() -> Field {
    Field::new(INTERFACE_FLOW_FIELD_ID, TYPE_REF_ID.clone(), |ctx| {
        FieldFuture::new(async move {
            let reactive_flow = ctx.parent_value.try_downcast_ref::<ReactiveFlow>()?;
            Ok(Some(FieldValue::value(ID(reactive_flow.id.to_string()))))
        })
    })
    .description("The unique identifier of the flow instance.")
}
