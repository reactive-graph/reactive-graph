use indradb::Identifier;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::extension::Extension;
use crate::PropertyType;
use crate::TypeContainer;

pub static NAMESPACE_ENTITY_TYPE: Uuid = Uuid::from_u128(0x6ba7c8109dcd11c180b400d04fd530c7);

/// Entity types defines the type of an entity instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityType {
    /// The namespace the entity type belongs to.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// The name of the entity type.
    ///
    /// The name is the unique identifier for entity types.
    pub name: String,

    /// Textual description of the entity type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the entity type.
    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    /// The properties which are defined by the entity type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Entity type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,

    #[serde(skip)]
    pub t: Identifier,
}

impl EntityType {
    pub fn new<S: Into<String>>(
        namespace: S,
        name: S,
        description: S,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> EntityType {
        let namespace = namespace.into();
        let name = name.into();
        let t = fully_qualified_identifier(namespace.as_str(), name.as_str());
        EntityType {
            namespace,
            name,
            description: description.into(),
            components,
            properties,
            extensions,
            t,
        }
    }
}

impl TypeContainer for EntityType {
    fn fully_qualified_name(&self) -> String {
        format!("{}__{}", self.namespace, self.name)
    }

    fn is_a<S: Into<String>>(&self, component_name: S) -> bool {
        self.components.contains(&component_name.into())
    }

    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        let property_name = property_name.into();
        self.properties.iter().any(|p| p.name == property_name)
    }

    fn has_own_extension<S: Into<String>>(&self, extension_name: S) -> bool {
        let extension_name = extension_name.into();
        self.extensions.iter().any(|extension| extension.name == extension_name)
    }
}

fn fully_qualified_identifier(namespace: &str, name: &str) -> Identifier {
    let fully_qualified_name = format!("{namespace}__{name}");
    Identifier::new(fully_qualified_name.as_str())
        .unwrap_or_else(|_| Identifier::new(Uuid::new_v5(&NAMESPACE_ENTITY_TYPE, fully_qualified_name.as_bytes()).to_string()).unwrap())
}
