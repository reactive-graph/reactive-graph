use std::str::FromStr;
use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::Error;
use async_graphql::ID;
use inexor_rgf_rt_api::EntityInstanceIsNotOfType;
use inexor_rgf_rt_api::EntityInstanceNotFound;
use log::trace;
use serde_json::Value;
use uuid::Uuid;

use crate::api::ReactiveEntityManager;
use crate::api::ReactiveRelationManager;
use crate::graphql::dynamic::create_properties_from_field_arguments;
use crate::graphql::dynamic::field_description::DynamicGraphFieldDescriptionExtension;
use crate::graphql::dynamic::field_name::DynamicGraphFieldNameExtension;
use crate::graphql::dynamic::interface::entity::INTERFACE_ENTITY_FIELD_ID;
use crate::graphql::dynamic::namespace_entities_union_type_name;
use crate::graphql::dynamic::to_field_value;
use crate::graphql::dynamic::to_input_type_ref;
use crate::graphql::dynamic::to_type_ref;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::graphql::dynamic::UNION_ALL_ENTITIES;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::DataType;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyType;
use crate::model::PropertyTypeDefinition;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::model_runtime::LabeledProperties::LABEL;
use crate::reactive::ReactiveEntity;
use crate::reactive::ReactiveProperties;

pub fn entity_query_field(entity_ty: &EntityTypeId, entity_type: &EntityType) -> Field {
    let ty = entity_ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(entity_ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager>>()?;
            if let Ok(id) = ctx.args.try_get("id") {
                let id = Uuid::from_str(id.string()?)?;
                let entity_instance = entity_instance_manager.get(id).ok_or(Error::new("Uuid not found"))?;
                if entity_instance.ty != ty {
                    return Err(Error::new(format!("Entity {} is not a {}", id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(entity_instance)])));
            }
            if let Ok(label) = ctx.args.try_get("label") {
                let entity_instance = entity_instance_manager.get_by_label(label.string()?).ok_or(Error::new("Label not found"))?;
                if entity_instance.ty != ty {
                    return Err(Error::new(format!("Entity {} is not a {}", entity_instance.id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(entity_instance)])));
            }
            let instances = get_entity_instances_by_type_filter_by_properties(&ctx, &entity_type, entity_instance_manager);
            return Ok(Some(FieldValue::list(instances.into_iter().map(FieldValue::owned_any))));
        })
    })
    .description(entity_type.description.clone())
    .argument(InputValue::new("id", TypeRef::named(TypeRef::STRING)))
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_entity_type_properties_as_field_arguments(field, entity_type, true, true);
    field
}

pub fn entity_creation_field(entity_ty: &EntityTypeId, entity_type: &EntityType) -> Option<Field> {
    let ty = entity_ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(entity_ty);
    let mut field = Field::new(dy_ty.mutation_field_name("create"), TypeRef::named_nn(dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager>>()?;
            let id = if let Some(id) = ctx.args.get("id") {
                let id = Uuid::from_str(id.string()?)?;
                if entity_instance_manager.has(id) {
                    return Err(Error::new(format!("Uuid {} is already taken", id)));
                }
                id
            } else {
                Uuid::new_v4()
            };
            let properties = create_properties_from_field_arguments(&ctx, &entity_type.properties)?;
            let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
            let reactive_entity = ReactiveEntity::builder().ty(&ty).id(id).properties(properties).build();
            if let Ok(reactive_entity) = entity_instance_manager.register_reactive_instance(reactive_entity) {
                return Ok(Some(FieldValue::owned_any(reactive_entity)));
            }
            Ok(None)
        })
    })
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)));
    field = add_entity_type_properties_as_field_arguments(field, entity_type, false, false);
    Some(field)
}

