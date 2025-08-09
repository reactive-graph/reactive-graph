use std::fmt::Display;
use std::fmt::Formatter;

use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::Component;
use crate::ComponentTypeId;
use crate::EntityType;
use crate::EntityTypeId;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum MatchingInboundOutboundType<T>
where
    T: NamespacedTypeGetter + TypeDefinitionGetter,
{
    /// Concrete type: The inbound or outbound type must be of the correct TypeIdType (either component or entity type) and must be the given namespaced type.
    NamespacedType(T),
    /// Star wildcard: The inbound or outbound type must be of the correct TypeIdType (either component or entity type) but any type (all components or all entity types) is allowed.
    Any,
}

impl<T> Display for MatchingInboundOutboundType<T>
where
    T: NamespacedTypeGetter + TypeDefinitionGetter + ToString,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let namespace_or_matching = match self {
            MatchingInboundOutboundType::NamespacedType(ty) => ty.to_string(),
            MatchingInboundOutboundType::Any => "*".to_string(),
        };
        write!(f, "{namespace_or_matching}")
    }
}

impl<T> TryFrom<&MatchingInboundOutboundType<T>> for NamespacedType
where
    T: NamespacedTypeGetter + TypeDefinitionGetter,
{
    type Error = InboundOutboundTypeConversionError;

    fn try_from(ty: &MatchingInboundOutboundType<T>) -> Result<Self, Self::Error> {
        match ty {
            MatchingInboundOutboundType::NamespacedType(ty) => Ok(ty.namespaced_type()),
            MatchingInboundOutboundType::Any => Err(InboundOutboundTypeConversionError::InboundOutboundIsAWildcard(T::type_id_type())),
        }
    }
}

impl<T> TryFrom<&MatchingInboundOutboundType<T>> for TypeDefinition
where
    T: NamespacedTypeGetter + TypeDefinitionGetter,
{
    type Error = InboundOutboundTypeConversionError;

    fn try_from(ty: &MatchingInboundOutboundType<T>) -> Result<Self, Self::Error> {
        match ty {
            MatchingInboundOutboundType::NamespacedType(ty) => Ok(ty.type_definition()),
            MatchingInboundOutboundType::Any => Err(InboundOutboundTypeConversionError::InboundOutboundIsAWildcard(T::type_id_type())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum InboundOutboundType {
    #[serde(rename = "component")]
    Component(MatchingInboundOutboundType<ComponentTypeId>),
    #[serde(rename = "entity_type")]
    EntityType(MatchingInboundOutboundType<EntityTypeId>),
}

// impl InboundOutboundType {
//     /// Returns true, if the given component type id is equal to the inner component type id.
//     pub fn eq_component(&self, component_ty: &ComponentTypeId) -> bool {
//         match self {
//             InboundOutboundType::Component(ty) => ty.eq(component_ty),
//             _ => false,
//         }
//     }
//
//     /// Returns true, if the given entity type id is equal to the inner entity type id.
//     pub fn eq_entity_type(&self, entity_ty: &EntityTypeId) -> bool {
//         match self {
//             InboundOutboundType::EntityType(ty) => ty.eq(entity_ty),
//             _ => false,
//         }
//     }
// }

impl PartialEq<ComponentTypeId> for InboundOutboundType {
    fn eq(&self, component_ty: &ComponentTypeId) -> bool {
        match self {
            InboundOutboundType::Component(matching_component_ty) => match matching_component_ty {
                MatchingInboundOutboundType::NamespacedType(ty) => ty.eq(component_ty),
                MatchingInboundOutboundType::Any => true,
            },
            _ => false,
        }
    }
}

impl PartialEq<EntityTypeId> for InboundOutboundType {
    fn eq(&self, entity_ty: &EntityTypeId) -> bool {
        match self {
            InboundOutboundType::EntityType(matching_entity_ty) => match matching_entity_ty {
                MatchingInboundOutboundType::NamespacedType(ty) => ty.eq(entity_ty),
                MatchingInboundOutboundType::Any => true,
            },
            _ => false,
        }
    }
}

impl From<&InboundOutboundType> for InboundOutboundType {
    fn from(ty: &InboundOutboundType) -> Self {
        ty.clone()
    }
}

impl From<ComponentTypeId> for InboundOutboundType {
    fn from(ty: ComponentTypeId) -> Self {
        InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ty))
    }
}

impl From<&Component> for InboundOutboundType {
    fn from(component: &Component) -> Self {
        InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(component.ty.clone()))
    }
}

impl TryFrom<InboundOutboundType> for ComponentTypeId {
    type Error = InboundOutboundTypeConversionError;

    fn try_from(ty: InboundOutboundType) -> Result<Self, Self::Error> {
        match ty {
            InboundOutboundType::EntityType(_) => Err(InboundOutboundTypeConversionError::TypeIdTypeMismatch(TypeIdType::EntityType, TypeIdType::Component)),
            InboundOutboundType::Component(MatchingInboundOutboundType::Any) => {
                Err(InboundOutboundTypeConversionError::InboundOutboundIsAWildcard(TypeIdType::Component))
            }
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ty)) => Ok(ty),
        }
    }
}

