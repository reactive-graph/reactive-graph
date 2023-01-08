use async_graphql::dynamic::FieldValue;
use serde_json::Value;

// use crate::model::DataType;
// use crate::model::PropertyType;

pub fn to_field_value<'a>(v: Value) -> Option<FieldValue<'a>> {
    async_graphql::to_value(v).map(|v| FieldValue::value(v)).ok()
}

// pub fn to_field_value_2(v: Value, property_type: &PropertyType) -> Option<FieldValue> {
//     match property_type.data_type {
//         DataType::Null => Some(FieldValue::value(())),
//         DataType::Bool => v.as_bool().map(|v| FieldValue::value(v)),
//         DataType::Number => v.as_f64().map(|v| FieldValue::value(v)),
//         DataType::String => v.as_str().map(|v| FieldValue::value(v)),
//         DataType::Array => v.as_array().map(|v| FieldValue::value(v)),
//         DataType::Object => v.as_object().map(|v| FieldValue::value(v)),
//         DataType::Any => FieldValue::value(v),
//     }
// }
