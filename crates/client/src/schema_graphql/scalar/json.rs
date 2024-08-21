use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(schema_module = "crate::schema_graphql::schema", graphql_type = "JSON")]
pub struct Json(pub Value);

impl From<Value> for Json {
    fn from(value: Value) -> Self {
        Json(value)
    }
}

impl From<Json> for Value {
    fn from(value: Json) -> Self {
        value.0
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
