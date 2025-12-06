use crate::type_ref::TYPE_REF_ID;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use reactive_graph_dynamic_graph_api::INTERFACE_ENTITY_FIELD_ID;
use reactive_graph_reactive_model_impl::ReactiveEntity;

pub fn entity_id_field() -> Field {
    Field::new(INTERFACE_ENTITY_FIELD_ID, TYPE_REF_ID.clone(), |ctx| {
        FieldFuture::new(async move {
            let reactive_entity = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(Some(FieldValue::value(ID(reactive_entity.id.to_string()))))
        })
    })
    .description("The unique identifier of the entity instance.")
}
