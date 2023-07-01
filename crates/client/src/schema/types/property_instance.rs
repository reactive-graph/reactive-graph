use crate::schema::scalar::Json;
use crate::schema::types::property_type::PropertyType;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct PropertyInstance {
    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub type_: Option<PropertyType>,

    /// The extension as JSON representation.
    pub value: Json,
}
