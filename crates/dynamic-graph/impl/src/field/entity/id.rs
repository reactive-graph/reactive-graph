use crate::interface::entity::INTERFACE_ENTITY_FIELD_ID;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_reactive_model_impl::ReactiveEntity;

pub fn entity_id_field() -> Field {
    Field::new(INTERFACE_ENTITY_FIELD_ID, TypeRef::named_nn(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(Some(FieldValue::value(ID(entity_instance.id.to_string()))))
        })
    })
    .description("The unique identifier of the entity")
}
