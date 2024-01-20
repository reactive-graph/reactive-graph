use std::str::FromStr;
use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::Error;
use async_graphql::ID;
use inexor_rgf_reactive_service_api::ReactiveRelationRegistrationError;
use serde_json::Value;
use uuid::Uuid;

use crate::create_properties_from_field_arguments;
use crate::namespace_entities_union_type_name;
use crate::to_field_value;
use crate::to_input_type_ref;
use crate::to_type_ref;
use crate::DynamicGraphTypeDefinition;
use crate::INTERFACE_RELATION_FIELD_INSTANCE_ID;
use crate::INTERFACE_RELATION_FIELD_KEY;
use crate::UNION_ALL_ENTITIES;
use inexor_rgf_dynamic_graph_api::SchemaBuilderContext;
use inexor_rgf_graph::ComponentOrEntityTypeId;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::DataType;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::NamespacedTypeGetter;
use inexor_rgf_graph::PropertyInstanceGetter;
use inexor_rgf_graph::PropertyType;
use inexor_rgf_graph::PropertyTypeDefinition;
use inexor_rgf_graph::RelationInstanceId;
use inexor_rgf_graph::RelationInstanceTypeId;
use inexor_rgf_graph::RelationType;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_reactive_model_impl::ReactiveProperties;
use inexor_rgf_reactive_model_impl::ReactiveRelation;
use inexor_rgf_reactive_service_api::ReactiveEntityManager;
use inexor_rgf_reactive_service_api::ReactiveRelationCreationError;
use inexor_rgf_reactive_service_api::ReactiveRelationManager;
use inexor_rgf_runtime_model::LabeledProperties::LABEL;

pub fn relation_query_field(relation_ty: &RelationTypeId, relation_type: &RelationType) -> Field {
    let ty = relation_ty.clone();
    let relation_type_inner = relation_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(relation_ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let relation_type = relation_type_inner.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
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
            return Ok(Some(FieldValue::list(instances.into_iter().map(FieldValue::owned_any))));
        })
    })
    .description(relation_type.description.clone());
    for property in relation_type.properties.iter() {
        if property.name == LABEL.property_name() {
            continue;
        }
        if let Some(type_ref) = to_input_type_ref(property.value(), true) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    field
}

pub fn relation_creation_field(relation_ty: &RelationTypeId, relation_type: &RelationType) -> Option<Field> {
    let ty = relation_ty.clone();
    let relation_type_inner = relation_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(relation_ty);
    let mut field = Field::new(dy_ty.mutation_field_name("create"), TypeRef::named_nn(&dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let relation_type = relation_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;

            let outbound_id = Uuid::from_str(ctx.args.try_get("outboundId")?.string()?)?;
            let inbound_id = Uuid::from_str(ctx.args.try_get("inboundId")?.string()?)?;
            let rty = match ctx.args.get("instanceId").and_then(|s| s.string().map(|s| s.to_string()).ok()) {
                Some(instance_id) => RelationInstanceTypeId::new_unique_for_instance_id(ty, instance_id),
                None => RelationInstanceTypeId::new_with_random_instance_id(ty),
            };
            let id = RelationInstanceId::builder().outbound_id(outbound_id).ty(&rty).inbound_id(inbound_id).build();

            if relation_instance_manager.has(&id) {
                return Err(ReactiveRelationRegistrationError::RelationInstanceAlreadyExists(id.clone()).into());
            }

            let outbound = entity_instance_manager
                .get(outbound_id)
                .ok_or::<Error>(ReactiveRelationCreationError::MissingOutboundEntityInstance(outbound_id).into())?;

            let inbound = entity_instance_manager
                .get(inbound_id)
                .ok_or::<Error>(ReactiveRelationCreationError::MissingInboundEntityInstance(inbound_id).into())?;

            let properties = create_properties_from_field_arguments(&ctx, &relation_type.properties)?;
            let properties = ReactiveProperties::new_with_id_from_properties(id, properties);
            let reactive_relation = ReactiveRelation::builder()
                .outbound(outbound)
                .ty(&rty)
                .inbound(inbound)
                .properties(properties)
                .build();
            if let Ok(reactive_relation) = relation_instance_manager.register_reactive_instance(reactive_relation) {
                return Ok(Some(FieldValue::owned_any(reactive_relation)));
            }
            Ok(None)
        })
    })
    .argument(InputValue::new("outboundId", TypeRef::named(TypeRef::ID)))
    .argument(InputValue::new("instanceId", TypeRef::named(TypeRef::ID)))
    .argument(InputValue::new("inboundId", TypeRef::named(TypeRef::ID)));
    for property in relation_type.properties.iter() {
        if let Some(type_ref) = to_input_type_ref(property.value(), false) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    Some(field)
}

