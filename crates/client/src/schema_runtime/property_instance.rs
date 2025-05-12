use crate::schema_runtime::scalar::Json;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-runtime-schema.graphql",
    schema_module = "crate::schema_runtime::schema"
)]
pub struct CommandResult {
    /// The name of the extension.
    pub name: String,

    // /// Textual description of the extension.
    // pub type_: Option<PropertyType>,
    /// The extension as JSON representation.
    pub value: Json,
}
