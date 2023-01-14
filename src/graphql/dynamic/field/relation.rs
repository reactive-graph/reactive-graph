use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::ID;

use crate::api::ReactiveRelationInstanceManager;
use crate::graphql::dynamic::namespace_entities_union_type_name;
use crate::graphql::dynamic::to_field_value;
use crate::graphql::dynamic::to_type_ref;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::graphql::dynamic::INTERFACE_RELATION_FIELD_INSTANCE_ID;
use crate::graphql::dynamic::INTERFACE_RELATION_FIELD_KEY;
use crate::graphql::dynamic::UNION_ALL_ENTITIES;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyType;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationType;

pub fn relation_query_field(relation_type: &RelationType) -> Field {
    let ty = relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    Field::new(dy_ty.field_name(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let instances = relation_instance_manager.get_by_type(&ty);
            return Ok(Some(FieldValue::list(
                instances.into_iter().map(|relation_instance| FieldValue::owned_any(relation_instance.clone())),
            )));
        })
    })
    .description(relation_type.description.clone())
}

pub fn relation_mutation_field(relation_type: &RelationType) -> Option<Field> {
    None
}

pub fn relation_key_field() -> Field {
    Field::new(INTERFACE_RELATION_FIELD_KEY, TypeRef::named_nn(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveRelationInstance>>()?;
            Ok(Some(FieldValue::value(ID(format!("{}", relation_instance.ty)))))
        })
    })
}

pub fn relation_instance_id_field() -> Field {
    Field::new(INTERFACE_RELATION_FIELD_INSTANCE_ID, TypeRef::named_nn(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveRelationInstance>>()?;
            Ok(Some(FieldValue::value(ID(relation_instance.instance_id()))))
        })
    })
}

pub fn relation_property_field(property_type: &PropertyType) -> Field {
    let property_type_inner = property_type.clone();
    Field::new(&property_type.name, to_type_ref(&property_type.data_type), move |ctx| {
        let property_type = property_type_inner.clone();
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveRelationInstance>>()?;
            Ok(relation_instance.get(&property_type.name).and_then(to_field_value))
        })
    })
    .description(&property_type.description)
}

pub fn relation_outbound_field(
    ty: &ComponentOrEntityTypeId,
    field_name: Option<String>,
    field_description: Option<String>,
    context: &SchemaBuilderContext,
) -> Vec<Field> {
    match ty {
        ComponentOrEntityTypeId::EntityType(ty) => {
            if ty.namespace() == "*" {
                vec![relation_outbound_entity_union_field(&UNION_ALL_ENTITIES, field_name, field_description)]
            } else if ty.type_name() == "*" {
                vec![relation_outbound_entity_union_field(
                    &namespace_entities_union_type_name(&ty.namespace()),
                    field_name,
                    field_description,
                )]
            } else {
                vec![relation_outbound_entity_field(ty, field_name, field_description)]
            }
        }
        ComponentOrEntityTypeId::Component(ty) => {
            if ty.namespace() == "*" {
                context
                    .component_manager
                    .get_all()
                    .into_iter()
                    .map(|component| component.ty)
                    .map(|ty| relation_outbound_component_field(&ty, None, None))
                    .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_by_namespace(&ty.namespace())
                    .into_iter()
                    .map(|component| component.ty)
                    .map(|ty| relation_outbound_component_field(&ty, None, None))
                    .collect()
            } else {
                vec![relation_outbound_component_field(ty, field_name, field_description)]
            }
        }
    }
}

