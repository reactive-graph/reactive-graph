use std::sync::Arc;

use async_graphql::ID;
use async_graphql::dynamic::*;
use log::trace;
use reactive_graph_dynamic_graph_api::ImmutablePropertyError;
use reactive_graph_dynamic_graph_api::PropertyDataTypeError;
use serde_json::Value;
use serde_json::json;

use crate::DynamicGraphTypeDefinition;
use crate::INTERFACE_ENTITY;
use crate::entity_id_field;
use crate::entity_inbound_relation_field;
use crate::entity_outbound_relation_field;
use crate::entity_property_field;
use crate::field_description::get_dynamic_graph_field_descriptions;
use crate::field_name::get_dynamic_graph_field_names;
use crate::inbound_entity_to_outbound_field;
use crate::instance_component_id_field;
use crate::is_divergent;
use crate::outbound_entity_to_inbound_field;
use crate::to_input_type_ref;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::DataType;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Mutability::Immutable;
use reactive_graph_graph::Mutability::Mutable;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_graph::RelationTypes;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_runtime_model::COMPONENT_ACTION;

pub fn get_entity_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for entity_type in context.entity_type_manager.get_all().iter() {
        let ty = ComponentOrEntityTypeId::EntityType(entity_type.key().clone());
        let outbound_types = context.relation_type_manager.get_outbound_relation_types(&ty, false);
        let inbound_types = context.relation_type_manager.get_inbound_relation_types(&ty, false);
        let entity_type = get_entity_type(entity_type.value(), outbound_types, inbound_types, context);
        schema = schema.register(entity_type);
    }
    schema
}

pub fn get_entity_type(entity_type: &EntityType, outbound_types: RelationTypes, inbound_types: RelationTypes, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut object = Object::new(dy_ty.to_string()).description(&entity_type.description).implement(INTERFACE_ENTITY);
    // Components
    for component_ty in entity_type.components.iter() {
        object = object.field(instance_component_id_field(component_ty.key()));
        let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty.key());
        if !is_divergent(entity_type, component_ty.key()) {
            object = object.implement(component_dy_ty.to_string());
        }
    }
    // ID field
    object = object.field(entity_id_field());
    // Property Fields
    for property_type in entity_type.properties.iter() {
        object = object.field(entity_property_field(&property_type));
    }
    // Outbound Relations
    for outbound_relation_type in outbound_types.iter() {
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(outbound_relation_type.value());
        let field_descriptions = get_dynamic_graph_field_descriptions(outbound_relation_type.value());

        if let Some(entity_outbound_relation_field) = entity_outbound_relation_field(outbound_relation_type.value(), &field_names, &field_descriptions) {
            object = object.field(entity_outbound_relation_field);
        }
        for field in outbound_entity_to_inbound_field(&outbound_relation_type, &field_names, &field_descriptions, context) {
            object = object.field(field);
        }
    }
    // Inbound Relations
    for inbound_relation_type in inbound_types.iter() {
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(inbound_relation_type.value());
        let field_descriptions = get_dynamic_graph_field_descriptions(inbound_relation_type.value());

        if let Some(entity_inbound_relation_field) = entity_inbound_relation_field(&inbound_relation_type, &field_names, &field_descriptions) {
            object = object.field(entity_inbound_relation_field);
        }
        for field in inbound_entity_to_outbound_field(&inbound_relation_type, &field_names, &field_descriptions, context) {
            object = object.field(field);
        }
    }
    object
}

pub fn get_entity_mutation_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for (entity_ty, entity_type) in context.entity_type_manager.get_all() {
        schema = schema.register(get_entity_mutation_type(&entity_ty, &entity_type));
    }
    schema
}

pub fn get_entity_mutation_type(entity_ty: &EntityTypeId, entity_type: &EntityType) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(entity_ty);
    let mut object = Object::new(dy_ty.mutation_type_name());
    if let Some(update_field) = get_entity_update_field(entity_type) {
        object = object.field(update_field);
    }
    if let Some(trigger_field) = get_entity_type_trigger_field(entity_type) {
        object = object.field(trigger_field);
    }
    object = object.field(get_entity_delete_field());
    object
}

