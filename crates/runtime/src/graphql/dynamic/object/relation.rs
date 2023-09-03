use std::sync::Arc;

use async_graphql::dynamic::*;
use async_graphql::ID;
use log::trace;
use serde_json::json;
use serde_json::Value;
use inexor_rgf_reactive::{ReactiveInstance, ReactiveRelation};

use crate::api::ReactiveRelationManager;
use crate::graphql::dynamic::data_type_error;
use crate::graphql::dynamic::field_description::get_dynamic_graph_field_descriptions;
use crate::graphql::dynamic::field_name::get_dynamic_graph_field_names;
use crate::graphql::dynamic::instance_component_id_field;
use crate::graphql::dynamic::is_divergent;
use crate::graphql::dynamic::mutability_error;
use crate::graphql::dynamic::number_error;
use crate::graphql::dynamic::relation_inbound_field;
use crate::graphql::dynamic::relation_instance_id_field;
use crate::graphql::dynamic::relation_key_field;
use crate::graphql::dynamic::relation_outbound_field;
use crate::graphql::dynamic::relation_property_field;
use crate::graphql::dynamic::to_input_type_ref;
use crate::graphql::dynamic::DynamicGraphTypeDefinition;
use crate::graphql::dynamic::SchemaBuilderContext;
use crate::graphql::dynamic::INTERFACE_RELATION;
use crate::model::Mutability::Immutable;
use crate::model::Mutability::Mutable;
use crate::model::*;
use crate::model_runtime::ActionProperties::TRIGGER;
use crate::model_runtime::COMPONENT_ACTION;

pub fn get_relation_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for relation_type in context.relation_type_manager.get_all().iter() {
        schema = schema.register(get_relation_type(relation_type.key(), relation_type.value(), context));
    }
    schema
}

pub fn get_relation_type(relation_ty: &RelationTypeId, relation_type: &RelationType, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(relation_ty);
    let mut object = Object::new(&dy_ty.to_string())
        .description(&relation_type.description)
        .implement(INTERFACE_RELATION);
    // Components
    for component_ty in relation_type.components.iter() {
        object = object.field(instance_component_id_field(&component_ty));
        let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty.key());
        if !is_divergent(relation_type, component_ty.key()) {
            object = object.implement(component_dy_ty.to_string());
        }
    }
    // Edge key field
    object = object.field(relation_key_field());
    object = object.field(relation_instance_id_field());
    for property_type in relation_type.properties.iter() {
        object = object.field(relation_property_field(property_type.value()));
    }
    // Look up field names and descriptions in extensions
    let field_names = get_dynamic_graph_field_names(relation_type);
    let field_descriptions = get_dynamic_graph_field_descriptions(relation_type);
    // Outbound fields
    for field in relation_outbound_field(
        &relation_type.outbound_type,
        field_names.from_relation_to_outbound_entity,
        field_descriptions.from_relation_to_outbound_entity,
        context,
    ) {
        object = object.field(field);
    }
    // Inbound fields
    for field in relation_inbound_field(
        &relation_type.inbound_type,
        field_names.from_relation_to_inbound_entity,
        field_descriptions.from_relation_to_inbound_entity,
        context,
    ) {
        object = object.field(field);
    }
    object
}

pub fn get_relation_mutation_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for (relation_ty, relation_type) in context.relation_type_manager.get_all() {
        schema = schema.register(get_relation_mutation_type(&relation_ty, &relation_type));
    }
    schema
}

pub fn get_relation_mutation_type(relation_ty: &RelationTypeId, relation_type: &RelationType) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(relation_ty);
    let mut object = Object::new(dy_ty.mutation_type_name());
    if let Some(update_field) = get_relation_update_field(relation_type) {
        object = object.field(update_field);
    }
    if let Some(trigger_field) = get_trigger_field(relation_type) {
        object = object.field(trigger_field);
    }
    object = object.field(get_relation_delete_field());
    object
}

