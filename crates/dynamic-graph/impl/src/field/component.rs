use std::sync::Arc;

use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InterfaceField;
use async_graphql::dynamic::TypeRef;

use crate::field::to_type_ref;
use crate::object::types::DynamicGraphTypeDefinition;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::PropertyType;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;

pub fn component_query_field(component: &Component) -> Field {
    let ty = component.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&ty);
    Field::new(dy_ty.field_name_with_suffix(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
            let entity_instances = entity_instance_manager.get_by_component(&ty).into_iter().map(entity_instance_component);
            let relation_instances = relation_instance_manager.get_by_component(&ty).into_iter().map(relation_instance_component);
            let field_values = entity_instances.chain(relation_instances);
            Ok(Some(FieldValue::list(field_values)))
        })
    })
    .description(component.description.clone())
}

fn entity_instance_component<'a>(entity_instance: ReactiveEntity) -> FieldValue<'a> {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_instance.ty);
    FieldValue::owned_any(entity_instance).with_type(dy_ty.to_string())
}

fn relation_instance_component<'a>(relation_instance: ReactiveRelation) -> FieldValue<'a> {
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_instance.relation_type_id());
    FieldValue::owned_any(relation_instance).with_type(dy_ty.to_string())
}

pub fn instance_component_id_field(ty: &ComponentTypeId) -> Field {
    let ty_inner = ty.clone();
    Field::new(format!("_{ty}"), TypeRef::named(TypeRef::ID), move |_ctx| {
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
    InterfaceField::new(format!("_{ty}"), TypeRef::named(TypeRef::ID))
}
