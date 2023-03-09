use async_graphql::dynamic::FieldValue;
use serde_json::Value;

pub fn to_field_value<'a>(v: Value) -> Option<FieldValue<'a>> {
    async_graphql::to_value(v).map(|v| FieldValue::value(v)).ok()
}
