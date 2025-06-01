use crate::field::to_field_value;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_reactive_model_impl::ReactiveEntity;

pub fn entity_export_field() -> Field {
    Field::new("export", TypeRef::named_nn_list_nn("JSON"), move |ctx| {
        FieldFuture::new(async move {
            Ok(Some(FieldValue::list(
                ctx.parent_value
                    .try_downcast_ref::<Vec<ReactiveEntity>>()?
                    .iter()
                    .filter_map(|reactive_entity| serde_json::to_value(reactive_entity).ok())
                    .filter_map(to_field_value),
            )))
        })
    })
}
