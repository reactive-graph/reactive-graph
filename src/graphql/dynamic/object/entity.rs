use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::ID;
use log::trace;
use serde_json::json;
use serde_json::Value;

use crate::api::ReactiveEntityInstanceManager;
use crate::graphql::dynamic::data_type_error;
use crate::graphql::dynamic::entity_id_field;
use crate::graphql::dynamic::entity_inbound_relation_field;
use crate::graphql::dynamic::entity_outbound_relation_field;
use crate::graphql::dynamic::entity_property_field;
use crate::graphql::dynamic::field_description::get_dynamic_graph_field_descriptions;
use crate::graphql::dynamic::field_name::get_dynamic_graph_field_names;
use crate::graphql::dynamic::inbound_entity_to_outbound_field;
use crate::graphql::dynamic::instance_component_id_field;
use crate::graphql::dynamic::is_divergent;
use crate::graphql::dynamic::mutability_error;
use crate::graphql::dynamic::number_error;
use crate::graphql::dynamic::outbound_entity_to_inbound_field;
use crate::graphql::dynamic::to_input_type_ref;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::graphql::dynamic::INTERFACE_ENTITY;
use crate::model::Mutability::Immutable;
use crate::model::Mutability::Mutable;
use crate::model::*;

pub fn get_entity_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for entity_type in context.entity_type_manager.get_all() {
        let ty = ComponentOrEntityTypeId::EntityType(entity_type.ty.clone());
        let outbound_types = context.relation_type_manager.get_outbound_relation_types(&ty, false);
        let inbound_types = context.relation_type_manager.get_inbound_relation_types(&ty, false);
        schema = schema.register(get_entity_type(entity_type.clone(), outbound_types, inbound_types, &context));
    }
    schema
}

pub fn get_entity_type(entity_type: EntityType, outbound_types: Vec<RelationType>, inbound_types: Vec<RelationType>, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut object = Object::new(&dy_ty.to_string())
        .description(&entity_type.description)
        .implement(INTERFACE_ENTITY);
    // Components
    for component_ty in entity_type.components.iter() {
        object = object.field(instance_component_id_field(&component_ty));
        let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty);
        if !is_divergent(&entity_type, &component_ty) {
            object = object.implement(component_dy_ty.to_string());
        }
    }
    // ID field
    object = object.field(entity_id_field());
    // Property Fields
    for field in entity_type.properties.iter().map(entity_property_field) {
        object = object.field(field);
    }
    // Outbound Relations
    for outbound_relation_type in outbound_types {
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(&outbound_relation_type);
        let field_descriptions = get_dynamic_graph_field_descriptions(&outbound_relation_type);

        object = object.field(entity_outbound_relation_field(&outbound_relation_type, &field_names, &field_descriptions));
        for field in outbound_entity_to_inbound_field(&outbound_relation_type, &field_names, &field_descriptions, &context) {
            object = object.field(field);
        }
    }
    // Inbound Relations
    for inbound_relation_type in inbound_types {
        // Look up field names and descriptions in extensions
        let field_names = get_dynamic_graph_field_names(&inbound_relation_type);
        let field_descriptions = get_dynamic_graph_field_descriptions(&inbound_relation_type);

        object = object.field(entity_inbound_relation_field(&inbound_relation_type, &field_names, &field_descriptions));
        for field in inbound_entity_to_outbound_field(&inbound_relation_type, &field_names, &field_descriptions, &context) {
            object = object.field(field);
        }
    }
    object
}

pub fn get_entity_mutation_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for entity_type in context.entity_type_manager.get_all() {
        schema = schema.register(get_entity_mutation_type(entity_type));
    }
    schema
}

pub fn get_entity_mutation_type(entity_type: EntityType) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut object = Object::new(&dy_ty.mutation_type_name());
    if let Some(update_field) = get_entity_update_field(&entity_type) {
        object = object.field(update_field);
    }
    object = object.field(get_entity_delete_field());
    object
}

pub fn get_entity_update_field(entity_type: &EntityType) -> Option<Field> {
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut update_field = Field::new("update", TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instances = ctx.parent_value.try_downcast_ref::<Vec<Arc<ReactiveEntityInstance>>>()?;
            for entity_instance in entity_instances {
                // First validate all input fields for mutability and correct datatype
                for property in entity_type.properties.iter() {
                    if let Ok(value) = ctx.args.try_get(&property.name) {
                        // Fail on every property which is immutable
                        if property.mutability == Immutable {
                            return Err(mutability_error(property));
                        }
                        match &property.data_type {
                            DataType::Null => {
                                // Fail on properties with the null datatype
                                return Err(data_type_error(property));
                            }
                            DataType::Bool => {
                                if value.boolean().is_err() {
                                    // Fail if no boolean was set for a boolean property
                                    return Err(data_type_error(property));
                                }
                            }
                            DataType::Number => {
                                if value.f64().is_err() || value.i64().is_err() || value.u64().is_err() {
                                    // Fail if no number was set for a number property
                                    return Err(data_type_error(property));
                                }
                            }
                            DataType::String => {
                                if value.string().is_err() {
                                    // Fail if no string was set for a string property
                                    return Err(data_type_error(property));
                                }
                            }
                            DataType::Array => {
                                if value.list().is_err() {
                                    // Fail if no list was set for a array property
                                    return Err(data_type_error(property));
                                }
                            }
                            DataType::Object => {
                                if value.object().is_err() {
                                    // Fail if no object was set for a object property
                                    return Err(data_type_error(property));
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
                                return Err(data_type_error(property));
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
                                    return Err(number_error(property));
                                }
                            }
                            DataType::String => {
                                entity_instance.set_checked(&property.name, Value::String(value.string()?.to_string()));
                            }
                            DataType::Array => {
                                let _list = value.list()?;
                                let value = value.deserialize::<Value>()?;
                                if !value.is_array() {
                                    return Err(data_type_error(property));
                                }
                                entity_instance.set_checked(&property.name, value);
                            }
                            DataType::Object => {
                                let value = value.deserialize::<Value>()?;
                                if !value.is_object() {
                                    return Err(data_type_error(property));
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
                entity_instances
                    .into_iter()
                    .map(|entity_instance| FieldValue::owned_any(entity_instance.clone())),
            )))
        })
    });
    let mut has_updatable_property = false;
    for property in entity_type.properties.iter() {
        if property.mutability == Mutable {
            if let Some(type_ref) = to_input_type_ref(&property, true) {
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

pub fn get_entity_delete_field() -> Field {
    Field::new("delete", TypeRef::named_nn_list_nn(TypeRef::ID), move |ctx| {
        FieldFuture::new(async move {
            let entity_instance_manager = ctx.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
            let mut ids = Vec::new();
            for entity_instance in ctx.parent_value.try_downcast_ref::<Vec<Arc<ReactiveEntityInstance>>>()? {
                trace!("Deleting entity instance {}", entity_instance);
                let id = entity_instance.id;
                entity_instance_manager.delete(id);
                ids.push(id);
            }
            Ok(Some(FieldValue::list(ids.iter().map(|id| FieldValue::value(ID(id.to_string()))))))
        })
    })
}
