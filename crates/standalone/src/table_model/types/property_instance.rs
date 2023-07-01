use serde_json::Value;

use crate::table_model::types::property_type::PropertyType;

pub struct PropertyInstance {
    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub type_: Option<PropertyType>,

    /// The extension as JSON representation.
    pub value: Value,
}