pub fn get_relation_update_field(relation_type: &RelationType) -> Option<Field> {
    let relation_type_inner = relation_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let mut update_field = Field::new("update", TypeRef::named_nn_list_nn(&dy_ty.to_string()), move |ctx| {
        let relation_type = relation_type_inner.clone();
        FieldFuture::new(async move {
            let relation_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveRelation>>()?;
            for relation_instance in relation_instances {
                // First validate all input fields for mutability and correct datatype
                for property in relation_type.properties.iter() {
                    if let Ok(value) = ctx.args.try_get(&property.name) {
                        // Fail on every property which is immutable
                        if property.mutability == Immutable {
                            return Err(mutability_error(property.value()));
                        }
                        match &property.data_type {
                            DataType::Null => {
                                // Fail on properties with the null datatype
                                return Err(data_type_error(property.value()));
                            }
                            DataType::Bool => {
                                if value.boolean().is_err() {
                                    // Fail if no boolean was set for a boolean property
                                    return Err(data_type_error(property.value()));
                                }
                            }
                            DataType::Number => {
                                if value.f64().is_err() && value.i64().is_err() && value.u64().is_err() {
                                    // Fail if no number was set for a number property
                                    return Err(data_type_error(property.value()));
                                }
                            }
                            DataType::String => {
                                if value.string().is_err() {
                                    // Fail if no string was set for a string property
                                    return Err(data_type_error(property.value()));
                                }
                            }
                            DataType::Array => {
                                if value.list().is_err() {
                                    // Fail if no list was set for a array property
                                    return Err(data_type_error(property.value()));
                                }
                            }
                            DataType::Object => {
                                if value.object().is_err() {
                                    // Fail if no object was set for a object property
                                    return Err(data_type_error(property.value()));
                                }
                            }
                            DataType::Any => {
                                // Accept input of any datatype
                            }
                        }
                    }
                }
                // Set properties
                for property in relation_type.properties.iter() {
                    if let Ok(value) = ctx.args.try_get(&property.name) {
                        match &property.data_type {
                            DataType::Null => {
                                return Err(data_type_error(property.value()));
                            }
                            DataType::Bool => {
                                relation_instance.set_checked(&property.name, Value::Bool(value.boolean()?));
                            }
                            DataType::Number => {
                                if let Ok(value) = value.i64() {
                                    relation_instance.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.u64() {
                                    relation_instance.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.f64() {
                                    relation_instance.set_checked(&property.name, json!(value));
                                } else {
                                    return Err(number_error(property.value()));
                                }
                            }
                            DataType::String => {
                                relation_instance.set_checked(&property.name, Value::String(value.string()?.to_string()));
                            }
                            DataType::Array => {
                                let _list = value.list()?;
                                let value = value.deserialize::<Value>()?;
                                if !value.is_array() {
                                    return Err(data_type_error(property.value()));
                                }
                                relation_instance.set_checked(&property.name, value);
                            }
                            DataType::Object => {
                                let value = value.deserialize::<Value>()?;
                                if !value.is_object() {
                                    return Err(data_type_error(property.value()));
                                }
                                relation_instance.set_checked(&property.name, value);
                            }
                            DataType::Any => {
                                // If it's possible to deserialize, accept the input
                                let value = value.deserialize::<Value>()?;
                                relation_instance.set_checked(&property.name, value);
                            }
                        }
                    }
                }
            }
            Ok(Some(FieldValue::list(
                relation_instances
                    .iter()
                    .map(|relation_instance| FieldValue::owned_any(relation_instance.clone())),
            )))
        })
    });
    let mut has_updatable_property = false;
    for property in relation_type.properties.iter() {
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

pub fn get_trigger_field(relation_type: &RelationType) -> Option<Field> {
    if !relation_type.is_a(&COMPONENT_ACTION) {
        return None;
    }
    let dy_ty = DynamicGraphTypeDefinition::from(&relation_type.ty);
    let trigger_field = Field::new(TRIGGER.property_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        FieldFuture::new(async move {
            let relation_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveRelation>>()?;
            for relation_instance in relation_instances {
                relation_instance.set(&TRIGGER.property_name(), json!(true));
            }
            Ok(Some(FieldValue::list(
                relation_instances
                    .iter()
                    .map(|relation_instance| FieldValue::owned_any(relation_instance.clone())),
            )))
        })
    });
    Some(trigger_field)
}

pub fn get_relation_delete_field() -> Field {
    Field::new("delete", TypeRef::named_nn_list_nn(TypeRef::ID), move |ctx| {
        FieldFuture::new(async move {
            let relation_instance_manager = ctx.data::<Arc<dyn ReactiveRelationManager>>()?;
            let mut ids = Vec::new();
            for reactive_relation in ctx.parent_value.try_downcast_ref::<Vec<ReactiveRelation>>()? {
                trace!("Deleting relation instance {}", reactive_relation);
                let id = reactive_relation.id();
                relation_instance_manager.delete(&id);
                ids.push(id);
            }
            Ok(Some(FieldValue::list(ids.iter().map(|id| {
                FieldValue::value(ID(id.to_string()))
            }))))
        })
    })
}
