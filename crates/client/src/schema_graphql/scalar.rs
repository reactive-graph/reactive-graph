use std::fmt::Display;
use std::fmt::Formatter;

use serde_json::Value;

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(schema_module = "crate::schema_graphql::schema", graphql_type = "JSON")]
pub struct Json(pub Value);

impl From<Value> for Json {
    fn from(value: Value) -> Self {
        Json(value)
    }
}

impl Display for Json {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
