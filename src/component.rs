use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::PropertyType;

/// A component defines a set of properties to be applied to entity
/// types and relation types.
#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
pub struct Component {
    /// The name of the component.
    pub name: String,

    /// Textual description of the entity type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties which are applied on entity instances.
    pub properties: Vec<PropertyType>,
}

impl Component {
    /// Constructs a new component with the given name and properties
    pub fn new(name: String, properties: Vec<PropertyType>) -> Component {
        Component {
            name,
            description: String::new(),
            properties,
        }
    }

    /// Returns true, if the component contains a property with the given name.
    pub fn has_property(&self, property_name: String) -> bool {
        !self
            .properties
            .iter()
            .filter(|&p| p.name == property_name)
            .collect::<Vec<_>>()
            .is_empty()
    }
}
