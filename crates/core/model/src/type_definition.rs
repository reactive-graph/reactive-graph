use indradb::Identifier;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

/// Definition of a type with the type of the type, the namespace and the name of the type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TypeDefinition {
    pub type_id_type: TypeIdType,
    pub namespace: String,
    pub type_name: String,
}

impl TypeDefinition {
    /// Constructs a new type definition from the given type of types and the given namespaced type.
    pub fn new(type_type: TypeIdType, nt: NamespacedType) -> TypeDefinition {
        TypeDefinition {
            type_id_type: type_type,
            namespace: nt.namespace,
            type_name: nt.type_name,
        }
    }

    /// Constructs a type definition from the given type of types, the given namespace and type name.
    pub fn new_from_type<S: Into<String>>(type_type: TypeIdType, namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition {
            type_id_type: type_type,
            namespace: namespace.into(),
            type_name: type_name.into(),
        }
    }

    /// Constructs a type definition for a component.
    pub fn component<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::Component, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a entity type.
    pub fn entity_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::EntityType, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a relation type.
    pub fn relation_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::RelationType, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a flow type.
    pub fn flow_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::FlowType, NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for TypeDefinition {
    fn namespace(&self) -> String {
        self.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.type_name.clone()
    }
}

/// Returns the fully qualified type name.
impl ToString for TypeDefinition {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.type_id_type.to_string(),
            &TYPE_ID_TYPE_SEPARATOR,
            &self.namespace,
            &TYPE_ID_TYPE_SEPARATOR,
            &self.type_name
        )
    }
}

/// Returns the type of the type.
impl From<&TypeDefinition> for TypeIdType {
    fn from(type_definition: &TypeDefinition) -> Self {
        type_definition.type_id_type
    }
}

/// Returns the type of the type.
impl From<&TypeDefinition> for NamespacedType {
    fn from(type_definition: &TypeDefinition) -> Self {
        NamespacedType {
            namespace: type_definition.namespace.clone(),
            type_name: type_definition.type_name.clone(),
        }
    }
}

/// Safely constructs a type identifier from the type definition.
///
/// Fallback: generate a UUID v5 based on the type type namespace and the type name. The generated
/// type identifier is stable for the type type, namespace and name.
impl From<&TypeDefinition> for Identifier {
    fn from(type_definition: &TypeDefinition) -> Self {
        let fully_qualified_name = type_definition.to_string();
        Identifier::new(fully_qualified_name.as_str())
            .unwrap_or_else(|_| Identifier::new(Uuid::new_v5(&type_definition.type_id_type.into(), fully_qualified_name.as_bytes()).to_string()).unwrap())
    }
}

impl TryFrom<&Identifier> for TypeDefinition {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        let namespace = s.next().ok_or(())?;
        let type_name = s.next().ok_or(())?;
        let nt = NamespacedType::new(namespace, type_name);
        Ok(TypeDefinition::new(type_type, nt))
    }
}

/// Grants access to the type definition of a type of types.
pub trait TypeDefinitionGetter {
    /// Returns the type definition of the type.
    fn type_definition(&self) -> TypeDefinition;

    /// Returns the type identifier of the type.
    fn type_id(&self) -> Identifier {
        (&self.type_definition()).into()
    }
}
