use crate::field::json::to_field_value;
use crate::field::property::datatype::to_input_type_ref;
use crate::field::property::datatype::to_type_ref;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::ResolverContext;
use itertools::Itertools;
use log::trace;
use log::warn;
use reactive_graph_dynamic_graph_api::ImmutablePropertyError;
use reactive_graph_dynamic_graph_api::PropertyContainerValidateUpdateError;
use reactive_graph_dynamic_graph_api::PropertyDataTypeError;
use reactive_graph_graph::DataType;
use reactive_graph_graph::Mutability::Immutable;
use reactive_graph_graph::Mutability::Mutable;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_reactive_model_api::ReactiveInstanceUnidentifiable;
use serde_json::Value;
use serde_json::json;
use std::sync::Arc;

pub mod datatype;

/// Adds fields for all properties of a `PropertyTypeContainer` to the `PropertyTypeContainer`'s
/// object.
pub fn property_container_property_fields<T: PropertyTypeContainer, I: PropertyInstanceGetter + 'static>(
    property_type_container: &T,
    mut object: Object,
) -> Object {
    for property_type in property_type_container
        .get_own_properties_cloned()
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.key(), &b.key()))
    {
        object = object.field(to_property_field::<I>(&property_type));
    }
    object
}

/// Creates a field for a property type of reactive instance.
pub fn to_property_field<T: PropertyInstanceGetter + 'static>(property_type: &PropertyType) -> Field {
    let property_type_inner = property_type.clone();
    Field::new(&property_type.name, to_type_ref(&property_type.data_type), move |ctx| {
        let property_type = property_type_inner.clone();
        FieldFuture::new(async move {
            if let Ok(reactive_instance) = ctx.parent_value.try_downcast_ref::<T>() {
                trace!("Resolved reactive instance using type bound");
                return Ok(reactive_instance.get(&property_type.name).and_then(to_field_value));
            }
            if let Ok(reactive_instance) = ctx.parent_value.try_downcast_ref::<Arc<Box<dyn ReactiveInstanceUnidentifiable>>>() {
                trace!("Resolved reactive instance using downcast to ReactiveInstanceUnidentifiable");
                return Ok(reactive_instance.as_ref().get(&property_type.name).and_then(to_field_value));
            }
            Err(Error::new("Failed to resolve reactive instance"))
            // match ctx.parent_value.try_downcast_ref::<T>() {
            //     Ok(reactive_instance) => {
            //         info!("Using Type Bounds");
            //         Ok(reactive_instance.get(&property_type.name).and_then(to_field_value))
            //     }
            //     Err(_) => {
            //         info!("Downcast ReactiveInstanceUnidentifiable");
            //         let reactive_instance = ctx.parent_value.try_downcast_ref::<Arc<Box<dyn ReactiveInstanceUnidentifiable>>>()?;
            //         // let r = reactive_instance.as_ref().as_ref().get;
            //         Ok(reactive_instance.as_ref().get(&property_type.name).and_then(to_field_value))
            //     }
            // }
            // let reactive_instance = ctx.parent_value.try_downcast_ref::<T>()?;
            // info!("y");
            // Ok(reactive_instance.get(&property_type.name).and_then(to_field_value))
        })
    })
    .description(&property_type.description)
}

/// Validate all input fields for mutability and correct datatype.
pub fn property_container_validate_input_fields<T: PropertyTypeContainer>(
    property_type_container: &T,
    ctx: &ResolverContext,
) -> Result<(), PropertyContainerValidateUpdateError> {
    // First validate all input fields for mutability and correct datatype
    for property in property_type_container.get_own_properties_cloned().iter() {
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
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Bool).into());
                    }
                }
                DataType::Number => {
                    if value.f64().is_err() && value.i64().is_err() && value.u64().is_err() {
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Number).into());
                    }
                }
                DataType::String => {
                    if value.string().is_err() {
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::String).into());
                    }
                }
                DataType::Array => {
                    if value.list().is_err() {
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Array).into());
                    }
                }
                DataType::Object => {
                    if value.object().is_err() {
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Object).into());
                    }
                }
                DataType::Any => {
                    // Accept input of any datatype
                }
            }
        }
    }
    Ok(())
}

pub fn property_container_update_properties<T: PropertyTypeContainer, I: PropertyInstanceSetter + ?Sized>(
    property_type_container: &T,
    reactive_instance: &I,
    ctx: &ResolverContext,
) -> Result<(), Error> {
    for property in property_type_container.get_own_properties_cloned().iter() {
        if let Ok(value) = ctx.args.try_get(&property.name) {
            match &property.data_type {
                DataType::Null => {
                    return Err(PropertyDataTypeError::NullIsNotAValidDataType(property.key().clone()).into());
                }
                DataType::Bool => {
                    reactive_instance.set_checked(&property.name, Value::Bool(value.boolean()?));
                }
                DataType::Number => {
                    if let Ok(value) = value.i64() {
                        reactive_instance.set_checked(&property.name, json!(value));
                    } else if let Ok(value) = value.u64() {
                        reactive_instance.set_checked(&property.name, json!(value));
                    } else if let Ok(value) = value.f64() {
                        reactive_instance.set_checked(&property.name, json!(value));
                    } else {
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Number).into());
                    }
                }
                DataType::String => {
                    reactive_instance.set_checked(&property.name, Value::String(value.string()?.to_string()));
                }
                DataType::Array => {
                    let _list = value.list()?;
                    let value = value.deserialize::<Value>()?;
                    if !value.is_array() {
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Array).into());
                    }
                    reactive_instance.set_checked(&property.name, value);
                }
                DataType::Object => {
                    let value = value.deserialize::<Value>()?;
                    if !value.is_object() {
                        return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Object).into());
                    }
                    reactive_instance.set_checked(&property.name, value);
                }
                DataType::Any => {
                    // If it's possible to deserialize, accept the input
                    let value = value.deserialize::<Value>()?;
                    reactive_instance.set_checked(&property.name, value);
                }
            }
        }
    }
    Ok(())
}

pub fn property_container_update_field_arguments<T: PropertyTypeContainer + NamespacedTypeGetter>(
    property_type_container: &T,
    mut field: Field,
) -> Option<Field> {
    let mut has_updatable_property = false;
    for property in property_type_container.get_own_properties_cloned().iter() {
        if property.mutability == Mutable {
            if let Some(type_ref) = to_input_type_ref(property.value(), true) {
                field = field.argument(InputValue::new(&property.name, type_ref));
                has_updatable_property = true;
            }
        }
    }
    if !has_updatable_property {
        warn!("{} has no updatable properties!", property_type_container.namespaced_type());
        return None;
    }
    Some(field)
}
