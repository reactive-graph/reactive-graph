use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;
use serde_json::to_value;
use serde_json::Error;
use serde_json::Value;

use crate::ExtensionTypeId;
use crate::TypeDefinition;

/// References an extension of a type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDefinitionExtension {
    /// The type definition.
    #[serde(alias = "type")]
    pub type_definition: TypeDefinition,

    /// The extension name.
    pub extension_ty: ExtensionTypeId,
}

impl TypeDefinitionExtension {
    pub fn new<T: Into<TypeDefinition>, X: Into<ExtensionTypeId>>(type_definition: T, extension_ty: X) -> Self {
        TypeDefinitionExtension {
            type_definition: type_definition.into(),
            extension_ty: extension_ty.into(),
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
