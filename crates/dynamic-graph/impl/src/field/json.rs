use async_graphql::ID;
use async_graphql::dynamic::FieldValue;
use serde_json::Value;
use uuid::Uuid;

pub fn to_field_value<'a>(v: Value) -> Option<FieldValue<'a>> {
    async_graphql::to_value(v).map(FieldValue::value).ok()
}

pub fn id_to_field_value<'a, ID: Into<Uuid>>(id: ID) -> FieldValue<'a> {
    FieldValue::value(ID(id.into().to_string()))
}

pub fn ids_to_field_value<'a>(ids: Vec<Uuid>) -> FieldValue<'a> {
    FieldValue::list(ids.iter().cloned().map(id_to_field_value))
}
