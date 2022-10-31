use serde::Deserialize;
use serde::Serialize;

use crate::ComponentTypeId;
use crate::EntityTypeId;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ComponentOrEntityTypeId {
    #[serde(rename = "component")]
    Component(ComponentTypeId),
    #[serde(rename = "entity_type")]
    EntityType(EntityTypeId),
}

impl From<ComponentTypeId> for ComponentOrEntityTypeId {
    fn from(ty: ComponentTypeId) -> Self {
        ComponentOrEntityTypeId::Component(ty)
    }
}

impl TryFrom<ComponentOrEntityTypeId> for ComponentTypeId {
    type Error = ();

    fn try_from(ty: ComponentOrEntityTypeId) -> Result<Self, Self::Error> {
        match ty {
            ComponentOrEntityTypeId::Component(ty) => Ok(ty),
            ComponentOrEntityTypeId::EntityType(_) => Err(()),
        }
    }
}

impl From<EntityTypeId> for ComponentOrEntityTypeId {
    fn from(ty: EntityTypeId) -> Self {
        ComponentOrEntityTypeId::EntityType(ty)
    }
}

impl TryFrom<ComponentOrEntityTypeId> for EntityTypeId {
    type Error = ();

    fn try_from(ty: ComponentOrEntityTypeId) -> Result<Self, Self::Error> {
        match ty {
            ComponentOrEntityTypeId::Component(_) => Err(()),
            ComponentOrEntityTypeId::EntityType(ty) => Ok(ty),
        }
    }
}

impl NamespacedTypeGetter for ComponentOrEntityTypeId {
    fn namespace(&self) -> String {
        match self {
            ComponentOrEntityTypeId::Component(ty) => ty.namespace(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.namespace(),
        }
    }

    fn type_name(&self) -> String {
        match self {
            ComponentOrEntityTypeId::Component(ty) => ty.type_name(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.type_name(),
        }
    }
}

impl TypeDefinitionGetter for ComponentOrEntityTypeId {
    fn type_definition(&self) -> TypeDefinition {
        match self {
            ComponentOrEntityTypeId::Component(ty) => ty.type_definition(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.type_definition(),
        }
    }
}

impl From<&ComponentOrEntityTypeId> for TypeDefinition {
    fn from(ty: &ComponentOrEntityTypeId) -> Self {
        match ty {
            ComponentOrEntityTypeId::Component(ty) => ty.type_definition(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.type_definition(),
        }
    }
}

impl From<&ComponentOrEntityTypeId> for NamespacedType {
    fn from(ty: &ComponentOrEntityTypeId) -> Self {
        match ty {
            ComponentOrEntityTypeId::Component(ty) => ty.into(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.into(),
        }
    }
}
