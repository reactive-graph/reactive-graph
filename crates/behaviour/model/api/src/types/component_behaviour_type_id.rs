use std::borrow::Cow;
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
use schemars::json_schema;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::BehaviourTypeId;
use crate::BehaviourTypeIds;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::NamespacedType;

/// The behaviour of a component.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct ComponentBehaviourTypeId {
    /// The component type.
    pub component_ty: ComponentTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}

impl ComponentBehaviourTypeId {
    pub fn new(component_ty: ComponentTypeId, behaviour_ty: BehaviourTypeId) -> Self {
        Self { component_ty, behaviour_ty }
    }

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        let namespaced_type = NamespacedType::new(namespace, type_name);
        Self::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<NamespacedType> for ComponentBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        Self::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<&BehaviourTypeId> for ComponentBehaviourTypeId {
    fn from(behaviour_ty: &BehaviourTypeId) -> Self {
        Self::new(NamespacedType::from(behaviour_ty).into(), behaviour_ty.clone())
    }
}

impl Display for ComponentBehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.component_ty, &self.behaviour_ty)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentBehaviourTypeIds(DashSet<ComponentBehaviourTypeId>);

impl ComponentBehaviourTypeIds {
    pub fn new() -> Self {
        Self(DashSet::new())
    }

    pub fn to_vec(&self) -> Vec<ComponentBehaviourTypeId> {
        let mut tys: Vec<ComponentBehaviourTypeId> = self.0.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }

    pub fn component_behaviour<B: Into<ComponentBehaviourTypeId>>(self, ty: B) -> Self {
        self.insert(ty.into());
        self
    }
}

impl Deref for ComponentBehaviourTypeIds {
    type Target = DashSet<ComponentBehaviourTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ComponentBehaviourTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ComponentBehaviourTypeIds {
    type Item = ComponentBehaviourTypeId;
    type IntoIter = OwningIter<ComponentBehaviourTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for ComponentBehaviourTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for ComponentBehaviourTypeIds {}

impl Hash for ComponentBehaviourTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for ComponentBehaviourTypeIds {
    fn schema_name() -> Cow<'static, str> {
        "ComponentBehaviourTypeIds".into()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = gen.subschema_for::<ComponentBehaviourTypeId>().into();
        json_schema!({
            "type": "array",
            "instance_type": sub_schema,
            "description": "Component Behaviour Type Ids",
        })
    }
}

impl From<Vec<ComponentBehaviourTypeId>> for ComponentBehaviourTypeIds {
    fn from(tys: Vec<ComponentBehaviourTypeId>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<ComponentBehaviourTypeIds> for Vec<ComponentBehaviourTypeId> {
    fn from(tys: ComponentBehaviourTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&ComponentBehaviourTypeIds> for Vec<ComponentBehaviourTypeId> {
    fn from(tys: &ComponentBehaviourTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<ComponentBehaviourTypeId>> for ComponentBehaviourTypeIds {
    fn from(tys: DashSet<ComponentBehaviourTypeId>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<ComponentBehaviourTypeId>> for ComponentBehaviourTypeIds {
    fn from(tys: &DashSet<ComponentBehaviourTypeId>) -> Self {
        Self(tys.clone())
    }
}

impl From<ComponentBehaviourTypeIds> for DashSet<ComponentBehaviourTypeId> {
    fn from(tys: ComponentBehaviourTypeIds) -> Self {
        tys.0
    }
}

impl From<&BehaviourTypeIds> for ComponentBehaviourTypeIds {
    fn from(behaviour_tys: &BehaviourTypeIds) -> Self {
        behaviour_tys.iter().map(|ty| ty.key().into()).collect()
    }
}

impl FromIterator<ComponentBehaviourTypeId> for ComponentBehaviourTypeIds {
    fn from_iter<I: IntoIterator<Item = ComponentBehaviourTypeId>>(iter: I) -> Self {
        let tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

// TODO: Replace this with LazyLock / LazyCell
#[macro_export]
macro_rules! component_behaviour_ty {
    (
        $component_behaviour_type_id: ident,
        $component_type_id: ident,
        $behaviour_type_id: ident
    ) => {
        lazy_static::lazy_static! {
            pub static ref $component_behaviour_type_id: $crate::ComponentBehaviourTypeId = $crate::ComponentBehaviourTypeId::new($component_type_id.clone(), $behaviour_type_id.clone());
        }
    };
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for ComponentBehaviourTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for ComponentBehaviourTypeIds {
    fn default_test() -> Self {
        let tys = Self::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            tys.insert(ComponentBehaviourTypeId::default_test());
        }
        tys
    }
}
