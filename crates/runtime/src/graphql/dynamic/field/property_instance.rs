use async_graphql::dynamic::ResolverContext;
use async_graphql::Error;
use serde_json::json;
use serde_json::Value;

use crate::graphql::dynamic::number_error;
use crate::graphql::dynamic::data_type_error;
use crate::model::DataType;
use crate::model::PropertyTypes;
use crate::model::PropertyInstances;

/// Returns a list of property instances from the field arguments.
///
/// Returns an error if a property doesn't exist in the entity type or relation type.
///
/// First, initializes the properties with the default values of the property types of
/// an entity type or relation type. Next, overwrites the properties with the value
/// provided by the field arguments.
pub fn create_properties_from_field_arguments(ctx: &ResolverContext, property_types: &PropertyTypes) -> Result<PropertyInstances, Error> {
    // for field_name in ctx.args.keys() {
    //     if !property_types.contains_key(field_name.as_str()) {
    //         return Err(property_does_not_exist_error(&field_name.to_string()));
    //     }
    // }
    let properties = PropertyInstances::new_from_property_types_with_defaults(&property_types);
    for property in property_types.iter() {
        let field_arg_value = ctx.args.try_get(property.key())?;
        match &property.data_type {
            DataType::Null => {
                return Err(data_type_error(property.value()));
            }
            DataType::Bool => {
                properties.insert(property.key().clone(), Value::Bool(field_arg_value.boolean()?));
            }
            DataType::Number => {
                if let Ok(value) = field_arg_value.i64() {
                    properties.insert(property.key().clone(), json!(value));
                } else if let Ok(value) = field_arg_value.u64() {
                    properties.insert(property.key().clone(), json!(value));
                } else if let Ok(value) = field_arg_value.f64() {
                    properties.insert(property.key().clone(), json!(value));
                } else {
                    return Err(number_error(property.value()));
                }
            }
            DataType::String => {
                properties.insert(property.key().clone(), Value::String(field_arg_value.string()?.to_string()));
            }
            DataType::Array => {
                let _ = field_arg_value.list()?;
                let value = field_arg_value.deserialize::<Value>()?;
                if !value.is_array() {
                    return Err(data_type_error(property.value()));
                }
                properties.insert(property.key().clone(), value);
            }
            DataType::Object => {
                let value = field_arg_value.deserialize::<Value>()?;
                if !value.is_object() {
                    return Err(data_type_error(property.value()));
                }
                properties.insert(property.key().clone(), value);
            }
            DataType::Any => {
                if let Ok(value) = field_arg_value.deserialize() {
                    properties.insert(property.key().clone(), value);
                }
            }
        }
    }
    Ok(properties)
}