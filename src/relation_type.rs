use indradb::Identifier;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::extension::Extension;
use crate::PropertyType;
use crate::TypeContainer;

pub static NAMESPACE_RELATION_TYPE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d01fd530c7);

/// A relation type defines the type of an relation instance.
///
/// The relation type defines the entity types of the outbound and inbound entity instances.
/// Also the relation type defines the properties of the relation instance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationType {
    /// The namespace the relation type belongs to.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// The name of the outbound entity type.
    pub outbound_type: String,

    /// The name of the relation type.
    ///
    /// The name is the unique identifier for relation types.
    #[serde(alias = "name")]
    pub type_name: String,

    /// The instance type name is unique between two entity instances and is set for a
    /// concrete relation instance.
    #[serde(default = "String::new")]
    pub instance_type_name: String,

    /// The name of the inbound entity type.
    pub inbound_type: String,

    /// Textual description of the relation type.
    #[serde(default = "String::new")]
    pub description: String,

    /// The names of the components of the relation type.
    #[serde(default = "Vec::new")]
    pub components: Vec<String>,

    /// The properties which are defined by the relation type.
    #[serde(default = "Vec::new")]
    pub properties: Vec<PropertyType>,

    /// Relation type specific extensions
    #[serde(default = "Vec::new")]
    pub extensions: Vec<Extension>,

    #[serde(skip)]
    pub t: Identifier,
}

impl RelationType {
    #[allow(clippy::too_many_arguments)]
    pub fn new<S: Into<String>>(
        namespace: S,
        outbound_type: S,
        type_name: S,
        inbound_type: S,
        description: S,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> RelationType {
        let namespace = namespace.into();
        let type_name = type_name.into();
        let t = fully_qualified_identifier(namespace.as_str(), type_name.as_str());
        RelationType {
            namespace,
            outbound_type: outbound_type.into(),
            instance_type_name: type_name.clone(),
            type_name,
            inbound_type: inbound_type.into(),
            description: description.into(),
            components,
            properties,
            extensions,
            t,
        }
    }
}

impl TypeContainer for RelationType {
    fn fully_qualified_name(&self) -> String {
        format!("{}__{}", self.namespace, self.type_name)
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
        .unwrap_or_else(|_| Identifier::new(Uuid::new_v5(&NAMESPACE_RELATION_TYPE, fully_qualified_name.as_bytes()).to_string()).unwrap())
}
