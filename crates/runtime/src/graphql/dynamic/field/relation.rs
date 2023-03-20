use std::str::FromStr;
use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::ID;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::builder::ReactiveRelationInstanceBuilder;
use crate::graphql::dynamic::data_type_error;
use crate::graphql::dynamic::entity_instance_not_found_error;
use crate::graphql::dynamic::namespace_entities_union_type_name;
use crate::graphql::dynamic::number_error;
use crate::graphql::dynamic::to_field_value;
use crate::graphql::dynamic::to_input_type_ref;
use crate::graphql::dynamic::to_type_ref;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::graphql::dynamic::INTERFACE_RELATION_FIELD_INSTANCE_ID;
use crate::graphql::dynamic::INTERFACE_RELATION_FIELD_KEY;
use crate::graphql::dynamic::UNION_ALL_ENTITIES;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::DataType;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyType;
use crate::model::PropertyTypeDefinition;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstanceTypeId;
use crate::model::RelationType;
use crate::model_runtime::LabeledProperties::LABEL;

pub fn relation_query_field(relation_type: &RelationType) -> Field {
    let ty = relation_type.ty.clone();
    let relation_type_inner = relation_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let relation_type = relation_type_inner.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let mut instances = relation_instance_manager.get_by_type(&ty);
            for property in relation_type.properties.iter() {
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
            return Ok(Some(FieldValue::list(
                instances.into_iter().map(|relation_instance| FieldValue::owned_any(relation_instance.clone())),
            )));
        })
    })
    .description(relation_type.description.clone());
    for property in relation_type.properties.iter() {
        if property.name == LABEL.property_name() {
            continue;
        }
        if let Some(type_ref) = to_input_type_ref(property, true) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    field
}

pub fn relation_creation_field(relation_type: &RelationType) -> Option<Field> {
    let ty = relation_type.ty.clone();
    let relation_type_inner = relation_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let mut field = Field::new(dy_ty.mutation_field_name("create"), TypeRef::named_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let relation_type = relation_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let outbound_id = Uuid::from_str(ctx.args.try_get("outboundId")?.string()?)?;
            let outbound = entity_instance_manager.get(outbound_id).ok_or(entity_instance_not_found_error(&outbound_id))?;
            let inbound_id = Uuid::from_str(ctx.args.try_get("inboundId")?.string()?)?;
            let inbound = entity_instance_manager.get(inbound_id).ok_or(entity_instance_not_found_error(&inbound_id))?;
            let rty = match ctx.args.get("instanceId").and_then(|s| s.string().map(|s| s.to_string()).ok()) {
                Some(instance_id) => RelationInstanceTypeId::new_unique_for_instance_id(ty, instance_id),
                None => RelationInstanceTypeId::new_with_random_instance_id(ty),
            };
            // if let Some(instance_id) = ctx.args.get("instanceId").and_then(|s| s.string().map(|s| s.to_string()).ok()) {
            //     let x = RelationInstanceTypeId::new_unique_for_instance_id(ty, instance_id);
            // } else {
            //     RelationInstanceTypeId::new_with_random_instance_id(ty);
            // }
            // builder = builder.id(id);

            let mut builder = ReactiveRelationInstanceBuilder::new(outbound, &rty, inbound);
            let mut builder = builder.set_properties_defaults(relation_type.clone());
            // builder.set_properties_defaults(relation_type.clone());
            // let mut builder = builder.;
            for property in relation_type.properties.iter() {
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
            let relation_instance = builder.build();
            if let Ok(relation_instance) = relation_instance_manager.register_reactive_instance(relation_instance) {
                return Ok(Some(FieldValue::owned_any(relation_instance.clone())));
            }
            return Ok(None);
        })
    })
    .argument(InputValue::new("outboundId", TypeRef::named(TypeRef::ID)))
    .argument(InputValue::new("instanceId", TypeRef::named(TypeRef::ID)))
    .argument(InputValue::new("inboundId", TypeRef::named(TypeRef::ID)));
    for property in relation_type.properties.iter() {
        if let Some(type_ref) = to_input_type_ref(property, false) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    Some(field)
}

pub fn relation_mutation_field(relation_type: &RelationType) -> Option<Field> {
    let ty = relation_type.ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let field = Field::new(dy_ty.field_name(), TypeRef::named_nn(&dy_ty.mutation_type_name()), move |ctx| {
        let ty = ty.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            let relation_instances: Vec<Arc<ReactiveRelationInstance>> = relation_instance_manager
                .get_by_type(&ty)
                .into_iter()
                .filter(|relation_instance| {
                    let Ok(id) = ctx.args.try_get("outboundId").and_then(|id| id.string().map(|s|s.to_string())) else {
                        return true;
                    };
                    let Ok(id) = Uuid::from_str(&id) else {
                        return true;
                    };
                    return relation_instance.outbound.id == id;
                })
                .filter(|relation_instance| {
                    let Ok(id) = ctx.args.try_get("inboundId").and_then(|id| id.string().map(|s|s.to_string())) else {
                        return true;
                    };
                    let Ok(id) = Uuid::from_str(&id) else {
                        return true;
                    };
                    return relation_instance.outbound.id == id;
                })
                // TODO: implement outbound_type search
                // TODO: implement inbound_type search
                // TODO: implement label matching
                // TODO: implement property search
                .collect();
            let field_value = FieldValue::owned_any(relation_instances);
            return Ok(Some(field_value));
        })
    })
    // .argument(InputValue::new("ids", TypeRef::named_nn_list(TypeRef::ID)))
    .argument(InputValue::new("outboundId", TypeRef::named(TypeRef::ID)))
    .argument(InputValue::new("inboundId", TypeRef::named(TypeRef::ID)))
    // TODO: implement label matching
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)))
    // TODO: implement property search
    // for property in properties {
    //   .argument(InputValue::new(property.name, TypeRef::named_nn_list(type_ref_properties(property))))
    .description(relation_type.description.clone());
    Some(field)
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
                vec![relation_outbound_entity_union_field(UNION_ALL_ENTITIES, field_name, field_description)]
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
                vec![relation_inbound_entity_union_field(UNION_ALL_ENTITIES, field_name, field_description)]
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
