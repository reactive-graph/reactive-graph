use serde::{Deserialize, Serialize};

use crate::PropertyType;

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
}

impl Component {
    /// Constructs a new component with the given name and properties
    pub fn new<S: Into<String>>(name: S, properties: Vec<PropertyType>) -> Component {
        Component {
            name: name.into(),
            description: String::new(),
            properties,
        }
    }

    /// Constructs an component with the given name but without properties
    pub fn new_without_properties<S: Into<String>>(name: S) -> Component {
        Component {
            name: name.into(),
            description: String::new(),
            properties: Vec::new(),
        }
    }

    /// Returns true, if the component contains a property with the given name.
    pub fn has_property(&self, property_name: String) -> bool {
        self.properties.iter().any(|p| p.name == property_name)
    }
}