pub fn entity_mutation_field(entity_type: &EntityType) -> Option<Field> {
    let ty = entity_type.ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn(dy_ty.mutation_type_name()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager>>()?;
            // Multiple ids
            if let Ok(ids) = ctx.args.try_get("ids") {
                let mut entity_instances = Vec::new();
                for id in ids
                    .list()?
                    .iter()
                    .filter_map(|id| id.string().map(str::to_string).ok())
                    .filter_map(|id| Uuid::from_str(&id).ok())
                {
                    if let Some(entity_instance) = entity_instance_manager.get(id) {
                        if entity_instance.ty != ty {
                            return Err(EntityInstanceIsNotOfType(id, ty.clone()).into());
                        }
                        entity_instances.push(entity_instance);
                    }
                }
                let field_value = FieldValue::owned_any(entity_instances);
                return Ok(Some(field_value));
            }
            // Single ids
            if let Ok(id) = ctx.args.try_get("id") {
                let id = Uuid::from_str(id.string()?)?;
                let entity_instance = entity_instance_manager.get(id).ok_or(EntityInstanceNotFound(id.clone()))?;
                if entity_instance.ty != ty {
                    return Err(EntityInstanceIsNotOfType(id, ty.clone()).into());
                }
                let entity_instances = vec![entity_instance];
                let field_value = FieldValue::owned_any(entity_instances);
                return Ok(Some(field_value));
            }
            // TODO: implement label matching
            let instances = get_entity_instances_by_type_filter_by_properties(&ctx, &entity_type, entity_instance_manager);
            let field_value = FieldValue::owned_any(instances);
            Ok(Some(field_value))
        })
    })
    .description(entity_type.description.clone())
    .argument(InputValue::new("ids", TypeRef::named_nn_list(TypeRef::ID)))
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)))
    // TODO: implement label matching
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_entity_type_properties_as_field_arguments(field, entity_type, true, true);
    Some(field)
}

pub fn entity_id_field() -> Field {
    Field::new(INTERFACE_ENTITY_FIELD_ID, TypeRef::named_nn(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(Some(FieldValue::value(ID(entity_instance.id.to_string()))))
        })
    })
}

pub fn entity_property_field(property_type: &PropertyType) -> Field {
    let property_type_inner = property_type.clone();
    Field::new(&property_type.name, to_type_ref(&property_type.data_type), move |ctx| {
        let property_type = property_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(entity_instance.get(&property_type.name).and_then(to_field_value))
        })
    })
    .description(&property_type.description)
}

pub fn entity_outbound_relation_field(
    outbound_relation_type: &RelationType,
    field_names: &DynamicGraphFieldNameExtension,
    field_descriptions: &DynamicGraphFieldDescriptionExtension,
) -> Option<Field> {
    let outbound_ty = outbound_relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&outbound_ty);

    let field_name = field_names.from_outbound_entity_to_relation.clone().unwrap_or(dy_ty.outbound_type_name());
    if field_name.is_empty() {
        return None;
    }
    let field_description = field_descriptions
        .from_outbound_entity_to_relation
        .clone()
        .unwrap_or(outbound_relation_type.description.clone());

    let field = Field::new(field_name, TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let outbound_ty = outbound_ty.clone();
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager>>()?;
            let relation_instances: Vec<FieldValue> = relation_instance_manager
                .get_by_outbound_entity(entity_instance.id)
                .iter()
                .filter(|relation_instance| outbound_ty.clone() == relation_instance.relation_type_id())
                .map(|relation_instance| FieldValue::owned_any(relation_instance.clone()))
                .collect();
            Ok(Some(FieldValue::list(relation_instances)))
        })
    })
    .description(field_description);
    Some(field)
}

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
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager>>()?;
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

