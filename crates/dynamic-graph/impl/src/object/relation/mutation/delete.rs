use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use log::trace;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use std::sync::Arc;

pub fn relation_delete_field() -> Field {
    Field::new("delete", TypeRef::named_nn_list_nn(TypeRef::ID), move |ctx| {
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
            let mut ids = Vec::new();
            for reactive_relation in ctx.parent_value.try_downcast_ref::<Vec<ReactiveRelation>>()? {
                trace!("Deleting relation instance {reactive_relation}");
                let id = reactive_relation.id();
                relation_instance_manager.delete(&id);
                ids.push(id);
            }
            Ok(Some(FieldValue::list(ids.iter().map(|id| FieldValue::value(ID(id.to_string()))))))
        })
    })
}
