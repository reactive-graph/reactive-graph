use crate::NamespacedTypeGetter;
use crate::PropertyTypeContainer;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use schemars::Schema;
use schemars::consts::meta_schemas::DRAFT2020_12;
use schemars::json_schema;
use serde::Serialize;
use serde::Serializer;
use serde_json::Value;
use serde_json::json;
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
        write!(f, "{}/dynamic_graph/{}.schema.json", JSON_SCHEMA_ID_URI_PREFIX, self.0.relative_url(),)
    }
}

impl<T> From<&T> for JsonSchemaId
where
    T: TypeDefinitionGetter,
{
    fn from(ty: &T) -> Self {
        Self::new(ty.type_definition())
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

pub trait JsonSchemaIdGetter: TypeDefinitionGetter {
    /// Returns the JSON Schema identifier ($id).
    fn json_schema_id(&self) -> JsonSchemaId {
        JsonSchemaId::new(self.type_definition())
    }

    fn json_schema_id_property(&self) -> Value {
        json!({
            "default": self.json_schema_id(),
            "description": "The schema identifier",
            "type": "string"
        })
    }
}

impl<T> JsonSchemaIdGetter for T where T: TypeDefinitionGetter {}

pub trait TypeDefinitionJsonSchemaGetter: TypeDefinitionGetter {
    /// Returns the JSON Schema.
    fn json_schema(&self) -> Schema;
}

pub struct TypeDefinitionJsonSchema(Schema);

impl TypeDefinitionJsonSchema {
    pub fn new<T: TypeDefinitionGetter + PropertyTypeContainer + JsonSchemaIdGetter>(ty: &T) -> Self {
        let schema_id = ty.json_schema_id();
        let title = ty.type_definition().namespace().to_string();
        let property_types = ty.get_own_properties_cloned();
        let mut properties = property_types.as_json_schema_properties();
        properties.insert("$id".to_string(), ty.json_schema_id_property());
        let mut required = property_types.names();
        required.sort();
        let schema = json_schema!({
            "$schema": DRAFT2020_12,
            "$id": schema_id,
            "type": "object",
            "title": title,
            "properties": properties,
            "required": required,
        });
        Self(schema)
    }

    pub fn title<S: Into<String>>(mut self, title: S) -> TypeDefinitionJsonSchema {
        self.0.insert("title".to_string(), Value::String(title.into()));
        self
    }

    pub fn description<S: Into<String>>(mut self, description: S) -> TypeDefinitionJsonSchema {
        self.0.insert("description".to_string(), Value::String(description.into()));
        self
    }

    pub fn property<S: Into<String>, V: Into<Value>>(mut self, property_name: S, value: V) -> TypeDefinitionJsonSchema {
        if let Some(properties) = self.0.ensure_object().get_mut("properties").and_then(Value::as_object_mut) {
            properties.insert(property_name.into(), value.into());
        }
        self
    }

    pub fn required<S: Into<String>>(mut self, property_name: S) -> TypeDefinitionJsonSchema {
        if let Some(properties) = self.0.ensure_object().get_mut("required").and_then(Value::as_array_mut) {
            properties.push(Value::String(property_name.into()));
        }
        self
    }

    pub fn required_property<S: Into<String>, V: Into<Value>>(self, property_name: S, value: V) -> TypeDefinitionJsonSchema {
        let property_name = property_name.into();
        self.property(&property_name, value).required(&property_name)
    }

    pub fn required_id_property<S: Into<String>>(self, property_name: S) -> TypeDefinitionJsonSchema {
        self.required_property(
            property_name,
            json!({
                "type": "string",
                "format": "uuid"
            }),
        )
    }

    pub fn required_string_property<S: Into<String>>(self, property_name: S) -> TypeDefinitionJsonSchema {
        self.required_property(
            property_name,
            json!({
                "type": "string"
            }),
        )
    }
}

impl<T> From<&T> for TypeDefinitionJsonSchema
where
    T: TypeDefinitionGetter + PropertyTypeContainer + JsonSchemaIdGetter,
{
    fn from(ty: &T) -> Self {
        let schema_id = ty.json_schema_id();
        let title = ty.type_definition().namespace().to_string();
        let property_types = ty.get_own_properties_cloned();
        let mut properties = property_types.as_json_schema_properties();
        properties.insert("$id".to_string(), ty.json_schema_id_property());
        let required = property_types.names();
        let schema = json_schema!({
            "$schema": DRAFT2020_12,
            "$id": schema_id,
            "type": "object",
            "title": title,
            "properties": properties,
            "required": required,
        });
        Self(schema)
    }
}
impl From<TypeDefinitionJsonSchema> for Schema {
    fn from(schema: TypeDefinitionJsonSchema) -> Self {
        schema.0
    }
}
