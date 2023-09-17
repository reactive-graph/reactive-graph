use serde::Deserialize;
use serde::Serialize;
use serde_json::from_value;
use serde_json::to_value;
use serde_json::Error;
use serde_json::Value;

use crate::ComponentTypeId;
use crate::TypeDefinition;

/// References a component of a type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDefinitionComponent {
    /// The type definition.
    #[serde(alias = "type")]
    pub type_definition: TypeDefinition,

    /// The component.
    pub component_ty: ComponentTypeId,
}

impl TypeDefinitionComponent {
    pub fn new<T: Into<TypeDefinition>, C: Into<ComponentTypeId>>(type_definition: T, component_ty: C) -> Self {
        TypeDefinitionComponent {
            type_definition: type_definition.into(),
            component_ty: component_ty.into(),
        }
    }
}

impl TryFrom<Value> for TypeDefinitionComponent {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let type_definition_component: Result<Self, Error> = from_value(value);
        type_definition_component
    }
}

impl TryFrom<TypeDefinitionComponent> for Value {
    type Error = Error;

    fn try_from(type_definition_component: TypeDefinitionComponent) -> Result<Self, Self::Error> {
        let value: Result<Self, Error> = to_value(type_definition_component);
        value
    }
}
