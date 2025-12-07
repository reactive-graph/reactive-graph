use async_graphql::Error;
use async_graphql::dynamic::ResolverContext;
use reactive_graph_dynamic_graph_api::PropertyArgumentMissingError;
use reactive_graph_dynamic_graph_api::PropertyDataTypeError;
use serde_json::Value;
use serde_json::json;

use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyTypes;

/// Returns a list of property instances from the field arguments.
///
/// Returns an error if a property doesn't exist in the entity type or relation type.
///
/// First, initializes the properties with the default values of the property types of
/// an entity type or relation type. Next, overwrites the properties with the value
/// provided by the field arguments.
pub fn create_properties_from_field_arguments(ctx: &ResolverContext, property_types: &PropertyTypes, optional: bool) -> Result<PropertyInstances, Error> {
    let properties = PropertyInstances::new_from_property_types_with_defaults(property_types);
    for property in property_types.iter() {
        let Ok(field_arg_value) = ctx.args.try_get(property.key()) else {
            if optional {
                continue;
            } else {
                return Err(PropertyArgumentMissingError::PropertyArgumentMissing(property.key().clone()).into());
            }
        };
        match &property.data_type {
            DataType::Null => {
                return Err(PropertyDataTypeError::NullIsNotAValidDataType(property.key().clone()).into());
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
                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Number).into());
                }
            }
            DataType::String => {
                properties.insert(property.key().clone(), Value::String(field_arg_value.string()?.to_string()));
            }
            DataType::Array => {
                let _ = field_arg_value.list()?;
                let value = field_arg_value.deserialize::<Value>()?;
                if !value.is_array() {
                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Array).into());
                }
                properties.insert(property.key().clone(), value);
            }
            DataType::Object => {
                let value = field_arg_value.deserialize::<Value>()?;
                if !value.is_object() {
                    return Err(PropertyDataTypeError::ValueIsNotOfTheExpectedDataType(property.name.clone(), property.data_type, DataType::Object).into());
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
