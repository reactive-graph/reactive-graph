use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;
use serde_json::to_value;
use serde_json::Error;
use serde_json::Value;

use crate::TypeDefinition;

/// References an extension of a type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TypeDefinitionExtension {
    /// The type definition.
    #[serde(alias = "type")]
    pub type_definition: TypeDefinition,

    /// The extension name.
    pub extension: String,
}

impl TypeDefinitionExtension {
    pub fn new<T: Into<TypeDefinition>>(type_definition: T, extension: String) -> Self {
        TypeDefinitionExtension {
            type_definition: type_definition.into(),
            extension,
        }
    }
}

impl TryFrom<Value> for TypeDefinitionExtension {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let type_definition_extension: Result<Self, Error> = from_value(value);
        type_definition_extension
    }
}

impl TryFrom<TypeDefinitionExtension> for Value {
    type Error = Error;

    fn try_from(type_definition_extension: TypeDefinitionExtension) -> Result<Self, Self::Error> {
        let value: Result<Self, Error> = to_value(type_definition_extension);
        value
    }
}
