use crate::field::ids_to_field_value;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::TypeRef;
use log::trace;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use std::sync::Arc;

pub fn entity_delete_field() -> Field {
    Field::new("delete", TypeRef::named_nn_list_nn(TypeRef::ID), move |ctx| {
        FieldFuture::new(async move {
            let reactive_entity_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
            let mut ids = Vec::new();
            for reactive_entity in ctx.parent_value.try_downcast_ref::<Vec<ReactiveEntity>>()? {
                trace!("Deleting entity instance {reactive_entity}");
                let id = reactive_entity.id;
                if reactive_entity_manager.delete(id) {
                    ids.push(id);
                }
            }
            Ok(Some(ids_to_field_value(ids)))
        })
    })
}
