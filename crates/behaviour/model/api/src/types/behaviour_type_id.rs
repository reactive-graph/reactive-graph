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
use reactive_graph_graph::prelude::*;
use reactive_graph_graph::NamespacedTypeIds;
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

use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeIdType;
use reactive_graph_graph::TYPE_ID_TYPE_SEPARATOR;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct BehaviourTypeId(NamespacedType);

impl BehaviourTypeId {
    pub fn new(nt: NamespacedType) -> Self {
        Self(nt)
    }

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        Self(NamespacedType::new(namespace, type_name))
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
            return Ok(Self(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for BehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BehaviourTypeIds(DashSet<BehaviourTypeId>);

impl BehaviourTypeIds {
    pub fn new() -> Self {
        NamespacedTypeIdContainer::new()
    }

    pub fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
        NamespacedTypeIdContainer::with_namespace(namespace)
    }

    pub fn behaviour<TypeId: Into<BehaviourTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }
}

impl NamespacedTypeIdContainer for BehaviourTypeIds {
    type TypeId = BehaviourTypeId;
    type TypeIds = Self;

    fn new() -> Self {
        Self(DashSet::new())
    }

    fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
        NamespacedTypeIds::new(namespace)
    }
}

impl Deref for BehaviourTypeIds {
    type Target = DashSet<BehaviourTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BehaviourTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for BehaviourTypeIds {
    type Item = BehaviourTypeId;
    type IntoIter = OwningIter<BehaviourTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for BehaviourTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for BehaviourTypeIds {}

impl Hash for BehaviourTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for BehaviourTypeIds {
    fn schema_name() -> String {
        "ComponentTypeIds".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<BehaviourTypeId>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl From<Vec<BehaviourTypeId>> for BehaviourTypeIds {
    fn from(tys: Vec<BehaviourTypeId>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<BehaviourTypeIds> for Vec<BehaviourTypeId> {
    fn from(tys: BehaviourTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&BehaviourTypeIds> for Vec<BehaviourTypeId> {
    fn from(tys: &BehaviourTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<BehaviourTypeId>> for BehaviourTypeIds {
    fn from(tys: DashSet<BehaviourTypeId>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<BehaviourTypeId>> for BehaviourTypeIds {
    fn from(tys: &DashSet<BehaviourTypeId>) -> Self {
        Self(tys.clone())
    }
}

impl From<BehaviourTypeIds> for DashSet<BehaviourTypeId> {
    fn from(tys: BehaviourTypeIds) -> Self {
        tys.0
    }
}

impl From<NamespacedTypeIds<BehaviourTypeIds>> for BehaviourTypeIds {
    fn from(tys: NamespacedTypeIds<Self>) -> Self {
        tys.deref().clone()
    }
}

impl FromIterator<BehaviourTypeId> for BehaviourTypeIds {
    fn from_iter<I: IntoIterator<Item = BehaviourTypeId>>(iter: I) -> Self {
        let tys = BehaviourTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

// pub struct NamespacedTypeIds<T> {
//     namespace: String,
//     tys: T,
// }
//
// impl<T> NamespacedTypeIds<T> {
//     pub fn new<N: Into<String>>(namespace: N, tys: T) -> Self {
//         Self { namespace, tys }
//     }
//
//     pub fn ty<S: Into<String>>(self, type_name: S) -> Self {
//         self.insert(NamespacedType::new(self.namespace.clone(), type_name.into()).into());
//         self
//     }
// }
//
// impl<T> Deref for NamespacedTypeIds<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.tys
//     }
// }
//
// impl<T> From<NamespacedTypeIds<T>> for T {
//     fn from(tys: NamespacedTypeIds<T>) -> Self {
//         tys.tys
//     }
// }

#[macro_export]
macro_rules! behaviour_ty {
    (
        $behaviour_type_id: ident,
        $namespace: ident,
        $behaviour_name_const: ident,
        $behaviour_name: expr
    ) => {
        pub const $behaviour_name_const: &str = $behaviour_name;
        lazy_static::lazy_static! {
            pub static ref $behaviour_type_id: $crate::BehaviourTypeId = $crate::BehaviourTypeId::new_from_type($namespace, $behaviour_name_const);
        }
    };
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for BehaviourTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for BehaviourTypeIds {
    fn default_test() -> Self {
        let tys = Self::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            tys.insert(BehaviourTypeId::default_test());
        }
        tys
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::BehaviourTypeId;
    use reactive_graph_graph::NamespacedType;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::TypeDefinition;
    use reactive_graph_graph::TypeDefinitionGetter;
    use reactive_graph_graph::TypeIdType;
    use reactive_graph_test_utils::r_string;

    #[test]
    fn behaviour_type_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty = BehaviourTypeId::new_from_type(&namespace, &type_name);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(type_name, ty.type_name());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(nt.type_name, ty.type_name());
        assert_eq!(format!("b__{namespace}__{type_name}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::Behaviour, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(type_name, type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::Behaviour, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(type_name, type_definition_3.type_name());
    }

    #[test]
    fn behaviour_type_id_new_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty2 = BehaviourTypeId::new(nt.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(type_name, ty2.type_name());

        let nt2 = NamespacedType::from(&ty2);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn behaviour_type_id_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty1 = BehaviourTypeId::from(nt);
        assert_eq!(namespace, ty1.namespace());
        assert_eq!(type_name, ty1.type_name());
    }

    #[test]
    fn behaviour_type_id_from_string_test() {
        let s1 = String::from("b__ns__ty");
        let ty1 = BehaviourTypeId::try_from(&s1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.type_name());

        let s2 = String::from("e__ns__ty");
        let ty2 = BehaviourTypeId::try_from(&s2);
        assert!(ty2.is_err());

        let s3 = String::from("b__");
        let ty3 = BehaviourTypeId::try_from(&s3);
        assert!(ty3.is_err());

        let s4 = String::from("b__ns");
        let ty4 = BehaviourTypeId::try_from(&s4);
        assert!(ty4.is_err());

        let s5 = String::from("b__ns__");
        let ty5 = BehaviourTypeId::try_from(&s5);
        assert!(ty5.is_err());

        let s6 = String::from("b__ns__ty__");
        let ty6 = BehaviourTypeId::try_from(&s6);
        assert!(ty6.is_err());

        let s7 = String::from("b__ns__ty__xx");
        let ty7 = BehaviourTypeId::try_from(&s7);
        assert!(ty7.is_err());
    }

    #[test]
    fn behaviour_type_id_json_schema() {
        let schema = schema_for!(BehaviourTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
