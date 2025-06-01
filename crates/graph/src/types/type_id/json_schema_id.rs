use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;
use std::fmt::Display;
use std::fmt::Formatter;

pub const JSON_SCHEMA_ID_URI_PREFIX: &str = "https://schema.reactive-graph.io/schema/json";

pub struct JsonSchemaId(TypeDefinition);

impl JsonSchemaId {
    pub fn new<TD: Into<TypeDefinition>>(type_definition: TD) -> Self {
        Self(type_definition.into())
    }
}

impl Display for JsonSchemaId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/dynamic_graph/{}/{}/{}.schema.json",
            JSON_SCHEMA_ID_URI_PREFIX,
            self.0.type_id_type.full_name().to_lowercase(),
            self.0.namespace,
            self.0.type_name,
        )
    }
}

impl<T> From<&T> for JsonSchemaId
where
    T: TypeDefinitionGetter,
{
    fn from(value: &T) -> Self {
        Self::new(value.type_definition())
    }
}

impl From<JsonSchemaId> for Value {
    fn from(json_schema_id: JsonSchemaId) -> Self {
        json_schema_id.to_string().into()
    }
}

impl Serialize for JsonSchemaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