pub fn relation_mutation_field(relation_ty: &RelationTypeId, relation_type: &RelationType) -> Option<Field> {
    let ty = relation_ty.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(relation_ty);
    let field = Field::new(dy_ty.field_name(), TypeRef::named_nn(dy_ty.mutation_type_name()), move |ctx| {
        let ty = ty.clone();
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
            let relation_instances: Vec<ReactiveRelation> = relation_instance_manager
                .get_by_type(&ty)
                .into_iter()
                .filter(|relation_instance| {
                    let Ok(id) = ctx.args.try_get("outboundId").and_then(|id| id.string().map(|s| s.to_string())) else {
                        return true;
                    };
                    let Ok(id) = Uuid::from_str(&id) else {
                        return true;
                    };
                    relation_instance.outbound.id == id
                })
                .filter(|relation_instance| {
                    let Ok(id) = ctx.args.try_get("inboundId").and_then(|id| id.string().map(|s| s.to_string())) else {
                        return true;
                    };
                    let Ok(id) = Uuid::from_str(&id) else {
                        return true;
                    };
                    relation_instance.outbound.id == id
                })
                // TODO: implement outbound_type search
                // TODO: implement inbound_type search
                // TODO: implement label matching
                // TODO: implement property search
                .collect();
            let field_value = FieldValue::owned_any(relation_instances);
            Ok(Some(field_value))
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
            let relation_instance = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
            Ok(Some(FieldValue::value(ID(format!("{}", relation_instance.ty)))))
        })
    })
}

pub fn relation_instance_id_field() -> Field {
    Field::new(INTERFACE_RELATION_FIELD_INSTANCE_ID, TypeRef::named_nn(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
            Ok(Some(FieldValue::value(ID(relation_instance.instance_id()))))
        })
    })
}

pub fn relation_property_field(property_type: &PropertyType) -> Field {
    let property_type_inner = property_type.clone();
    Field::new(&property_type.name, to_type_ref(&property_type.data_type), move |ctx| {
        let property_type = property_type_inner.clone();
        FieldFuture::new(async move {
            let relation_instance = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
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
                    .get_type_ids()
                    .iter()
                    .map(|ty| relation_outbound_component_field(ty.key(), None, None))
                    .collect()
                // .get_all()
                // .into_iter()
                // // .map(|component| component.ty)
                // .map(|(component_ty, component)| relation_outbound_component_field(&component_ty, None, None))
                // .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_types_by_namespace(&ty.namespace())
                    .iter()
                    .map(|ty| relation_outbound_component_field(ty.key(), None, None))
                    .collect()
                // .get_by_namespace(&ty.namespace())
                // .into_iter()
                // // .map(|component| component.ty)
                // .map(|(component_ty, component)| relation_outbound_component_field(&component_ty, None, None))
                // .collect()
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
                    .get_type_ids()
                    .iter()
                    .map(|ty| relation_inbound_component_field(ty.key(), None, None))
                    .collect()
                // .get_all()
                // .into_iter()
                // .map(|(component_ty, component)| relation_inbound_component_field(&component_ty, None, None))
                // .collect()
            } else if ty.type_name() == "*" {
                context
                    .component_manager
                    .get_types_by_namespace(&ty.namespace())
                    .iter()
                    .map(|ty| relation_inbound_component_field(ty.key(), None, None))
                    .collect()
                // .get_by_namespace(&ty.namespace())
                // .into_iter()
                // .map(|(component_ty, component)| relation_inbound_component_field(&component_ty, None, None))
                // .collect()
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
            let relation_instance = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
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
            let relation_instance = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
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
            let relation_instance = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
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
            let relation_instance = ctx.parent_value.try_downcast_ref::<ReactiveRelation>()?;
            Ok(Some(FieldValue::owned_any(relation_instance.inbound.clone())))
        })
    });
    if let Some(field_description) = field_description {
        field = field.description(field_description);
    }
    field
}
