use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;

#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::RelationTypeId;
use crate::TYPE_ID_TYPE_SEPARATOR;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

/// Type identifier of a relation instance.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct RelationInstanceTypeId {
    /// The type definition of the relation type.
    #[serde(flatten)]
    ty: RelationTypeId,

    /// The instance id.
    instance_id: String,
}

impl RelationInstanceTypeId {
    /// Between two entity instances there can be only one relation instance.
    pub fn new_unique_id<RT: Into<RelationTypeId>>(ty: RT) -> Self {
        Self {
            ty: ty.into(),
            instance_id: String::new(),
        }
    }

    /// Between two entity instances there can be only one relation instance with the same instance
    /// id.
    ///
    /// For example, multiple connectors exists between two entity instances. But only one connector
    /// is allowed between two properties.
    pub fn new_unique_for_instance_id<RT: Into<RelationTypeId>, S: Into<String>>(ty: RT, instance_id: S) -> Self {
        Self {
            ty: ty.into(),
            instance_id: instance_id.into(),
        }
    }

    /// Between two entity instances there can be multiple one relation instances. The instance id
    /// of the relation instance will be generated randomly.
    pub fn new_with_random_instance_id<RT: Into<RelationTypeId>>(ty: RT) -> Self {
        Self {
            ty: ty.into(),
            instance_id: Uuid::new_v4().to_string(),
        }
    }

    /// Between two entity instances there can be only one relation instance.
    pub fn new_from_type_unique_id<S: Into<String>>(namespace: S, type_name: S) -> Self {
        Self::new_unique_id(RelationTypeId::new(NamespacedType::new(namespace, type_name)))
    }

    /// Between two entity instances there can be only one relation instance with the same instance_id.
    pub fn new_from_type_unique_for_instance_id<S: Into<String>>(namespace: S, type_name: S, instance_id: S) -> Self {
        Self::new_unique_for_instance_id(RelationTypeId::new(NamespacedType::new(namespace, type_name)), instance_id)
    }

    /// Between two entity instances there can be multiple one relation instances. The instance id
    /// of the relation instance will be generated randomly.
    pub fn new_from_type_with_random_instance_id<S: Into<String>>(namespace: S, type_name: S) -> Self {
        Self::new_with_random_instance_id(RelationTypeId::new(NamespacedType::new(namespace, type_name)))
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

impl PartialOrd<Self> for RelationInstanceTypeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelationInstanceTypeId {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty.cmp(&other.ty) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.instance_id.cmp(&other.instance_id),
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl From<&RelationInstanceTypeId> for RelationInstanceTypeId {
    fn from(ty: &RelationInstanceTypeId) -> Self {
        ty.clone()
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

impl TryFrom<String> for RelationInstanceTypeId {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        RelationInstanceTypeId::try_from(&s)
    }
}

impl Display for RelationInstanceTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
        // TODO: 2023-09-03
        // if self.instance_id.is_empty() {
        //     write!(f, "{}", &self.type_definition().to_string())
        // } else {
        //     write!(f, "{}{}{}", &self.type_definition().to_string(), &TYPE_ID_TYPE_SEPARATOR, self.instance_id)
        // }
    }
}

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;

#[cfg(any(test, feature = "test"))]
impl DefaultTest for RelationInstanceTypeId {
    fn default_test() -> Self {
        RelationInstanceTypeId::new_with_random_instance_id(NamespacedType::generate_random())
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::RelationInstanceTypeId;
    use crate::RelationTypeId;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn relation_instance_type_id_unique_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let rty = RelationTypeId::new_from_type(&namespace, &type_name);
        let ty = RelationInstanceTypeId::new_unique_id(rty.clone());
        assert_eq!(namespace, ty.namespace());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(nt.type_name, ty.type_name());
        assert_eq!(format!("r__{namespace}__{type_name}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(type_name, type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(type_name, type_definition_3.type_name());

        let ty2 = RelationInstanceTypeId::new_unique_id(rty.clone());
        assert_eq!(ty, ty2);
        assert_eq!(ty.namespace(), ty2.namespace());
        assert_eq!(ty.type_name(), ty2.type_name());
        assert_eq!(ty.instance_id(), ty2.instance_id());
        assert_eq!(ty.to_string(), ty2.to_string());
    }

    #[test]
    fn relation_instance_type_id_unique_for_instance_id_test() {
        let namespace = r_string();
        let type_name = r_string();
        let instance_id = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let rty = RelationTypeId::new_from_type(&namespace, &type_name);
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(rty.clone(), &instance_id);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id), ty.type_name());
        assert_eq!(instance_id, ty.instance_id());
        assert_eq!(rty, ty.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id), type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id), type_definition_3.type_name());

        let instance_id_2 = r_string();
        let ty2 = RelationInstanceTypeId::new_unique_for_instance_id(rty.clone(), &instance_id_2);
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(nt.namespace, ty2.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id_2), ty2.type_name());
        assert_eq!(instance_id_2, ty2.instance_id());
        assert_eq!(rty, ty2.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{instance_id_2}"), format!("{}", ty2));
        assert_ne!(ty, ty2);
        assert_eq!(ty.namespace(), ty2.namespace());
        assert_ne!(ty.type_name(), ty2.type_name());
        assert_eq!(ty.relation_type_id(), ty2.relation_type_id());
        assert_ne!(ty.instance_id(), ty2.instance_id());
        assert_ne!(ty.to_string(), ty2.to_string());
    }

