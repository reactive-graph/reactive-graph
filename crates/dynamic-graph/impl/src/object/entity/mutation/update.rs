use crate::field::to_input_type_ref;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;
use reactive_graph_dynamic_graph_api::ImmutablePropertyError;
use reactive_graph_dynamic_graph_api::PropertyDataTypeError;
use reactive_graph_graph::DataType;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::Mutability::Immutable;
use reactive_graph_graph::Mutability::Mutable;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use serde_json::Value;
use serde_json::json;

pub fn entity_update_field(entity_type: &EntityType) -> Option<Field> {
    let entity_type_inner = entity_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&entity_type.ty);
    let mut update_field = Field::new("update", TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let entity_type = entity_type_inner.clone();
        FieldFuture::new(async move {
            let reactive_entities = ctx.parent_value.try_downcast_ref::<Vec<ReactiveEntity>>()?;
            for reactive_entity in reactive_entities {
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
                                reactive_entity.set_checked(&property.name, Value::Bool(value.boolean()?));
                            }
                            DataType::Number => {
                                if let Ok(value) = value.i64() {
                                    reactive_entity.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.u64() {
                                    reactive_entity.set_checked(&property.name, json!(value));
                                } else if let Ok(value) = value.f64() {
                                    reactive_entity.set_checked(&property.name, json!(value));
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
                                reactive_entity.set_checked(&property.name, Value::String(value.string()?.to_string()));
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
                                reactive_entity.set_checked(&property.name, value);
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
                                reactive_entity.set_checked(&property.name, value);
                            }
                            DataType::Any => {
                                // If it's possible to deserialize, accept the input
                                let value = value.deserialize::<Value>()?;
                                reactive_entity.set_checked(&property.name, value);
                            }
                        }
                    }
                }
            }
            Ok(Some(FieldValue::list(
                reactive_entities.iter().map(|entity_instance| FieldValue::owned_any(entity_instance.clone())),
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
