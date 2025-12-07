use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NAMESPACE_SEPARATOR;
use reactive_graph_graph::NamespacedType;

use crate::BehaviourTypeId;

#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use reactive_graph_graph::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use reactive_graph_graph::RandomNamespacedTypeId;
#[cfg(any(test, feature = "test"))]
use reactive_graph_graph::RandomNamespacedTypeIds;

/// The behaviour of an entity type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct EntityBehaviourTypeId {
    /// The entity type.
    pub entity_ty: EntityTypeId,

    /// The behaviour type.
    pub behaviour_ty: BehaviourTypeId,
}

impl EntityBehaviourTypeId {
    pub fn new(entity_ty: EntityTypeId, behaviour_ty: BehaviourTypeId) -> Self {
        Self { entity_ty, behaviour_ty }
    }
}

impl From<NamespacedType> for EntityBehaviourTypeId {
    fn from(namespaced_type: NamespacedType) -> Self {
        Self::new(namespaced_type.clone().into(), namespaced_type.into())
    }
}

impl From<&BehaviourTypeId> for EntityBehaviourTypeId {
    fn from(behaviour_ty: &BehaviourTypeId) -> Self {
        Self::new(NamespacedType::from(behaviour_ty).into(), behaviour_ty.clone())
    }
}

impl From<&EntityTypeId> for EntityBehaviourTypeId {
    fn from(entity_ty: &EntityTypeId) -> Self {
        Self::new(entity_ty.clone(), NamespacedType::from(entity_ty).into())
    }
}

impl Display for EntityBehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", &self.entity_ty, NAMESPACE_SEPARATOR, &self.behaviour_ty)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntityBehaviourTypeIds(DashSet<EntityBehaviourTypeId>);

impl EntityBehaviourTypeIds {
    pub fn new() -> Self {
        Self(DashSet::new())
    }

    pub fn to_vec(&self) -> Vec<EntityBehaviourTypeId> {
        let mut tys: Vec<EntityBehaviourTypeId> = self.0.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }

    pub fn component_behaviour<B: Into<EntityBehaviourTypeId>>(self, ty: B) -> Self {
        self.insert(ty.into());
        self
    }
}

impl Deref for EntityBehaviourTypeIds {
    type Target = DashSet<EntityBehaviourTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityBehaviourTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for EntityBehaviourTypeIds {
    type Item = EntityBehaviourTypeId;
    type IntoIter = OwningIter<EntityBehaviourTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for EntityBehaviourTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for EntityBehaviourTypeIds {}

impl Hash for EntityBehaviourTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for EntityBehaviourTypeIds {
    fn schema_name() -> Cow<'static, str> {
        "EntityBehaviourTypeIds".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<EntityBehaviourTypeId>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
        })
    }
}

impl From<Vec<EntityBehaviourTypeId>> for EntityBehaviourTypeIds {
    fn from(tys: Vec<EntityBehaviourTypeId>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<EntityBehaviourTypeIds> for Vec<EntityBehaviourTypeId> {
    fn from(tys: EntityBehaviourTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&EntityBehaviourTypeIds> for Vec<EntityBehaviourTypeId> {
    fn from(tys: &EntityBehaviourTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<EntityBehaviourTypeId>> for EntityBehaviourTypeIds {
    fn from(tys: DashSet<EntityBehaviourTypeId>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<EntityBehaviourTypeId>> for EntityBehaviourTypeIds {
    fn from(tys: &DashSet<EntityBehaviourTypeId>) -> Self {
        Self(tys.clone())
    }
}

impl From<EntityBehaviourTypeIds> for DashSet<EntityBehaviourTypeId> {
    fn from(tys: EntityBehaviourTypeIds) -> Self {
        tys.0
    }
}

impl FromIterator<EntityBehaviourTypeId> for EntityBehaviourTypeIds {
    fn from_iter<I: IntoIterator<Item = EntityBehaviourTypeId>>(iter: I) -> Self {
        let tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

#[macro_export]
macro_rules! entity_behaviour_ty {
    (
        $entity_behaviour_type_id: ident,
        $entity_type_id: ident,
        $behaviour_type_id: ident
    ) => {
        pub static $entity_behaviour_type_id: std::sync::LazyLock<$crate::EntityBehaviourTypeId> =
            std::sync::LazyLock::new(|| $crate::EntityBehaviourTypeId::new($entity_type_id.clone(), $behaviour_type_id.clone()));
    };
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedTypeId for EntityBehaviourTypeId {
    type Error = NamespacedTypeError;

    fn random_type_id() -> Result<Self, NamespacedTypeError> {
        Ok(Self::new(EntityTypeId::random_type_id()?, BehaviourTypeId::random_type_id()?))
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedTypeIds for EntityBehaviourTypeIds {
    type Error = NamespacedTypeError;

    fn random_type_ids() -> Result<Self, NamespacedTypeError> {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            tys.insert(EntityBehaviourTypeId::random_type_id()?);
        }
        Ok(tys)
    }
}
