use crate::extension::divergent::is_divergent;
use crate::field::flow::flow_id_field;
use crate::field::flow::flow_property_field;
use crate::field::instance_component_id_field;
use crate::field::to_input_type_ref;
use crate::interface::flow::INTERFACE_FLOW;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::SchemaBuilder;
use async_graphql::dynamic::TypeRef;
use itertools::Itertools;
use log::trace;
use reactive_graph_dynamic_graph_api::ImmutablePropertyError;
use reactive_graph_dynamic_graph_api::PropertyDataTypeError;
use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::DataType;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::Mutability::Immutable;
use reactive_graph_graph::Mutability::Mutable;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_runtime_model::COMPONENT_ACTION;
use serde_json::Value;
use serde_json::json;
use std::sync::Arc;

pub fn get_flow_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for flow_type in context.flow_type_manager.get_all().iter() {
        schema = schema.register(get_flow_type(flow_type.key(), flow_type.value(), context));
    }
    schema
}

pub fn get_flow_type(relation_ty: &FlowTypeId, flow_type: &FlowType, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(relation_ty);
    let mut object = Object::new(dy_ty.to_string()).description(&flow_type.description).implement(INTERFACE_FLOW);
    // ID field
    object = object.field(flow_id_field());
    // wrapper entity instance
    // entities
    // relations
    // variables

    // Only applicable if the entity type of the flow type actually exists
    let entity_ty = flow_type.wrapper_type();
    if let Some(entity_type) = context.entity_type_manager.get(&entity_ty) {
        // Components
        for component_ty in entity_type.components.iter() {
            object = object.field(instance_component_id_field(&component_ty));
            let component_dy_ty = DynamicGraphTypeDefinition::from(component_ty.key());
            if !is_divergent(&entity_type, component_ty.key()) {
                object = object.implement(component_dy_ty.to_string());
            }
        }
        // Property Fields
        for property_type in entity_type.properties.iter().sorted_by(|a, b| Ord::cmp(&a.key(), &b.key())) {
            object = object.field(flow_property_field(&property_type));
        }
    }

    object
}

pub fn get_flow_mutation_types(mut schema: SchemaBuilder, context: &SchemaBuilderContext) -> SchemaBuilder {
    for (flow_ty, flow_type) in context.flow_type_manager.get_all() {
        schema = schema.register(get_flow_mutation_type(&flow_ty, &flow_type, context));
    }
    schema
}

pub fn get_flow_mutation_type(flow_ty: &FlowTypeId, flow_type: &FlowType, context: &SchemaBuilderContext) -> Object {
    let dy_ty = DynamicGraphTypeDefinition::from(flow_ty);
    let mut object = Object::new(dy_ty.mutation_type_name());
    if let Some(update_field) = get_flow_update_field(flow_type, context) {
        object = object.field(update_field);
    }
    if let Some(trigger_field) = get_flow_type_trigger_field(flow_type) {
        object = object.field(trigger_field);
    }
    object = object.field(get_flow_delete_field());
    object
}

pub fn get_flow_update_field(flow_type: &FlowType, context: &SchemaBuilderContext) -> Option<Field> {
    let entity_ty = flow_type.wrapper_type();
    let entity_type = context.entity_type_manager.get(&entity_ty)?;
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&flow_type.ty);
    let mut update_field = Field::new("update", TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let flow_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveFlow>>()?;
            for flow_instance in flow_instances {
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
                                flow_instance.set_checked(&property.name, Value::Bool(value.boolean()?));
                            }
                            DataType::Number => {
                                if let Ok(value) = value.i64() {
                                    flow_instance.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.u64() {
                                    flow_instance.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.f64() {
                                    flow_instance.set_checked(&property.name, json!(value));
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
                                flow_instance.set_checked(&property.name, Value::String(value.string()?.to_string()));
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
                                flow_instance.set_checked(&property.name, value);
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
                                flow_instance.set_checked(&property.name, value);
                            }
                            DataType::Any => {
                                // If it's possible to deserialize, accept the input
                                let value = value.deserialize::<Value>()?;
                                flow_instance.set_checked(&property.name, value);
                            }
                        }
                    }
                }
            }
            Ok(Some(FieldValue::list(
                flow_instances.iter().map(|flow_instance| FieldValue::owned_any(flow_instance.clone())),
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

pub fn get_flow_type_trigger_field(flow_type: &FlowType) -> Option<Field> {
    if !flow_type.wrapper_entity_instance.is_a(&COMPONENT_ACTION) {
        return None;
    }
    let dy_ty = DynamicGraphTypeDefinition::from(&flow_type.ty);
    let trigger_field = Field::new(TRIGGER.property_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        FieldFuture::new(async move {
            let flow_instances = ctx.parent_value.try_downcast_ref::<Vec<ReactiveFlow>>()?;
            for flow_instance in flow_instances {
                flow_instance.set(TRIGGER.property_name(), json!(true));
            }
            Ok(Some(FieldValue::list(
                flow_instances.iter().map(|flow_instance| FieldValue::owned_any(flow_instance.clone())),
            )))
        })
    });
    Some(trigger_field)
}

pub fn get_flow_delete_field() -> Field {
    Field::new("delete", TypeRef::named_nn_list_nn(TypeRef::ID), move |ctx| {
        FieldFuture::new(async move {
            let flow_instance_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
            let mut ids = Vec::new();
            for flow_instance in ctx.parent_value.try_downcast_ref::<Vec<ReactiveFlow>>()? {
                trace!("Deleting flow instance {flow_instance}");
                let id = flow_instance.id;
                flow_instance_manager.delete(id);
                ids.push(id);
            }
            Ok(Some(FieldValue::list(ids.iter().map(|id| FieldValue::value(ID(id.to_string()))))))
        })
    })
}
