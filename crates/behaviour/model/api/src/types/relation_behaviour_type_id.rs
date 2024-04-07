use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::iter_set::OwningIter;
use dashmap::DashSet;
use schemars::gen::SchemaGenerator;
use schemars::schema::ArrayValidation;
use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::BehaviourTypeId;
use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::RelationTypeId;

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand_derive2::RandGen;

/// The behaviour of a relation type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct RelationBehaviourTypeId {
    /// The relation type.
    pub relation_ty: RelationTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}

impl RelationBehaviourTypeId {
    pub fn new(relation_ty: RelationTypeId, behaviour_ty: BehaviourTypeId) -> Self {
        Self { relation_ty, behaviour_ty }
    }

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        let namespaced_type = NamespacedType::new(namespace, type_name);
        Self::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<NamespacedType> for RelationBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        Self::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<&BehaviourTypeId> for RelationBehaviourTypeId {
    fn from(behaviour_ty: &BehaviourTypeId) -> Self {
        Self::new(NamespacedType::from(behaviour_ty).into(), behaviour_ty.clone())
    }
}

impl Display for RelationBehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.relation_ty, &self.behaviour_ty)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RelationBehaviourTypeIds(DashSet<RelationBehaviourTypeId>);

impl RelationBehaviourTypeIds {
    pub fn new() -> Self {
        Self(DashSet::new())
    }

    pub fn to_vec(&self) -> Vec<RelationBehaviourTypeId> {
        let mut tys: Vec<RelationBehaviourTypeId> = self.0.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }

    pub fn component_behaviour<B: Into<RelationBehaviourTypeId>>(self, ty: B) -> Self {
        self.insert(ty.into());
        self
    }
}

impl Deref for RelationBehaviourTypeIds {
    type Target = DashSet<RelationBehaviourTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RelationBehaviourTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for RelationBehaviourTypeIds {
    type Item = RelationBehaviourTypeId;
    type IntoIter = OwningIter<RelationBehaviourTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for RelationBehaviourTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for RelationBehaviourTypeIds {}

impl Hash for RelationBehaviourTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for RelationBehaviourTypeIds {
    fn schema_name() -> String {
        "RelationBehaviourTypeIds".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<RelationBehaviourTypeId>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
    }
}

impl From<Vec<RelationBehaviourTypeId>> for RelationBehaviourTypeIds {
    fn from(tys: Vec<RelationBehaviourTypeId>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<RelationBehaviourTypeIds> for Vec<RelationBehaviourTypeId> {
    fn from(tys: RelationBehaviourTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&RelationBehaviourTypeIds> for Vec<RelationBehaviourTypeId> {
    fn from(tys: &RelationBehaviourTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<RelationBehaviourTypeId>> for RelationBehaviourTypeIds {
    fn from(tys: DashSet<RelationBehaviourTypeId>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<RelationBehaviourTypeId>> for RelationBehaviourTypeIds {
    fn from(tys: &DashSet<RelationBehaviourTypeId>) -> Self {
        Self(tys.clone())
    }
}

impl From<RelationBehaviourTypeIds> for DashSet<RelationBehaviourTypeId> {
    fn from(tys: RelationBehaviourTypeIds) -> Self {
        tys.0
    }
}

impl FromIterator<RelationBehaviourTypeId> for RelationBehaviourTypeIds {
    fn from_iter<I: IntoIterator<Item = RelationBehaviourTypeId>>(iter: I) -> Self {
        let tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

// TODO: Replace this with LazyLock / LazyCell
#[macro_export]
macro_rules! relation_behaviour_ty {
    (
        $relation_behaviour_type_id: ident,
        $relation_type_id: ident,
        $behaviour_type_id: ident
    ) => {
        lazy_static::lazy_static! {
            pub static ref $relation_behaviour_type_id: $crate::RelationBehaviourTypeId = $crate::RelationBehaviourTypeId::new($relation_type_id.clone(), $behaviour_type_id.clone());
        }
    };
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for RelationBehaviourTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for RelationBehaviourTypeIds {
    fn default_test() -> Self {
        let tys = Self::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            tys.insert(RelationBehaviourTypeId::default_test());
        }
        tys
    }
}
