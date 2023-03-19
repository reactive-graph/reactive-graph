use std::str::FromStr;
use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::Error;
use async_graphql::ID;
use log::trace;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::builder::ReactiveEntityInstanceBuilder;
use crate::graphql::dynamic::data_type_error;
use crate::graphql::dynamic::entity_instance_not_found_error;
use crate::graphql::dynamic::entity_instance_not_of_entity_type_error;
use crate::graphql::dynamic::field_description::DynamicGraphFieldDescriptionExtension;
use crate::graphql::dynamic::field_name::DynamicGraphFieldNameExtension;
use crate::graphql::dynamic::interface::entity::INTERFACE_ENTITY_FIELD_ID;
use crate::graphql::dynamic::namespace_entities_union_type_name;
use crate::graphql::dynamic::number_error;
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
use crate::model::ReactiveEntityInstance;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::model_runtime::LabeledProperties::LABEL;

pub fn entity_query_field(entity_type: &EntityType) -> Field {
    let ty = entity_type.ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
            if let Ok(id) = ctx.args.try_get("id") {
                let id = Uuid::from_str(id.string()?)?;
                let entity_instance = entity_instance_manager.get(id).ok_or(Error::new("Uuid not found"))?;
                if &entity_instance.ty != &ty {
                    return Err(Error::new(format!("Entity {} is not a {}", id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(entity_instance.clone())])));
            }
            if let Ok(label) = ctx.args.try_get("label") {
                let entity_instance = entity_instance_manager.get_by_label(label.string()?).ok_or(Error::new("Label not found"))?;
                if &entity_instance.ty != &ty {
                    return Err(Error::new(format!("Entity {} is not a {}", entity_instance.id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(entity_instance.clone())])));
            }
            let instances = get_entity_instances_by_type_filter_by_properties(&ctx, &entity_type, &entity_instance_manager);
            return Ok(Some(FieldValue::list(
                instances.into_iter().map(|entity_instance| FieldValue::owned_any(entity_instance.clone())),
            )));
        })
    })
    .description(entity_type.description.clone())
    .argument(InputValue::new("id", TypeRef::named(TypeRef::STRING)))
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_entity_type_properties_as_field_arguments(field, &entity_type, true, true);
    field
}

pub fn entity_creation_field(entity_type: &EntityType) -> Option<Field> {
    let ty = entity_type.ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut field = Field::new(dy_ty.mutation_field_name("create"), TypeRef::named_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
            let mut builder = ReactiveEntityInstanceBuilder::new(&ty);
            let mut builder = builder.id(Uuid::new_v4());
            builder = builder.set_properties_defaults(entity_type.clone());
            for property in entity_type.properties.iter() {
                let value = ctx.args.try_get(&property.name)?;
                match &property.data_type {
                    DataType::Null => {
                        return Err(data_type_error(property));
                    }
                    DataType::Bool => {
                        builder = builder.property(&property.name, Value::Bool(value.boolean()?));
                    }
                    DataType::Number => {
                        if let Ok(value) = value.i64() {
                            builder = builder.property(&property.name, json!(value));
                        } else if let Ok(value) = value.u64() {
                            builder = builder.property(&property.name, json!(value));
                        } else if let Ok(value) = value.f64() {
                            builder = builder.property(&property.name, json!(value));
                        } else {
                            return Err(number_error(property));
                        }
                    }
                    DataType::String => {
                        builder = builder.property(&property.name, Value::String(value.string()?.to_string()));
                    }
                    DataType::Array => {
                        let _ = value.list()?;
                        let value = value.deserialize::<Value>()?;
                        if !value.is_array() {
                            return Err(data_type_error(property));
                        }
                        builder = builder.property(&property.name, value);
                    }
                    DataType::Object => {
                        let value = value.deserialize::<Value>()?;
                        if !value.is_object() {
                            return Err(data_type_error(property));
                        }
                        builder = builder.property(&property.name, value);
                    }
                    DataType::Any => {
                        if let Ok(value) = value.deserialize() {
                            builder = builder.property(&property.name, value);
                        }
                    }
                }
            }
            if let Some(id) = ctx.args.get("id") {
                let id = Uuid::from_str(id.string()?)?;
                if entity_instance_manager.has(id) {
                    return Err(Error::new(format!("Uuid {} is already taken", id)));
                }
                builder = builder.id(id);
            }
            let entity_instance = builder.build();
            if let Ok(entity_instance) = entity_instance_manager.register_reactive_instance(entity_instance) {
                return Ok(Some(FieldValue::owned_any(entity_instance.clone())));
            }
            return Ok(None);
        })
    })
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)));
    field = add_entity_type_properties_as_field_arguments(field, &entity_type, false, false);
    Some(field)
}

