use std::fmt::Display;
use std::fmt::Formatter;

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::Component;
use crate::ComponentTypeId;
use crate::EntityType;
use crate::EntityTypeId;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ComponentOrEntityTypeId {
    #[serde(rename = "component")]
    Component(ComponentTypeId),
    #[serde(rename = "entity_type")]
    EntityType(EntityTypeId),
}

impl ComponentOrEntityTypeId {
    /// Returns true, if the given component type id is equal to the inner component type id.
    pub fn eq_component(&self, component_ty: &ComponentTypeId) -> bool {
        match self {
            ComponentOrEntityTypeId::Component(ty) => ty.eq(component_ty),
            _ => false,
        }
    }

    /// Returns true, if the given entity type id is equal to the inner entity type id.
    pub fn eq_entity_type(&self, entity_ty: &EntityTypeId) -> bool {
        match self {
            ComponentOrEntityTypeId::EntityType(ty) => ty.eq(entity_ty),
            _ => false,
        }
    }
}

impl From<&ComponentOrEntityTypeId> for ComponentOrEntityTypeId {
    fn from(ty: &ComponentOrEntityTypeId) -> Self {
        ty.clone()
    }
}

impl From<ComponentTypeId> for ComponentOrEntityTypeId {
    fn from(ty: ComponentTypeId) -> Self {
        ComponentOrEntityTypeId::Component(ty)
    }
}

impl From<&Component> for ComponentOrEntityTypeId {
    fn from(component: &Component) -> Self {
        ComponentOrEntityTypeId::Component(component.ty.clone())
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

impl From<&EntityType> for ComponentOrEntityTypeId {
    fn from(entity_type: &EntityType) -> Self {
        ComponentOrEntityTypeId::EntityType(entity_type.ty.clone())
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

impl Display for ComponentOrEntityTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            ComponentOrEntityTypeId::Component(ty) => ty.to_string(),
            ComponentOrEntityTypeId::EntityType(ty) => ty.to_string(),
        };
        write!(f, "{}", &d)
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

#[cfg(any(test, feature = "test"))]
impl DefaultTest for ComponentOrEntityTypeId {
    fn default_test() -> Self {
        let mut rng = rand::rng();
        let b: bool = rng.random();
        if b {
            ComponentOrEntityTypeId::Component(NamespacedType::generate_random().into())
        } else {
            ComponentOrEntityTypeId::EntityType(NamespacedType::generate_random().into())
        }
    }
}