pub fn get_entity_update_field(entity_type: &EntityType) -> Option<Field> {
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut update_field = Field::new("update", TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveEntity>>()?;
            for entity_instance in entity_instances {
                // First validate all input fields for mutability and correct datatype
                for property in entity_type.properties.iter() {
                    if let Ok(value) = ctx.args.try_get(&property.name) {
                        // Fail on every property which is immutable
                        if property.mutability == Immutable {
                            return Err(ImmutablePropertyError(property.key().clone()).into());
                        }
                        match &property.data_type {
                            DataType::Null => {
                                return Err(PropertyDataTypeError::NullIsNotAValidDataType(property.key().clone()).into());
                            }
                            DataType::Bool => {
                                if value.boolean().is_err() {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::Bool,
                                    )
                                    .into());
                                }
                            }
                            DataType::Number => {
                                if value.f64().is_err() && value.i64().is_err() && value.u64().is_err() {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::Number,
                                    )
                                    .into());
                                }
                            }
                            DataType::String => {
                                if value.string().is_err() {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::String,
                                    )
                                    .into());
                                }
                            }
                            DataType::Array => {
                                if value.list().is_err() {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::Array,
                                    )
                                    .into());
                                }
                            }
                            DataType::Object => {
                                if value.object().is_err() {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::Object,
                                    )
                                    .into());
                                }
                            }
                            DataType::Any => {
                                // Accept input of any datatype
                            }
                        }
                    }
                }
                // Set properties
                for property in entity_type.properties.iter() {
                    if let Ok(value) = ctx.args.try_get(&property.name) {
                        match &property.data_type {
                            DataType::Null => {
                                return Err(PropertyDataTypeError::NullIsNotAValidDataType(property.key().clone()).into());
                            }
                            DataType::Bool => {
                                entity_instance.set_checked(&property.name, Value::Bool(value.boolean()?));
                            }
                            DataType::Number => {
                                if let Ok(value) = value.i64() {
                                    entity_instance.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.u64() {
                                    entity_instance.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.f64() {
                                    entity_instance.set_checked(&property.name, json!(value));
                                } else {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::Number,
                                    )
                                    .into());
                                }
                            }
                            DataType::String => {
                                entity_instance.set_checked(&property.name, Value::String(value.string()?.to_string()));
                            }
                            DataType::Array => {
                                let _list = value.list()?;
                                let value = value.deserialize::<Value>()?;
                                if !value.is_array() {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::Array,
                                    )
                                    .into());
                                }
                                entity_instance.set_checked(&property.name, value);
                            }
                            DataType::Object => {
                                let value = value.deserialize::<Value>()?;
                                if !value.is_object() {
                                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(
                                        property.name.clone(),
                                        property.data_type,
                                        DataType::Object,
                                    )
                                    .into());
                                }
                                entity_instance.set_checked(&property.name, value);
                            }
                            DataType::Any => {
                                // If it's possible to deserialize, accept the input
                                let value = value.deserialize::<Value>()?;
                                entity_instance.set_checked(&property.name, value);
                            }
                        }
                    }
                }
            }
            Ok(Some(FieldValue::list(
                entity_instances.iter().map(|entity_instance| FieldValue::owned_any(entity_instance.clone())),
            )))
        })
    });
    let mut has_updatable_property = false;
    for property in entity_type.properties.iter() {
        if property.mutability == Mutable {
            if let Some(type_ref) = to_input_type_ref(property.value(), true) {
                update_field = update_field.argument(InputValue::new(&property.name, type_ref));
                has_updatable_property = true;
            }
        }
    }
    if !has_updatable_property {
        return None;
    }
    Some(update_field)
}

pub fn get_entity_type_trigger_field(entity_type: &EntityType) -> Option<Field> {
    if !entity_type.is_a(&COMPONENT_ACTION) {
        return None;
    }
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let trigger_field = Field::new(TRIGGER.property_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        FieldFuture::new(async move {
            let entity_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveEntity>>()?;
            for entity_instance in entity_instances {
                entity_instance.set(TRIGGER.property_name(), json!(true));
            }
            Ok(Some(FieldValue::list(
                entity_instances.iter().map(|entity_instance| FieldValue::owned_any(entity_instance.clone())),
            )))
        })
    });
    Some(trigger_field)
}

pub fn get_entity_delete_field() -> Field {
    Field::new("delete", TypeRef::named_nn_list_nn(TypeRef::ID), move |ctx| {
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
            let mut ids = Vec::new();
            for entity_instance in ctx.parent_value.try_downcast_ref::<Vec<ReactiveEntity>>()? {
                trace!("Deleting entity instance {}", entity_instance);
                let id = entity_instance.id;
                entity_instance_manager.delete(id);
                ids.push(id);
            }
            Ok(Some(FieldValue::list(ids.iter().map(|id| FieldValue::value(ID(id.to_string()))))))
        })
    })
}
