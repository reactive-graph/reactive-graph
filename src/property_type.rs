use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::extension::Extension;
use crate::DataType;
use crate::SocketType;

pub static NAMESPACE_PROPERTY_TYPE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d02fd540c7);

/// Definition of a property. The definition contains
/// the name of the property, the data type and the socket
/// type.
#[derive(Serialize, Deserialize, Clone, Debug)]
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

    /// Property specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl PropertyType {
    pub fn new<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::None,
            extensions: Vec::new(),
        }
    }

    pub fn new_with_socket<S: Into<String>>(name: S, data_type: DataType, socket_type: SocketType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type,
            extensions: Vec::new(),
        }
    }

    pub fn input<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::Input,
            extensions: Vec::new(),
        }
    }

    pub fn output<S: Into<String>>(name: S, data_type: DataType) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: String::new(),
            data_type,
            socket_type: SocketType::Output,
            extensions: Vec::new(),
        }
    }

    pub fn new_with_all<S: Into<String>>(name: S, description: S, data_type: DataType, socket_type: SocketType, extensions: Vec<Extension>) -> PropertyType {
        PropertyType {
            name: name.into(),
            description: description.into(),
            data_type,
            socket_type,
            extensions,
        }
    }
}
