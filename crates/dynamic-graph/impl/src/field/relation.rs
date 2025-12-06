use crate::type_ref::TYPE_REF_ID;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use reactive_graph_dynamic_graph_api::INTERFACE_RELATION_FIELD_ID;
use reactive_graph_dynamic_graph_api::INTERFACE_RELATION_FIELD_INSTANCE_ID;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_impl::ReactiveRelation;

/// Creates a field with the relation instance id.
/// `{outbound_id}--[{fully_qualified_relation_type_namespace}__{instance_id}]-->{inbound_id}`
pub fn relation_id_field() -> Field {
    Field::new(INTERFACE_RELATION_FIELD_ID, TYPE_REF_ID.clone(), |ctx| {
        FieldFuture::new(async move {
            let reactive_relation = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
            Ok(Some(FieldValue::value(ID(reactive_relation.id().to_string()))))
        })
    })
}

pub fn relation_instance_id_field() -> Field {
    Field::new(INTERFACE_RELATION_FIELD_INSTANCE_ID, TYPE_REF_ID.clone(), |ctx| {
        FieldFuture::new(async move {
            let reactive_relation = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
            Ok(Some(FieldValue::value(ID(reactive_relation.instance_id().to_string()))))
        })
    })
}
