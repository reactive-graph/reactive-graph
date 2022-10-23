use std::fmt::Debug;

use indradb::Identifier;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

/// Separator for the string representation of a type definition.
pub static TYPE_OF_TYPE_SEPARATOR: &str = "__";

/// String representation of the type of component types.
pub const TYPE_OF_TYPE_COMPONENT: &str = "c";

/// String representation of the type of entity types.
pub const TYPE_OF_TYPE_ENTITY_TYPE: &str = "e";

/// String representation of the type of relation types.
pub const TYPE_OF_TYPE_RELATION_TYPE: &str = "r";

/// String representation of the type of flow types.
pub const TYPE_OF_TYPE_FLOW_TYPE: &str = "f";

pub static TYPE_OF_TYPE_NAMESPACE_COMPONENT: Uuid = Uuid::from_u128(0x1ab7c8109d3d13c180f468262fd540d9);
pub static TYPE_OF_TYPE_NAMESPACE_ENTITY_TYPE: Uuid = Uuid::from_u128(0x6ba7c8109dcd11c180b400d04fd530c7);
pub static TYPE_OF_TYPE_NAMESPACE_RELATION_TYPE: Uuid = Uuid::from_u128(0x1ab7c8109dcd11c180b400d01fd530c7);
pub static TYPE_OF_TYPE_NAMESPACE_FLOW_TYPE: Uuid = Uuid::from_u128(0x62b7c5106d3d18c189f468202fd45230);

/// The type of a type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy, Eq)]
pub enum TypeOfType {
    Component,
    EntityType,
    RelationType,
    FlowType,
}

/// Converts the type of a type into the uuid namespace representation.
impl From<TypeOfType> for Uuid {
    fn from(t: TypeOfType) -> Self {
        match t {
            TypeOfType::Component => TYPE_OF_TYPE_NAMESPACE_COMPONENT,
            TypeOfType::EntityType => TYPE_OF_TYPE_NAMESPACE_ENTITY_TYPE,
            TypeOfType::RelationType => TYPE_OF_TYPE_NAMESPACE_RELATION_TYPE,
            TypeOfType::FlowType => TYPE_OF_TYPE_NAMESPACE_FLOW_TYPE,
        }
    }
}

/// Converts the type of a type into a one letter string representation.
impl ToString for TypeOfType {
    fn to_string(&self) -> String {
        match self {
            TypeOfType::Component => TYPE_OF_TYPE_COMPONENT.to_string(),
            TypeOfType::EntityType => TYPE_OF_TYPE_ENTITY_TYPE.to_string(),
            TypeOfType::RelationType => TYPE_OF_TYPE_RELATION_TYPE.to_string(),
            TypeOfType::FlowType => TYPE_OF_TYPE_FLOW_TYPE.to_string(),
        }
    }
}

impl TryFrom<&str> for TypeOfType {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            TYPE_OF_TYPE_COMPONENT => Ok(TypeOfType::Component),
            TYPE_OF_TYPE_ENTITY_TYPE => Ok(TypeOfType::EntityType),
            TYPE_OF_TYPE_RELATION_TYPE => Ok(TypeOfType::RelationType),
            TYPE_OF_TYPE_FLOW_TYPE => Ok(TypeOfType::FlowType),
            _ => Err(()),
        }
    }
}

/// Grants access to the namespace and the type name of a type of types.
pub trait NamespacedTypeGetter {
    /// Returns the namespace of the type.
    fn namespace(&self) -> String;