pub fn outbound_entity_to_inbound_field(
    outbound_relation_type: &RelationType,
    field_names: &DynamicGraphFieldNameExtension,
    field_descriptions: &DynamicGraphFieldDescriptionExtension,
    context: &SchemaBuilderContext,
) -> Vec<Field> {
    let Some(relation_type) = context.relation_type_manager.get(&outbound_relation_type.ty) else {
        return Vec::new();
    };
    let field_name = field_names.from_outbound_entity_to_inbound_entity.clone();
    // if field_name.is_empty() {
    //     return Vec::new();
    // }
    trace!("from outbound {} to inbound {:?} {}", &outbound_relation_type.ty, field_name, &relation_type.inbound_type);
    let field_description = field_descriptions.from_outbound_entity_to_inbound_entity.clone();

    match &relation_type.inbound_type {
        ComponentOrEntityTypeId::EntityType(ty) => {
            if ty.namespace() == "*" {
                optional_field_to_vec(outbound_entity_to_inbound_entities_union_field(
                    &relation_type.ty,
                    UNION_ALL_ENTITIES,
                    field_name,
                    field_description,
                ))
            } else if ty.type_name() == "*" {
                optional_field_to_vec(outbound_entity_to_inbound_entities_union_field(
                    &relation_type.ty,
                    &namespace_entities_union_type_name(&ty.namespace()),
                    field_name,
                    field_description,
                ))
            } else {
                optional_field_to_vec(outbound_entity_to_inbound_entities_field(&relation_type.ty, ty, field_name, field_description))
            }
        }
        ComponentOrEntityTypeId::Component(ty) => {
            if ty.namespace() == "*" {
                context
                    .component_manager
                    .get_type_ids()
                    .into_iter()
                    .filter_map(|ty| outbound_entity_to_inbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_types_by_namespace(&ty.namespace())
                    .into_iter()
                    .filter_map(|ty| outbound_entity_to_inbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else {
                optional_field_to_vec(outbound_entity_to_inbound_components_field(&relation_type.ty, ty, field_name, field_description))
            }
        }
    }
}

pub fn inbound_entity_to_outbound_field(
    inbound_relation_type: &RelationType,
    field_names: &DynamicGraphFieldNameExtension,
    field_descriptions: &DynamicGraphFieldDescriptionExtension,
    context: &SchemaBuilderContext,
) -> Vec<Field> {
    let Some(relation_type) = context.relation_type_manager.get(&inbound_relation_type.ty) else {
        return Vec::new();
    };
    let field_name = field_names.from_inbound_entity_to_outbound_entity.clone();
    trace!("from inbound {} to outbound {:?} {}", &inbound_relation_type.ty, field_name, &relation_type.outbound_type);
    let field_description = field_descriptions.from_inbound_entity_to_outbound_entity.clone();

    match &relation_type.outbound_type {
        ComponentOrEntityTypeId::EntityType(ty) => {
            if ty.namespace() == "*" {
                optional_field_to_vec(inbound_entity_to_outbound_entities_union_field(
                    &relation_type.ty,
                    UNION_ALL_ENTITIES,
                    field_name,
                    field_description,
                ))
            } else if ty.type_name() == "*" {
                optional_field_to_vec(inbound_entity_to_outbound_entities_union_field(
                    &relation_type.ty,
                    &namespace_entities_union_type_name(&ty.namespace()),
                    field_name,
                    field_description,
                ))
            } else {
                optional_field_to_vec(inbound_entity_to_outbound_entities_field(&relation_type.ty, ty, field_name, field_description))
            }
        }
        ComponentOrEntityTypeId::Component(ty) => {
            if ty.namespace() == "*" {
                context
                    .component_manager
                    .get_type_ids()
                    .into_iter()
                    .filter_map(|ty| inbound_entity_to_outbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_types_by_namespace(&ty.namespace())
                    .into_iter()
                    .filter_map(|ty| inbound_entity_to_outbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else {
                optional_field_to_vec(inbound_entity_to_outbound_components_field(&relation_type.ty, ty, field_name, field_description))
            }
        }
    }
}

///////////////////////////////

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
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager>>()?;
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
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager>>()?;
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

////////////////////////////////

pub fn inbound_entity_to_outbound_entities_field(
    ty: &RelationTypeId,
    outbound_ty: &EntityTypeId,
    field_name: Option<String>,
    field_description: Option<String>,
) -> Option<Field> {
    let dy_ty = DynamicGraphTypeDefinition::from(outbound_ty);
    let field_name = field_name.unwrap_or(dy_ty.outbound_type_name());
    create_inbound_entity_to_outbound_field(ty, &dy_ty.to_string(), &field_name, field_description)
}

pub fn inbound_entity_to_outbound_entities_union_field(
    ty: &RelationTypeId,
    type_name: &str,
    field_name: Option<String>,
    field_description: Option<String>,
) -> Option<Field> {
    let field_name = field_name.unwrap_or("outbound".to_string());
    if field_name.is_empty() {
        return None;
    }
    let relation_ty_inner = ty.clone();
    let mut field = Field::new(field_name, TypeRef::named_nn_list_nn(type_name), move |ctx| {
        let ty = relation_ty_inner.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_inbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| relation_instance.relation_type_id() == ty)
                    .map(|relation_instance| {
                        let outbound = relation_instance.outbound.clone();
                        let dy_ty = DynamicGraphTypeDefinition::from(&outbound.ty);
                        FieldValue::owned_any(outbound).with_type(dy_ty.to_string())
                    }),
            )))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    Some(field)
}

pub fn inbound_entity_to_outbound_components_field(
    ty: &RelationTypeId,
    component_ty: &ComponentTypeId,
    field_name: Option<String>,
    field_description: Option<String>,
) -> Option<Field> {
    let dy_ty = DynamicGraphTypeDefinition::from(component_ty);
    let field_name = field_name.unwrap_or(dy_ty.outbound_type_name());
    create_inbound_entity_to_outbound_field(ty, &dy_ty.to_string(), &field_name, field_description)
}

pub fn create_inbound_entity_to_outbound_field(ty: &RelationTypeId, type_name: &str, field_name: &str, field_description: Option<String>) -> Option<Field> {
    if field_name.is_empty() {
        return None;
    }
    let relation_ty_inner = ty.clone();
    let mut field = Field::new(field_name, TypeRef::named_nn_list_nn(type_name), move |ctx| {
        let ty = relation_ty_inner.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_inbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| relation_instance.relation_type_id() == ty)
                    .map(|relation_instance| FieldValue::owned_any(relation_instance.outbound.clone())),
            )))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    Some(field)
}

fn optional_field_to_vec(field: Option<Field>) -> Vec<Field> {
    match field {
        Some(field) => vec![field],
        None => vec![],
    }
}

fn get_entity_instances_by_type_filter_by_properties(
    ctx: &ResolverContext,
    entity_type: &EntityType,
    entity_instance_manager: &Arc<dyn ReactiveEntityManager>,
) -> Vec<ReactiveEntity> {
    let mut instances = entity_instance_manager.get_by_type(&entity_type.ty);
    for property in entity_type.properties.iter() {
        let Some(expected_value) = ctx.args.get(&property.name) else {
            continue;
        };
        instances.retain(|instance| match instance.get(&property.name) {
            Some(actual_value) => match &property.data_type {
                DataType::Null => false,
                DataType::Bool => expected_value
                    .boolean()
                    .map(|expected_value| actual_value.as_bool().map(|actual_value| expected_value == actual_value).unwrap_or(false))
                    .unwrap_or(false),
                DataType::Number => {
                    if let Ok(expected_value) = expected_value.i64() {
                        actual_value.as_i64().map(|actual_value| expected_value == actual_value).unwrap_or(false)
                    } else if let Ok(expected_value) = expected_value.u64() {
                        actual_value.as_u64().map(|actual_value| expected_value == actual_value).unwrap_or(false)
                    } else if let Ok(expected_value) = expected_value.f64() {
                        actual_value.as_f64().map(|actual_value| expected_value == actual_value).unwrap_or(false)
                    } else {
                        false
                    }
                }
                DataType::String => expected_value
                    .string()
                    .map(|expected_value| actual_value.as_str().map(|actual_value| expected_value == actual_value).unwrap_or(false))
                    .unwrap_or(false),
                DataType::Array => {
                    if let Ok(_l) = expected_value.list() {
                        if let Ok(expected_value) = expected_value.deserialize::<Value>() {
                            if expected_value.is_array() && actual_value.is_array() {
                                expected_value == actual_value
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                DataType::Object => {
                    if let Ok(_o) = expected_value.object() {
                        if let Ok(expected_value) = expected_value.deserialize::<Value>() {
                            if expected_value.is_object() && actual_value.is_object() {
                                expected_value == actual_value
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                DataType::Any => match expected_value.deserialize::<Value>() {
                    Ok(expected_value) => expected_value == actual_value,
                    Err(_) => false,
                },
            },
            None => false,
        });
    }
    instances
}

fn add_entity_type_properties_as_field_arguments(mut field: Field, entity_type: &EntityType, is_optional: bool, exclude_label: bool) -> Field {
    for property in entity_type.properties.iter() {
        if exclude_label && property.name == LABEL.property_name() {
            continue;
        }
        if let Some(type_ref) = to_input_type_ref(property.value(), is_optional) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    field
}
