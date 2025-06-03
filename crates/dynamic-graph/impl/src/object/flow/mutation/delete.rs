use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use log::trace;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use std::sync::Arc;

pub fn get_flow_delete_field() -> Field {
    Field::new("delete", TypeRef::named_nn_list_nn(TypeRef::ID), move |ctx| {
        FieldFuture::new(async move {
            let flow_instance_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
            let mut ids = Vec::new();
            for flow_instance in ctx.parent_value.try_downcast_ref::<Vec<ReactiveFlow>>()? {
                trace!("Deleting flow instance {flow_instance}");
                let id = flow_instance.id;
                flow_instance_manager.delete(id);
                ids.push(id);
            }
            Ok(Some(FieldValue::list(ids.iter().map(|id| FieldValue::value(ID(id.to_string()))))))
        })
    })
}