impl From<EntityTypeId> for InboundOutboundType {
    fn from(ty: EntityTypeId) -> Self {
        InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty))
    }
}

impl From<&EntityType> for InboundOutboundType {
    fn from(entity_type: &EntityType) -> Self {
        InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(entity_type.ty.clone()))
    }
}

#[derive(Debug, Error)]
pub enum InboundOutboundTypeConversionError {
    #[error("The inbound/outbound type is a wildcard which cannot be constructed to a {0}.")]
    InboundOutboundIsAWildcard(TypeIdType),
    #[error("The inbound/outbound type is a {0} which cannot be constructed to a {1}.")]
    TypeIdTypeMismatch(TypeIdType, TypeIdType),
}

impl TryFrom<InboundOutboundType> for EntityTypeId {
    type Error = InboundOutboundTypeConversionError;

    fn try_from(ty: InboundOutboundType) -> Result<Self, Self::Error> {
        match ty {
            InboundOutboundType::Component(_) => Err(InboundOutboundTypeConversionError::TypeIdTypeMismatch(TypeIdType::Component, TypeIdType::EntityType)),
            InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => {
                Err(InboundOutboundTypeConversionError::InboundOutboundIsAWildcard(TypeIdType::EntityType))
            }
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => Ok(ty),
        }
    }
}

// impl NamespacedTypeGetter for InboundOutboundType {
//     fn namespace(&self) -> Namespace {
//         match self {
//             InboundOutboundType::Component(ty) => ty.namespace(),
//             InboundOutboundType::EntityType(ty) => ty.namespace(),
//         }
//     }
//
//     fn path(&self) -> Namespace {
//         match self {
//             InboundOutboundType::Component(ty) => ty.path(),
//             InboundOutboundType::EntityType(ty) => ty.path(),
//         }
//     }
//
//     fn type_name(&self) -> NamespaceSegment {
//         match self {
//             InboundOutboundType::Component(ty) => ty.type_name(),
//             InboundOutboundType::EntityType(ty) => ty.type_name(),
//         }
//     }
// }

// impl TypeDefinitionGetter for InboundOutboundType {
//     fn type_definition(&self) -> TypeDefinition {
//         match self {
//             InboundOutboundType::Component(ty) => ty.type_definition(),
//             InboundOutboundType::EntityType(ty) => ty.type_definition(),
//         }
//     }
// }

// impl Display for InboundOutboundType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let namespace_or_matching = match self {
//             InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ty))
//             | InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => ty.to_string(),
//             // InboundOutboundType::Component(MatchingInboundOutboundType::Any) => {
//             //     "*".to_string()
//             // }
//             // InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(ty)) => {
//             //     ty.to_string()
//             // }
//             InboundOutboundType::Component(MatchingInboundOutboundType::Any) | InboundOutboundType::EntityType(MatchingInboundOutboundType::Any) => {
//                 "*".to_string()
//             } // InboundOutboundType::Component(ty) => {
//             //     match ty {
//             //         MatchingInboundOutboundType::NamespacedType(ty) => ty.to_string(),
//             //         MatchingInboundOutboundType::Any => "*".to_string(),
//             //     }
//             // }
//             // InboundOutboundType::EntityType(ty) => {
//             //     match ty {
//             //         MatchingInboundOutboundType::NamespacedType(ty) => ty.to_string(),
//             //         MatchingInboundOutboundType::Any => "*".to_string(),
//             //     }
//             // }
//         };
//         write!(f, "{namespace_or_matching}")
//     }
// }

impl Display for InboundOutboundType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            InboundOutboundType::Component(ty) => ty.to_string(),
            InboundOutboundType::EntityType(ty) => ty.to_string(),
        };
        write!(f, "{}", &d)
    }
}

impl TryFrom<&InboundOutboundType> for TypeDefinition {
    type Error = InboundOutboundTypeConversionError;

    fn try_from(ty: &InboundOutboundType) -> Result<Self, Self::Error> {
        match ty {
            InboundOutboundType::Component(ty) => TypeDefinition::try_from(ty),
            InboundOutboundType::EntityType(ty) => TypeDefinition::try_from(ty),
        }
    }
}

impl TryFrom<&InboundOutboundType> for NamespacedType {
    type Error = InboundOutboundTypeConversionError;
    fn try_from(ty: &InboundOutboundType) -> Result<Self, Self::Error> {
        match ty {
            InboundOutboundType::Component(ty) => NamespacedType::try_from(ty),
            InboundOutboundType::EntityType(ty) => NamespacedType::try_from(ty),
        }
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for InboundOutboundType {
    fn default_test() -> Self {
        let mut rng = rand::rng();
        let b: bool = rng.random();
        if b {
            InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ComponentTypeId::new(NamespacedType::generate_random())))
        } else {
            InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(EntityTypeId::new(NamespacedType::generate_random())))
        }
    }
}
