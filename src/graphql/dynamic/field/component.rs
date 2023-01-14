use std::sync::Arc;

use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InterfaceField;
use async_graphql::dynamic::TypeRef;
use async_graphql::ID;

use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::graphql::dynamic::to_type_ref;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::model::PropertyType;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveRelationInstance;

pub fn component_field(component: &Component) -> Field {
    let ty = component.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&component.ty);
    Field::new(dy_ty.type_name_with_suffix(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let entity_instances = entity_instance_manager.get_by_component(&ty).into_iter().map(entity_instance_component);
            let relation_instances = relation_instance_manager.get_by_component(&ty).into_iter().map(relation_instance_component);
            let field_values = entity_instances.chain(relation_instances);
            return Ok(Some(FieldValue::list(field_values)));
        })
    })
    .description(component.description.clone())
}

fn entity_instance_component<'a>(entity_instance: Arc<ReactiveEntityInstance>) -> FieldValue<'a> {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_instance.ty);
    FieldValue::owned_any(entity_instance.clone()).with_type(dy_ty.to_string())
}

fn relation_instance_component<'a>(relation_instance: Arc<ReactiveRelationInstance>) -> FieldValue<'a> {
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_instance.relation_type_id());
    FieldValue::owned_any(relation_instance.clone()).with_type(dy_ty.to_string())
}

pub fn instance_component_id_field(ty: &ComponentTypeId) -> Field {
    let ty_inner = ty.clone();
    Field::new(format!("_{}", ty), TypeRef::named(TypeRef::ID), move |_ctx| {
        let ty = ty_inner.clone();
        FieldFuture::new(async move {
            let dy_ty = DynamicGraphTypeDefinition::from(&ty);
            Ok(Some(FieldValue::value(ID(dy_ty.to_string()))))
        })
    })
}

pub fn component_property_field(property_type: &PropertyType) -> InterfaceField {
    InterfaceField::new(&property_type.name, to_type_ref(&property_type.data_type)).description(&property_type.description)
}

pub fn component_id_field(ty: &ComponentTypeId) -> InterfaceField {
    InterfaceField::new(format!("_{}", ty), TypeRef::named(TypeRef::ID))
}
