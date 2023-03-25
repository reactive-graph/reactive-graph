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

pub enum RelationTypeCardinality {
    /// Exactly one relation instance of a relation type can exist between two entity instances.
    Unique,

    /// Multiple relation instances of a relation type can exist between two entity instances. The
    /// instance_id must be specified.
    Multiple,

    /// Multiple relation instances of a relation type can exist between two entity instances. The
    /// instance_id is generated randomly.
    Random,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RelationTypeId(NamespacedType);

impl RelationTypeId {
    pub fn new(nt: NamespacedType) -> RelationTypeId {
        RelationTypeId(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> RelationTypeId {
        RelationTypeId(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for RelationTypeId {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for RelationTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&RelationTypeId> for RelationTypeId {
    fn from(ty: &RelationTypeId) -> Self {
        ty.clone()
    }
}

impl From<&RelationTypeId> for TypeDefinition {
    fn from(ty: &RelationTypeId) -> Self {
        TypeDefinition::new(TypeIdType::RelationType, ty.0.clone())
    }
}

impl From<&RelationTypeId> for NamespacedType {
    fn from(ty: &RelationTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for RelationTypeId {
    fn from(nt: NamespacedType) -> Self {
        RelationTypeId(nt)
    }
}

impl TryFrom<&TypeDefinition> for RelationTypeId {
    type Error = ();

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::RelationType => Ok(RelationTypeId::new_from_type(type_definition.namespace.clone(), type_definition.type_name.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Identifier> for RelationTypeId {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::RelationType == type_type {
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
            return Ok(RelationTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl TryFrom<&String> for RelationTypeId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::RelationType == type_type {
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
            return Ok(RelationTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for RelationTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[macro_export]
macro_rules! relation_ty {
    (
        $relation_type_id: ident,
        $namespace: ident,
        $relation_type_name_const: ident,
        $relation_type_name: expr
    ) => {
        pub const $relation_type_name_const: &str = $relation_type_name;
        lazy_static::lazy_static! {
            pub static ref $relation_type_id: $crate::RelationTypeId = $crate::RelationTypeId::new_from_type($namespace, $relation_type_name_const);
        }
    };
}