    /// Returns the name of the type.
    fn type_name(&self) -> String;
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NamespacedType {
    pub namespace: String,
    pub type_name: String,
}

impl NamespacedType {
    pub fn new<S: Into<String>>(namespace: S, type_name: S) -> NamespacedType {
        NamespacedType {
            namespace: namespace.into(),
            type_name: type_name.into(),
        }
    }
}

impl NamespacedTypeGetter for NamespacedType {
    fn namespace(&self) -> String {
        self.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.type_name.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentType(NamespacedType);

impl ComponentType {
    pub fn new(nt: NamespacedType) -> ComponentType {
        ComponentType(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> ComponentType {
        ComponentType(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for ComponentType {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for ComponentType {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&ComponentType> for TypeDefinition {
    fn from(ty: &ComponentType) -> Self {
        TypeDefinition::new(TypeOfType::Component, ty.0.clone())
    }
}

impl From<&ComponentType> for NamespacedType {
    fn from(ty: &ComponentType) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for ComponentType {
    fn from(nt: NamespacedType) -> Self {
        ComponentType(nt)
    }
}

impl TryFrom<&Identifier> for ComponentType {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_OF_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeOfType::Component == type_type {
            let namespace = s.next().ok_or(())?;
            let type_name = s.next().ok_or(())?;
            return Ok(ComponentType(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl TryFrom<&String> for ComponentType {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_OF_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeOfType::Component == type_type {
            let namespace = s.next().ok_or(())?;
            let type_name = s.next().ok_or(())?;
            return Ok(ComponentType(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityTypeType(NamespacedType);

impl EntityTypeType {
    pub fn new(nt: NamespacedType) -> EntityTypeType {
        EntityTypeType(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> EntityTypeType {
        EntityTypeType(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for EntityTypeType {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for EntityTypeType {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&EntityTypeType> for TypeDefinition {
    fn from(ty: &EntityTypeType) -> Self {
        TypeDefinition::new(TypeOfType::EntityType, ty.0.clone())
    }
}

impl From<&EntityTypeType> for NamespacedType {
    fn from(ty: &EntityTypeType) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for EntityTypeType {
    fn from(nt: NamespacedType) -> Self {
        EntityTypeType(nt)
    }
}

impl TryFrom<&Identifier> for EntityTypeType {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_OF_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeOfType::EntityType == type_type {
            let namespace = s.next().ok_or(())?;
            let type_name = s.next().ok_or(())?;
            return Ok(EntityTypeType(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationTypeType(NamespacedType);

impl RelationTypeType {
    pub fn new(nt: NamespacedType) -> RelationTypeType {
        RelationTypeType(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> RelationTypeType {
        RelationTypeType(NamespacedType::new(namespace, type_name))
    }

    /// Returns true, if the type name starts with the given relation type name.
    pub fn starts_with(&self, ty: &RelationTypeType) -> bool {
        self.0.namespace == ty.0.namespace && self.0.type_name.starts_with(&ty.0.type_name)
    }
}

impl NamespacedTypeGetter for RelationTypeType {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for RelationTypeType {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&RelationTypeType> for TypeDefinition {
    fn from(ty: &RelationTypeType) -> Self {
        TypeDefinition::new(TypeOfType::RelationType, ty.0.clone())
    }
}

impl From<&RelationTypeType> for NamespacedType {
    fn from(ty: &RelationTypeType) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for RelationTypeType {
    fn from(nt: NamespacedType) -> Self {
        RelationTypeType(nt)
    }
}

impl TryFrom<&Identifier> for RelationTypeType {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_OF_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeOfType::RelationType == type_type {
            let namespace = s.next().ok_or(())?;
            let type_name = s.next().ok_or(())?;
            return Ok(RelationTypeType(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FlowTypeType(NamespacedType);

impl FlowTypeType {
    pub fn new(nt: NamespacedType) -> FlowTypeType {
        FlowTypeType(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> FlowTypeType {
        FlowTypeType(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for FlowTypeType {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for FlowTypeType {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&FlowTypeType> for TypeDefinition {
    fn from(ty: &FlowTypeType) -> Self {
        TypeDefinition::new(TypeOfType::FlowType, ty.0.clone())
    }
}

impl From<&FlowTypeType> for NamespacedType {
    fn from(ty: &FlowTypeType) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for FlowTypeType {
    fn from(nt: NamespacedType) -> Self {
        FlowTypeType(nt)
    }
}

impl TryFrom<&Identifier> for FlowTypeType {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_OF_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeOfType::FlowType == type_type {
            let namespace = s.next().ok_or(())?;
            let type_name = s.next().ok_or(())?;
            return Ok(FlowTypeType(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

/// Definition of a type with the type of the type, the namespace and the name of the type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TypeDefinition {
    pub type_type: TypeOfType,
    pub namespace: String,
    pub type_name: String,
}

impl TypeDefinition {
    /// Constructs a new type definition from the given type of types and the given namespaced type.
    pub fn new(type_type: TypeOfType, nt: NamespacedType) -> TypeDefinition {
        TypeDefinition {
            type_type,
            namespace: nt.namespace,
            type_name: nt.type_name,
        }
    }

    /// Constructs a type definition from the given type of types, the given namespace and type name.
    pub fn new_from_type<S: Into<String>>(type_type: TypeOfType, namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition {
            type_type,
            namespace: namespace.into(),
            type_name: type_name.into(),
        }
    }

    /// Constructs a type definition for a component.
    pub fn component<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeOfType::Component, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a entity type.
    pub fn entity_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeOfType::EntityType, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a relation type.
    pub fn relation_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeOfType::RelationType, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a flow type.
    pub fn flow_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeOfType::FlowType, NamespacedType::new(namespace, type_name))
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
            self.type_type.to_string(),
            &TYPE_OF_TYPE_SEPARATOR,
            &self.namespace,
            &TYPE_OF_TYPE_SEPARATOR,
            &self.type_name
        )
    }
}

/// Returns the type of the type.
impl From<&TypeDefinition> for TypeOfType {
    fn from(type_definition: &TypeDefinition) -> Self {
        type_definition.type_type
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
            .unwrap_or_else(|_| Identifier::new(Uuid::new_v5(&type_definition.type_type.into(), fully_qualified_name.as_bytes()).to_string()).unwrap())
    }
}

impl TryFrom<&Identifier> for TypeDefinition {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_OF_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        let namespace = s.next().ok_or(())?;
        let type_name = s.next().ok_or(())?;
        let nt = NamespacedType::new(namespace, type_name);
        Ok(TypeDefinition::new(type_type, nt))
    }
}
