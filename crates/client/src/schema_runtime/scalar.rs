use std::fmt::Display;
use std::fmt::Formatter;

use cynic::coercions::CoercesTo;
use serde_json::Value;

use crate::schema_runtime::schema::JSONObject;

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(schema_module = "crate::schema_runtime::schema", graphql_type = "JSON")]
pub struct Json(pub Value);

impl CoercesTo<JSONObject> for Json {}

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
