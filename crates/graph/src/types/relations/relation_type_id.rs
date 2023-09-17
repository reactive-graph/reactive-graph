use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::iter_set::OwningIter;
use dashmap::DashSet;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand_derive2::RandGen;
use schemars::gen::SchemaGenerator;
use schemars::schema::ArrayValidation;
use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeIdContainer;
use crate::NamespacedTypeIds;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct RelationTypeId(NamespacedType);

impl RelationTypeId {
    pub fn new(nt: NamespacedType) -> Self {
        Self(nt)
    }

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        Self(NamespacedType::new(namespace, type_name))
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

impl<N: Into<String>, T: Into<String>> From<(N, T)> for RelationTypeId {
    fn from(ty: (N, T)) -> Self {
        Self(NamespacedType::from(ty))
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
            return Ok(Self(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for RelationTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RelationTypeIds(DashSet<RelationTypeId>);

impl RelationTypeIds {
    pub fn new() -> Self {
        NamespacedTypeIdContainer::new()
    }

    pub fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
        <Self as NamespacedTypeIdContainer>::with_namespace(namespace)
    }

    pub fn relation_ty<TypeId: Into<RelationTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }
}

impl NamespacedTypeIdContainer for RelationTypeIds {
    type TypeId = RelationTypeId;
    type TypeIds = Self;

    fn new() -> Self {
        Self(DashSet::new())
    }

    fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
        NamespacedTypeIds::new(namespace)
    }
}

impl Deref for RelationTypeIds {
    type Target = DashSet<RelationTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RelationTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for RelationTypeIds {
    type Item = RelationTypeId;
    type IntoIter = OwningIter<RelationTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for RelationTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for RelationTypeIds {}

impl Hash for RelationTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for RelationTypeIds {
    fn schema_name() -> String {
        "RelationTypeIds".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<RelationTypeId>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl From<Vec<RelationTypeId>> for RelationTypeIds {
    fn from(tys: Vec<RelationTypeId>) -> Self {
        RelationTypeIds(tys.into_iter().collect())
    }
}

impl From<RelationTypeIds> for Vec<RelationTypeId> {
    fn from(tys: RelationTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&RelationTypeIds> for Vec<RelationTypeId> {
    fn from(tys: &RelationTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<RelationTypeId>> for RelationTypeIds {
    fn from(tys: DashSet<RelationTypeId>) -> Self {
        RelationTypeIds(tys)
    }
}

impl From<&DashSet<RelationTypeId>> for RelationTypeIds {
    fn from(tys: &DashSet<RelationTypeId>) -> Self {
        RelationTypeIds(tys.clone())
    }
}

impl From<RelationTypeIds> for DashSet<RelationTypeId> {
    fn from(tys: RelationTypeIds) -> Self {
        tys.0
    }
}

impl From<NamespacedTypeIds<RelationTypeIds>> for RelationTypeIds {
    fn from(tys: NamespacedTypeIds<Self>) -> Self {
        tys.deref().clone()
    }
}

impl FromIterator<RelationTypeId> for RelationTypeIds {
    fn from_iter<I: IntoIterator<Item = RelationTypeId>>(iter: I) -> Self {
        let tys = RelationTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
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

#[cfg(any(test, feature = "test"))]
impl DefaultTest for RelationTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for RelationTypeIds {
    fn default_test() -> Self {
        let tys = RelationTypeIds::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            tys.insert(RelationTypeId::default_test());
        }
        tys
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::test_utils::r_string;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::RelationTypeId;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;

    #[test]
    fn relation_type_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty = RelationTypeId::new_from_type(&namespace, &type_name);
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
    }

    #[test]
    fn relation_type_id_new_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty2 = RelationTypeId::new(nt.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(type_name, ty2.type_name());

        let nt2 = NamespacedType::from(&ty2);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn relation_type_id_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty1 = RelationTypeId::from(nt);
        assert_eq!(namespace, ty1.namespace());
        assert_eq!(type_name, ty1.type_name());
    }

    #[test]
    fn relation_type_id_from_string_test() {
        let s1 = String::from("r__ns__ty");
        let ty1 = RelationTypeId::try_from(&s1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.type_name());

        let s2 = String::from("e__ns__ty");
        let ty2 = RelationTypeId::try_from(&s2);
        assert!(ty2.is_err());

        let s3 = String::from("r__");
        let ty3 = RelationTypeId::try_from(&s3);
        assert!(ty3.is_err());

        let s4 = String::from("r__ns");
        let ty4 = RelationTypeId::try_from(&s4);
        assert!(ty4.is_err());

        let s5 = String::from("r__ns__");
        let ty5 = RelationTypeId::try_from(&s5);
        assert!(ty5.is_err());

        let s6 = String::from("r__ns__ty__");
        let ty6 = RelationTypeId::try_from(&s6);
        assert!(ty6.is_err());

        let s7 = String::from("r__ns__ty__xx");
        let ty7 = RelationTypeId::try_from(&s7);
        assert!(ty7.is_err());
    }

    #[test]
    fn relation_type_id_json_schema() {
        let schema = schema_for!(RelationTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
