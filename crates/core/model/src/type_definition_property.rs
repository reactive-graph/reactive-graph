use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;
use serde_json::to_value;
use serde_json::Error;
use serde_json::Value;

use crate::TypeDefinition;

/// References a property of a type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TypeDefinitionProperty {
    /// The type definition.
    #[serde(alias = "type")]
    pub type_definition: TypeDefinition,

    /// The property name.
    pub property: String,
}

impl TypeDefinitionProperty {
    pub fn new<T: Into<TypeDefinition>>(type_definition: T, property: String) -> Self {
        TypeDefinitionProperty {
            type_definition: type_definition.into(),
            property,
        }
    }
}

impl TryFrom<Value> for TypeDefinitionProperty {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let type_definition_property: Result<Self, Error> = from_value(value);
        type_definition_property
    }
}

impl TryFrom<TypeDefinitionProperty> for Value {
    type Error = Error;

    fn try_from(type_definition_property: TypeDefinitionProperty) -> Result<Self, Self::Error> {
        let value: Result<Self, Error> = to_value(type_definition_property);
        value
    }
}
