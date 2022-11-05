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
pub struct BehaviourTypeId(NamespacedType);

impl BehaviourTypeId {
    pub fn new(nt: NamespacedType) -> BehaviourTypeId {
        BehaviourTypeId(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> BehaviourTypeId {
        BehaviourTypeId(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for BehaviourTypeId {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for BehaviourTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&BehaviourTypeId> for BehaviourTypeId {
    fn from(ty: &BehaviourTypeId) -> Self {
        ty.clone()
    }
}

impl From<&BehaviourTypeId> for TypeDefinition {
    fn from(ty: &BehaviourTypeId) -> Self {
        TypeDefinition::new(TypeIdType::Behaviour, ty.0.clone())
    }
}

impl From<&BehaviourTypeId> for NamespacedType {
    fn from(ty: &BehaviourTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for BehaviourTypeId {
    fn from(nt: NamespacedType) -> Self {
        BehaviourTypeId(nt)
    }
}

impl TryFrom<&TypeDefinition> for BehaviourTypeId {
    type Error = ();

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::Behaviour => Ok(BehaviourTypeId::new_from_type(type_definition.namespace.clone(), type_definition.type_name.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Identifier> for BehaviourTypeId {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::Behaviour == type_type {
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
            return Ok(BehaviourTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl TryFrom<&String> for BehaviourTypeId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::Behaviour == type_type {
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
            return Ok(BehaviourTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for BehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}
