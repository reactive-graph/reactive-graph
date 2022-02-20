use serde::{Deserialize, Serialize};

use crate::{Extension, PropertyType};

/// A component defines a set of properties to be applied to entity
/// types and relation types.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Component {
    /// The name of the component.
    pub name: String,

    /// Textual description of the component.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties which are applied on entity or relation instances.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Component specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,
}

impl Component {
    /// Constructs a new component with the given name and properties
    pub fn new<S: Into<String>>(name: S, properties: Vec<PropertyType>) -> Component {
        Component {
            name: name.into(),
            description: String::new(),
            properties,
            extensions: Vec::new(),
        }
    }

    /// Constructs a new component with the given name and properties
    pub fn new_with_extensions<S: Into<String>>(name: S, properties: Vec<PropertyType>, extensions: Vec<Extension>) -> Component {
        Component {
            name: name.into(),
            description: String::new(),
            properties,
            extensions,
        }
    }

    /// Constructs an component with the given name but without properties
    pub fn new_without_properties<S: Into<String>>(name: S) -> Component {
        Component {
            name: name.into(),
            description: String::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    /// Returns true, if the component contains a property with the given name.
    pub fn has_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    /// Returns true, if the component contains an extension with the given name.
    pub fn has_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }
}