pub fn entity_mutation_field(entity_type: &EntityType) -> Option<Field> {
    let ty = entity_type.ty.clone();
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn(&dy_ty.mutation_type_name()), move |ctx| {
        let ty = ty.clone();
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
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
                        if &entity_instance.ty != &ty {
                            return Err(entity_instance_not_of_entity_type_error(&id, &ty));
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
                let entity_instance = entity_instance_manager.get(id).ok_or(entity_instance_not_found_error(&id))?;
                if &entity_instance.ty != &ty {
                    return Err(entity_instance_not_of_entity_type_error(&id, &ty));
                }
                let entity_instances = vec![entity_instance.clone()];
                let field_value = FieldValue::owned_any(entity_instances);
                return Ok(Some(field_value));
            }
            // TODO: implement label matching
            let instances = get_entity_instances_by_type_filter_by_properties(&ctx, &entity_type, &entity_instance_manager);
            let field_value = FieldValue::owned_any(instances);
            return Ok(Some(field_value));
        })
    })
    .description(entity_type.description.clone())
    .argument(InputValue::new("ids", TypeRef::named_nn_list(TypeRef::ID)))
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)))
    // TODO: implement label matching
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_entity_type_properties_as_field_arguments(field, &entity_type, true, true);
    Some(field)
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

pub fn entity_outbound_relation_field(
    outbound_relation_type: &RelationType,
    field_names: &DynamicGraphFieldNameExtension,
    field_descriptions: &DynamicGraphFieldDescriptionExtension,
) -> Option<Field> {
    let outbound_ty = outbound_relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&outbound_ty);
    let outbound_ty_inner = outbound_ty.clone();

    let field_name = field_names.from_outbound_entity_to_relation.clone().unwrap_or(dy_ty.outbound_type_name());
    if field_name.is_empty() {
        return None;
    }
    let field_description = field_descriptions
        .from_outbound_entity_to_relation
        .clone()
        .unwrap_or(outbound_relation_type.description.clone());

    let field = Field::new(field_name, TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
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
    .description(&field_description);
    Some(field)
}

pub fn entity_inbound_relation_field(
    inbound_relation_type: &RelationType,
    field_names: &DynamicGraphFieldNameExtension,
    field_descriptions: &DynamicGraphFieldDescriptionExtension,
) -> Option<Field> {
    let inbound_ty = inbound_relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&inbound_ty);
    let inbound_ty_inner = inbound_ty.clone();

    let field_name = field_names.from_inbound_entity_to_relation.clone().unwrap_or(dy_ty.inbound_type_name());
    if field_name.is_empty() {
        return None;
    }
    let field_description = field_descriptions
        .from_inbound_entity_to_relation
        .clone()
        .unwrap_or(inbound_relation_type.description.clone());

    let field = Field::new(field_name, TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
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
    .description(&field_description);
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
                    &UNION_ALL_ENTITIES,
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
                    .get_all()
                    .into_iter()
                    .map(|component| component.ty)
                    .filter_map(|ty| outbound_entity_to_inbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_by_namespace(&ty.namespace())
                    .into_iter()
                    .map(|component| component.ty)
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
                    &UNION_ALL_ENTITIES,
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
                    .get_all()
                    .into_iter()
                    .map(|component| component.ty)
                    .filter_map(|ty| inbound_entity_to_outbound_components_field(&relation_type.ty, &ty, None, None))
                    .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_by_namespace(&ty.namespace())
                    .into_iter()
                    .map(|component| component.ty)
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
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_outbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| &relation_instance.relation_type_id() == &ty)
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
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_outbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| &relation_instance.relation_type_id() == &ty)
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
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_inbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| &relation_instance.relation_type_id() == &ty)
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
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let entity_instance = ctx.parent_value.try_downcast_ref::<Arc<ReactiveEntityInstance>>()?;
            Ok(Some(FieldValue::list(
                relation_instance_manager
                    .get_by_inbound_entity(entity_instance.id)
                    .iter()
                    .filter(|relation_instance| &relation_instance.relation_type_id() == &ty)
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
    entity_instance_manager: &Arc<dyn ReactiveEntityInstanceManager>,
) -> Vec<Arc<ReactiveEntityInstance>> {
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
        if exclude_label && &property.name == &LABEL.property_name() {
            continue;
        }
        if let Some(type_ref) = to_input_type_ref(&property, is_optional) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    field
}