pub fn relation_inbound_field(
    ty: &ComponentOrEntityTypeId,
    field_name: Option<String>,
    field_description: Option<String>,
    context: &SchemaBuilderContext,
) -> Vec<Field> {
    match ty {
        ComponentOrEntityTypeId::EntityType(ty) => {
            if ty.namespace() == "*" {
                vec![relation_inbound_entity_union_field(&UNION_ALL_ENTITIES, field_name, field_description)]
            } else if ty.type_name() == "*" {
                vec![relation_inbound_entity_union_field(
                    &namespace_entities_union_type_name(&ty.namespace()),
                    field_name,
                    field_description,
                )]
            } else {
                vec![relation_inbound_entity_field(ty, field_name, field_description)]
            }
        }
        ComponentOrEntityTypeId::Component(ty) => {
            if ty.namespace() == "*" {
                context
                    .component_manager
                    .get_all()
                    .into_iter()
                    .map(|component| component.ty)
                    .map(|ty| relation_inbound_component_field(&ty, None, None))
                    .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_by_namespace(&ty.namespace())
                    .into_iter()
                    .map(|component| component.ty)
                    .map(|ty| relation_inbound_component_field(&ty, None, None))
                    .collect()
            } else {
                vec![relation_inbound_component_field(ty, field_name, field_description)]
            }
        }
    }
}

pub fn relation_outbound_entity_field(ty: &EntityTypeId, field_name: Option<String>, field_description: Option<String>) -> Field {
    let dy_ty = DynamicGraphTypeDefinition::from(ty);
    let field_name = field_name.unwrap_or(dy_ty.outbound_type_name());
    create_relation_outbound_field(&dy_ty.to_string(), &field_name, field_description)
}

pub fn relation_outbound_entity_union_field(type_name: &str, field_name: Option<String>, field_description: Option<String>) -> Field {
    let field_name = field_name.unwrap_or("outbound".to_string());
    let mut field = Field::new(field_name, TypeRef::named_nn(type_name), move |ctx| {
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveRelationInstance>>()?;
            let dy_ty = DynamicGraphTypeDefinition::from(&relation_instance.outbound.ty);
            Ok(Some(FieldValue::owned_any(relation_instance.outbound.clone()).with_type(dy_ty.to_string())))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    field
}

pub fn relation_outbound_component_field(ty: &ComponentTypeId, field_name: Option<String>, field_description: Option<String>) -> Field {
    let dy_ty = DynamicGraphTypeDefinition::from(ty);
    let field_name = field_name.unwrap_or(dy_ty.outbound_type_name());
    create_relation_outbound_field(&dy_ty.to_string(), &field_name, field_description)
}

pub fn create_relation_outbound_field(type_name: &str, field_name: &str, field_description: Option<String>) -> Field {
    let mut field = Field::new(field_name, TypeRef::named_nn(type_name), move |ctx| {
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveRelationInstance>>()?;
            Ok(Some(FieldValue::owned_any(relation_instance.outbound.clone())))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    field
}

pub fn relation_inbound_entity_field(ty: &EntityTypeId, field_name: Option<String>, field_description: Option<String>) -> Field {
    let dy_ty = DynamicGraphTypeDefinition::from(ty);
    let field_name = field_name.unwrap_or(dy_ty.inbound_type_name());
    create_relation_inbound_field(&dy_ty.to_string(), &field_name, field_description)
}

pub fn relation_inbound_entity_union_field(type_name: &str, field_name: Option<String>, field_description: Option<String>) -> Field {
    let field_name = field_name.unwrap_or("inbound".to_string());
    let mut field = Field::new(field_name, TypeRef::named_nn(type_name), move |ctx| {
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveRelationInstance>>()?;
            let dy_ty = DynamicGraphTypeDefinition::from(&relation_instance.inbound.ty);
            Ok(Some(FieldValue::owned_any(relation_instance.inbound.clone()).with_type(dy_ty.to_string())))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    field
}

pub fn relation_inbound_component_field(ty: &ComponentTypeId, field_name: Option<String>, field_description: Option<String>) -> Field {
    let dy_ty = DynamicGraphTypeDefinition::from(ty);
    let field_name = field_name.unwrap_or(dy_ty.inbound_type_name());
    create_relation_inbound_field(&dy_ty.to_string(), &field_name, field_description)
}

pub fn create_relation_inbound_field(type_name: &str, field_name: &str, field_description: Option<String>) -> Field {
    let mut field = Field::new(field_name, TypeRef::named_nn(type_name), move |ctx| {
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveRelationInstance>>()?;
            Ok(Some(FieldValue::owned_any(relation_instance.inbound.clone())))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    field
}
