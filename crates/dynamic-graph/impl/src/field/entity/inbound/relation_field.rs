use crate::extension::field_description::DynamicGraphFieldDescriptionExtension;
use crate::extension::field_name::DynamicGraphFieldNameExtension;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_graph::RelationType;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use std::sync::Arc;

pub fn entity_inbound_relation_field(
    inbound_relation_type: &RelationType,
    field_names: &DynamicGraphFieldNameExtension,
    field_descriptions: &DynamicGraphFieldDescriptionExtension,
) -> Option<Field> {
    let inbound_ty = inbound_relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&inbound_ty);

    let field_name = field_names.from_inbound_entity_to_relation.clone().unwrap_or(dy_ty.inbound_type_name());
    if field_name.is_empty() {
        return None;
    }
    let field_description = field_descriptions
        .from_inbound_entity_to_relation
        .clone()
        .unwrap_or(inbound_relation_type.description.clone());

    let field = Field::new(field_name, TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let inbound_ty = inbound_ty.clone();
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
            let relation_instances: Vec<FieldValue> = relation_instance_manager
                .get_by_inbound_entity(entity_instance.id)
                .iter()
                .filter(|relation_instance| inbound_ty.clone() == relation_instance.relation_type_id())
                .map(|relation_instance| FieldValue::owned_any(relation_instance.clone()))
                .collect();
            Ok(Some(FieldValue::list(relation_instances)))
        })
    })
    .description(field_description);
    Some(field)
}