    #[test]
    fn relation_instance_type_id_with_random_instance_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let rty = RelationTypeId::new_from_type(&namespace, &type_name);
        let ty = RelationInstanceTypeId::new_with_random_instance_id(rty.clone());
        assert_eq!(namespace, ty.namespace());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(format!("{}__{}", type_name, ty.instance_id()), ty.type_name());
        assert_eq!(rty, ty.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{}", ty.instance_id()), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::RelationType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(format!("{}__{}", type_name, ty.instance_id()), type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::RelationType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(format!("{}__{}", type_name, ty.instance_id()), type_definition_3.type_name());

        let ty2 = RelationInstanceTypeId::new_with_random_instance_id(rty.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(nt.namespace, ty2.namespace());
        assert_eq!(format!("{}__{}", type_name, ty2.instance_id()), ty2.type_name());
        assert_ne!(ty.instance_id(), ty2.instance_id());
        assert_eq!(rty, ty2.relation_type_id());
        assert_eq!(format!("r__{namespace}__{type_name}__{}", ty2.instance_id()), format!("{}", ty2));
        assert_ne!(ty, ty2);
        assert_eq!(ty.namespace(), ty2.namespace());
        assert_ne!(ty.type_name(), ty2.type_name());
        assert_eq!(ty.relation_type_id(), ty2.relation_type_id());
        assert_ne!(ty.instance_id(), ty2.instance_id());
        assert_ne!(ty.to_string(), ty2.to_string());
    }

    #[test]
    fn relation_instance_type_id_from_string_test() {
        let t1 = String::from("r__ns__ty");
        let ty1 = RelationInstanceTypeId::try_from(&t1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.relation_type_id().type_name());
        assert_eq!("ty", ty1.type_name());
        assert!(ty1.instance_id().is_empty());

        let t2 = String::from("r__ns__ty__instance");
        let ty2 = RelationInstanceTypeId::try_from(&t2).unwrap();
        assert_eq!("ns", ty2.namespace());
        assert_eq!("ty", ty2.relation_type_id().type_name());
        assert_eq!("ty__instance", ty2.type_name());
        assert_eq!("instance", ty2.instance_id());

        let t3 = String::from("r__ns__ty__outbound__inbound");
        let ty3 = RelationInstanceTypeId::try_from(&t3).unwrap();
        assert_eq!("ns", ty3.namespace());
        assert_eq!("ty", ty3.relation_type_id().type_name());
        assert_eq!("ty__outbound__inbound", ty3.type_name());
        assert_eq!("outbound__inbound", ty3.instance_id());

        let t4 = String::from("e__ns__ty");
        let ty4 = RelationInstanceTypeId::try_from(&t4);
        assert!(ty4.is_err());

        let t5 = String::from("r__");
        let ty5 = RelationInstanceTypeId::try_from(&t5);
        assert!(ty5.is_err());

        let t6 = String::from("r__ns");
        let ty6 = RelationInstanceTypeId::try_from(&t6);
        assert!(ty6.is_err());

        let t7 = String::from("r__ns__");
        let ty7 = RelationInstanceTypeId::try_from(&t7);
        assert!(ty7.is_err());
    }

    #[test]
    fn relation_instance_type_id_json_schema() {
        let schema = schema_for!(RelationInstanceTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
