use std::cmp::Ordering;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::extension::Extension;
use crate::DataType;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Mutability;
use crate::SocketType;

pub static NAMESPACE_PROPERTY_TYPE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d02fd540c7);

/// Definition of a property. The definition contains
/// the name of the property, the data type and the socket
/// type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PropertyType {
    /// The name of the property
    pub name: String,

    /// The description of the property.
    #[serde(default = "String::new")]
    pub description: String,

    /// The data type of the property
    pub data_type: DataType,

    /// Specifies the type of socket - either input socket or output socket or none
    #[serde(default = "SocketType::none")]
    pub socket_type: SocketType,

    /// Specifies if the property is mutable.
    #[serde(default = "Mutability::mutable")]
    pub mutability: Mutability,

    /// Property specific extensions
    #[serde(default = "Vec::<Extension>::new")]
    pub extensions: Vec<Extension>,
}

impl PropertyType {
    pub fn new<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::None,
            mutability: Mutability::Mutable,
            extensions: Vec::new(),
        }
    }

    pub fn new_with_socket<S: Into<String>>(name: S, data_type: DataType, socket_type: SocketType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type,
            mutability: Mutability::Mutable,
            extensions: Vec::new(),
        }
    }

    pub fn input<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::Input,
            mutability: Mutability::Mutable,
            extensions: Vec::new(),
        }
    }

    pub fn output<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::Output,
            mutability: Mutability::Immutable,
            extensions: Vec::new(),
        }
    }

    pub fn new_with_all<S: Into<String>>(
        name: S,
        description: S,
        data_type: DataType,
        socket_type: SocketType,
        mutability: Mutability,
        extensions: Vec<Extension>,
    ) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: description.into(),
            data_type,
            socket_type,
            mutability,
            extensions,
        }
    }

    pub fn bool<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Bool)
    }

    pub fn number<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Number)
    }

    pub fn string<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::String)
    }

    pub fn array<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Array)
    }

    pub fn object<S: Into<String>>(name: S) -> PropertyType {
        PropertyType::new(name, DataType::Object)
    }

    /// Returns true, if the property contains an extension with the given type.
    pub fn has_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.extensions.iter().any(|extension| &extension.ty == ty)
    }

    pub fn get_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.iter().find(|extension| &extension.ty == ty).cloned()
    }
}

impl ExtensionContainer for PropertyType {
    fn has_own_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.has_extension(ty)
    }

    fn get_own_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.get_extension(ty)
    }

    fn merge_extensions(&mut self, extensions_to_merge: Vec<Extension>) {
        for extension_to_merge in extensions_to_merge {
            if !self.has_own_extension(&extension_to_merge.ty) {
                self.extensions.push(extension_to_merge);
            } else if let Some(existing_extension) = self.extensions.iter_mut().find(|e| e.ty == extension_to_merge.ty) {
                existing_extension.description = extension_to_merge.description.clone();
                existing_extension.extension = extension_to_merge.extension.clone();
            }
        }
    }
}

impl PartialOrd<Self> for PropertyType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PropertyType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

pub trait PropertyTypeDefinition {
    /// The property name.
    fn property_name(&self) -> String;

    /// The default value of the property.
    fn default_value(&self) -> Value;
}
