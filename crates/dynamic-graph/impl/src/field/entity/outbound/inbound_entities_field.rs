use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use std::sync::Arc;

pub fn outbound_entity_to_inbound_entities_field(
    ty: &RelationTypeId,
    inbound_ty: &EntityTypeId,
    field_name: Option<String>,
    field_description: Option<String>,
) -> Option<Field> {
    let dy_ty = DynamicGraphTypeDefinition::from(inbound_ty);
    let field_name = field_name.unwrap_or(dy_ty.inbound_type_name());
    create_outbound_entity_to_inbound_field(ty, &dy_ty.to_string(), &field_name, field_description)
}

pub fn outbound_entity_to_inbound_entities_union_field(
    ty: &RelationTypeId,
    type_name: &str,
    field_name: Option<String>,
    field_description: Option<String>,
) -> Option<Field> {
    let field_name = field_name.unwrap_or("inbound".to_string());
    if field_name.is_empty() {
        return None;
    }
    let relation_ty_inner = ty.clone();
    let mut field = Field::new(field_name, TypeRef::named_nn_list_nn(type_name), move |ctx| {
        let ty = relation_ty_inner.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_outbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| relation_instance.relation_type_id() == ty)
                    .map(|relation_instance| {
                        let inbound = relation_instance.inbound.clone();
                        let dy_ty = DynamicGraphTypeDefinition::from(&inbound.ty);
                        FieldValue::owned_any(inbound).with_type(dy_ty.to_string())
                    }),
            )))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    Some(field)
}

pub fn outbound_entity_to_inbound_components_field(
    ty: &RelationTypeId,
    component_ty: &ComponentTypeId,
    field_name: Option<String>,
    field_description: Option<String>,
) -> Option<Field> {
    let dy_ty = DynamicGraphTypeDefinition::from(component_ty);
    let field_name = field_name.unwrap_or(dy_ty.inbound_type_name());
    create_outbound_entity_to_inbound_field(ty, &dy_ty.to_string(), &field_name, field_description)
}

pub fn create_outbound_entity_to_inbound_field(ty: &RelationTypeId, type_name: &str, field_name: &str, field_description: Option<String>) -> Option<Field> {
    if field_name.is_empty() {
        return None;
    }
    let relation_ty_inner = ty.clone();
    let mut field = Field::new(field_name, TypeRef::named_nn_list_nn(type_name), move |ctx| {
        let ty = relation_ty_inner.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_outbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| relation_instance.relation_type_id() == ty)
                    .map(|relation_instance| FieldValue::owned_any(relation_instance.inbound.clone())),
            )))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    Some(field)
}
