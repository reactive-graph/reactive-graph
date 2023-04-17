use crate::schema::property_type::PropertyType;
use crate::schema::scalar::Json;

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
