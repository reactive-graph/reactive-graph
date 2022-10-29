use std::fmt::Display;
use std::fmt::Formatter;

use indradb::Identifier;
use serde::Deserialize;
use serde::Serialize;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityTypeId(NamespacedType);

impl EntityTypeId {
    pub fn new(nt: NamespacedType) -> EntityTypeId {
        EntityTypeId(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> EntityTypeId {
        EntityTypeId(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for EntityTypeId {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for EntityTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&EntityTypeId> for TypeDefinition {
    fn from(ty: &EntityTypeId) -> Self {
        TypeDefinition::new(TypeIdType::EntityType, ty.0.clone())
    }
}

impl From<&EntityTypeId> for NamespacedType {
    fn from(ty: &EntityTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for EntityTypeId {
    fn from(nt: NamespacedType) -> Self {
        EntityTypeId(nt)
    }
}

impl TryFrom<&Identifier> for EntityTypeId {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::EntityType == type_type {
            let namespace = s.next().ok_or(())?;
            if namespace.is_empty() {
                return Err(());
            }
            let type_name = s.next().ok_or(())?;
            if type_name.is_empty() {
                return Err(());
            }
            if s.next().is_some() {
                return Err(());
            }
            return Ok(EntityTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl TryFrom<&String> for EntityTypeId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::EntityType == type_type {
            let namespace = s.next().ok_or(())?;
            if namespace.is_empty() {
                return Err(());
            }
            let type_name = s.next().ok_or(())?;
            if type_name.is_empty() {
                return Err(());
            }
            if s.next().is_some() {
                return Err(());
            }
            return Ok(EntityTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for EntityTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}
