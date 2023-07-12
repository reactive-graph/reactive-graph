use std::cmp::Ordering;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::ExtensionTypeId;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// Extension on a type. The extension allows to extend information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Extension {
    /// The type definition contains the namespace and the type name.
    #[serde(flatten)]
    pub ty: ExtensionTypeId,

    /// Textual description of the extension.
    #[serde(default = "String::new")]
    pub description: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

impl Extension {
    /// Constructs an extension from the given namespaced type with the given description, components, properties and extensions.
    pub fn new<T: Into<ExtensionTypeId>, S: Into<String>>(ty: T, description: S, extension: Value) -> Extension {
        Extension {
            ty: ty.into(),
            description: description.into(),
            extension,
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S, description: S, extension: Value) -> Extension {
        Extension {
            ty: ExtensionTypeId::new_from_type(namespace, type_name),
            description: description.into(),
            extension,
        }
    }
}

impl NamespacedTypeGetter for Extension {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for Extension {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}

impl From<&Extension> for TypeDefinition {
    fn from(extension: &Extension) -> Self {
        TypeDefinition {
            type_id_type: TypeIdType::Extension,
            namespace: extension.namespace(),
            type_name: extension.type_name(),
        }
    }
}

impl PartialOrd<Self> for Extension {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Extension {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty.cmp(&other.ty)
    }
}
