use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::ID;

use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::graphql::dynamic::interface::entity::INTERFACE_ENTITY_FIELD_ID;
use crate::graphql::dynamic::to_field_value;
use crate::graphql::dynamic::to_type_ref;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::model::EntityType;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyType;
use crate::model::ReactiveEntityInstance;
use crate::model::RelationType;

pub fn entity_field(entity_type: &EntityType) -> Field {
    let ty = entity_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    Field::new(dy_ty.type_name(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
            let instances = entity_instance_manager.get_by_type(&ty);
            return Ok(Some(FieldValue::list(
                instances.into_iter().map(|entity_instance| FieldValue::owned_any(entity_instance.clone())),
            )));
        })
    })
}

pub fn entity_id_field() -> Field {
    Field::new(INTERFACE_ENTITY_FIELD_ID, TypeRef::named_nn(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            Ok(Some(FieldValue::value(ID(entity_instance.id.to_string()))))
        })
    })
}

pub fn entity_property_field(property_type: &PropertyType) -> Field {
    let property_type_inner = property_type.clone();
    Field::new(&property_type.name, to_type_ref(&property_type.data_type), move |ctx| {
        let property_type = property_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            Ok(entity_instance.get(&property_type.name).and_then(to_field_value))
        })
    })
    .description(&property_type.description)
}

pub fn entity_outbound_field(outbound_relation_type: &RelationType) -> Field {
    let outbound_ty = outbound_relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&outbound_ty);
    let outbound_ty_inner = outbound_ty.clone();
    Field::new(&dy_ty.outbound_type_name(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let outbound_ty = outbound_ty_inner.clone();
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let relation_instances: Vec<FieldValue> = relation_instance_manager
                .get_by_outbound_entity(entity_instance.id)
                .iter()
                .filter(|relation_instance| &outbound_ty.clone() == &relation_instance.relation_type_id())
                .map(|relation_instance| FieldValue::owned_any(relation_instance.clone()))
                .collect();
            Ok(Some(FieldValue::list(relation_instances)))
        })
    })
    .description(&outbound_relation_type.description)
}

pub fn entity_inbound_field(inbound_relation_type: &RelationType) -> Field {
    let inbound_ty = inbound_relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&inbound_ty);
    let inbound_ty_inner = inbound_ty.clone();
    Field::new(&dy_ty.inbound_type_name(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let inbound_ty = inbound_ty_inner.clone();
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let relation_instances: Vec<FieldValue> = relation_instance_manager
                .get_by_inbound_entity(entity_instance.id)
                .iter()
                .filter(|relation_instance| &inbound_ty.clone() == &relation_instance.relation_type_id())
                .map(|relation_instance| FieldValue::owned_any(relation_instance.clone()))
                .collect();
            Ok(Some(FieldValue::list(relation_instances)))
        })
    })
    .description(&inbound_relation_type.description)
}
