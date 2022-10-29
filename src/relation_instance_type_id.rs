use indradb::Identifier;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use uuid::Uuid;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::RelationTypeId;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

/// Type identifier of a relation instance.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationInstanceTypeId {
    ty: RelationTypeId,
    instance_id: String,
}

impl RelationInstanceTypeId {
    /// Between two entity instances there can be only one relation instance.
    pub fn new_unique_id(ty: RelationTypeId) -> RelationInstanceTypeId {
        RelationInstanceTypeId {
            ty,
            instance_id: String::new(),
        }
    }

    /// Between two entity instances there can be only one relation instance with the same instance
    /// id.
    ///
    /// For example, multiple connectors exists between two entity instances. But only one connector
    /// is allowed between two properties.
    pub fn new_unique_for_instance_id<S: Into<String>>(ty: RelationTypeId, instance_id: S) -> RelationInstanceTypeId {
        RelationInstanceTypeId {
            ty,
            instance_id: instance_id.into(),
        }
    }

    /// Between two entity instances there can be multiple one relation instances. The instance id
    /// of the relation instance will be generated randomly.
    pub fn new_with_random_instance_id(ty: RelationTypeId) -> RelationInstanceTypeId {
        RelationInstanceTypeId {
            ty,
            instance_id: Uuid::new_v4().to_string(),
        }
    }

    /// Between two entity instances there can be only one relation instance.
    pub fn new_from_type_unique_id<S: Into<String>>(namespace: S, type_name: S) -> RelationInstanceTypeId {
        RelationInstanceTypeId::new_unique_id(RelationTypeId::new(NamespacedType::new(namespace, type_name)))
    }

    /// Between two entity instances there can be only one relation instance with the same instance_id.
    pub fn new_from_type_unique_for_instance_id<S: Into<String>>(namespace: S, type_name: S, instance_id: S) -> RelationInstanceTypeId {
        RelationInstanceTypeId::new_unique_for_instance_id(RelationTypeId::new(NamespacedType::new(namespace, type_name)), instance_id)
    }

    /// Between two entity instances there can be multiple one relation instances. The instance id
    /// of the relation instance will be generated randomly.
    pub fn new_from_type_with_random_instance_id<S: Into<String>>(namespace: S, type_name: S) -> RelationInstanceTypeId {
        RelationInstanceTypeId::new_with_random_instance_id(RelationTypeId::new(NamespacedType::new(namespace, type_name)))
    }

    /// Returns true, if the relation instance type id is of the given relation type id.
    pub fn is_a(&self, ty: &RelationTypeId) -> bool {
        &self.ty == ty
    }

    /// Returns the inner relation type id.
    pub fn relation_type_id(&self) -> RelationTypeId {
        self.ty.clone()
    }

    /// Returns the instance id.
    pub fn instance_id(&self) -> String {
        self.instance_id.clone()
    }
}

impl NamespacedTypeGetter for RelationInstanceTypeId {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    /// Returns the full instance type name (relation type name + instance id)
    fn type_name(&self) -> String {
        if !self.instance_id.is_empty() {
            format!("{}{}{}", self.ty.type_name(), &TYPE_ID_TYPE_SEPARATOR, &self.instance_id)
        } else {
            self.ty.type_name()
        }
    }
}

impl TypeDefinitionGetter for RelationInstanceTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }
}

impl From<&RelationInstanceTypeId> for TypeDefinition {
    fn from(ty: &RelationInstanceTypeId) -> Self {
        TypeDefinition::new(TypeIdType::RelationType, ty.into())
    }
}

impl From<&RelationInstanceTypeId> for NamespacedType {
    fn from(ty: &RelationInstanceTypeId) -> Self {
        // Returns the namespaced type with the full instance type name (relation type name + instance id)
        NamespacedType::new(ty.namespace(), ty.type_name())
    }
}

impl TryFrom<&Identifier> for RelationInstanceTypeId {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
        let mut s = s.splitn(4, &TYPE_ID_TYPE_SEPARATOR);
        let type_id_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::RelationType == type_id_type {
            let namespace = s.next().ok_or(())?;
            if namespace.is_empty() {
                return Err(());
            }
            let type_name = s.next().ok_or(())?;
            if type_name.is_empty() {
                return Err(());
            }
            let rty = RelationTypeId::new_from_type(namespace, type_name);
            let ty = match s.next() {
                Some(instance_id) => RelationInstanceTypeId::new_unique_for_instance_id(rty, instance_id),
                None => RelationInstanceTypeId::new_unique_id(rty),
            };
            return Ok(ty);
        }
        Err(())
    }
}

impl TryFrom<&String> for RelationInstanceTypeId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.splitn(4, &TYPE_ID_TYPE_SEPARATOR);
        let type_id_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::RelationType == type_id_type {
            let namespace = s.next().ok_or(())?;
            if namespace.is_empty() {
                return Err(());
            }
            let type_name = s.next().ok_or(())?;
            if type_name.is_empty() {
                return Err(());
            }
            let rty = RelationTypeId::new_from_type(namespace, type_name);
            let ty = match s.next() {
                Some(instance_id) => RelationInstanceTypeId::new_unique_for_instance_id(rty, instance_id),
                None => RelationInstanceTypeId::new_unique_id(rty),
            };
            return Ok(ty);
        }
        Err(())
    }
}

impl Display for RelationInstanceTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}
