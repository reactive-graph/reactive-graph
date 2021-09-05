use std::str::FromStr;

use async_graphql::SimpleObject;
use indradb::Type;
use serde::{Deserialize, Serialize};

use crate::extension::Extension;
use crate::PropertyType;

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
pub struct RelationType {
    /// The name of the outbound entity type.
    pub outbound_type: String,

    /// The name of the relation type.
    ///
    /// The name is the unique identifier for relation types.
    #[serde(alias = "name")]
    #[graphql(name = "name")]
    pub type_name: String,

    /// The full type name of the relation type.
    #[serde(default = "String::new")]
    pub full_name: String,

    /// The name of the inbound entity type.
    pub inbound_type: String,

    /// The relation type belongs to the given group of relation types.
    #[serde(default = "String::new")]
    pub group: String,

    /// Textual description of the relation type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the relation type.
    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    /// The names of the behaviours to be applied on the relation instances.
    #[serde(default = "Vec::new")]
    pub behaviours: Vec<String>,

    /// The properties which are defined by the relation type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,

    #[serde(skip)]
    #[graphql(skip)]
    pub t: Type,
}

impl RelationType {
    pub fn new(
        outbound_type: String,
        type_name: String,
        inbound_type: String,
        components: Vec<String>,
        behaviours: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> RelationType {
        let t = Type::from_str(type_name.clone().as_str()).unwrap();
        RelationType {
            outbound_type,
            full_name: type_name.clone(),
            type_name,
            inbound_type,
            group: String::new(),
            description: String::new(),
            components,
            behaviours,
            properties,
            extensions,
            t,
        }
    }

    /// Returns true, if the relation type is a component with the given name.
    pub fn is_a(&self, component_name: String) -> bool {
        self.components.contains(&component_name)
    }

    /// Returns true, if the relation type behaves as the behaviour with the given name.
    pub fn behaves_as(&self, behaviour_name: String) -> bool {
        self.behaviours.contains(&behaviour_name)
    }

    /// Returns true, if the relation type contains an own property with the given name.
    /// Doesn't respect properties from potential components.
    pub fn has_own_property(&self, property_name: String) -> bool {
        !self
            .properties
            .iter()
            .filter(|&p| p.name == property_name)
            .collect::<Vec<_>>()
            .is_empty()
    }
}
