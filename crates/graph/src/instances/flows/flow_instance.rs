use crate::instances::named::NamedInstanceContainer;
use crate::EntityInstance;
use crate::EntityInstances;
#[cfg(any(test, feature = "test"))]
use crate::EntityType;
use crate::EntityTypeId;
use crate::NamespacedTypeGetter;
use crate::RelationInstances;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use dashmap::iter::OwningIter;
use dashmap::DashMap;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use reactive_graph_test_utils::DefaultFrom;
use schemars::json_schema;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;
use typed_builder::TypedBuilder;
use uuid::Uuid;

/// A flow instance is a container for entity instances and relation instances.
///
/// A flow instance is strictly associated with a wrapper entity instance. The properties
/// of the wrapper entity instance are the properties of the flow.
///
/// Additionally, flows can be nested -  from the perspective of the outer flow
/// the inner flow acts like an entity instance. The wrapper entity instance of
/// the inner flow is the interface which can be accessed by the outer flow.
///
/// Entity instances and relation instances can be shared with multiple flows.
///
/// It's even possible to connect entity instances from different flows with relation
/// instances.
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[schemars(deny_unknown_fields)]
pub struct FlowInstance {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The type definition of the entity type of the wrapper entity instance.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub ty: EntityTypeId,

    /// The name of the flow instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub name: String,

    /// Textual description of the flow instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The entity instances which are contained in this flow instance.
    ///
    /// It can't have a default because the wrapper entity instance must be
    /// present in the list of entities.
    #[serde(default = "EntityInstances::new", alias = "entities")]
    #[builder(default, setter(into))]
    pub entity_instances: EntityInstances,

    /// The relation instances which are contained in this flow instance.
    ///
    /// By default, no relation instances are contained in this flow instance.
    #[serde(default = "RelationInstances::new", alias = "relations")]
    #[builder(default, setter(into))]
    pub relation_instances: RelationInstances,
}

impl FlowInstance {
    /// Constructs a new flow instance from the wrapper entity instance.
    pub fn from_instance_with_name<S: Into<String>>(wrapper_entity_instance: EntityInstance, name: S) -> FlowInstance {
        FlowInstance {
            id: wrapper_entity_instance.id,
            ty: wrapper_entity_instance.ty.clone(),
            name: name.into(),
            description: String::new(),
            entity_instances: EntityInstances::new_with_instance(wrapper_entity_instance),
            relation_instances: RelationInstances::new(),
        }
    }
}

impl NamedInstanceContainer for FlowInstance {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

impl From<EntityInstance> for FlowInstance {
    fn from(wrapper_entity_instance: EntityInstance) -> FlowInstance {
        FlowInstance {
            id: wrapper_entity_instance.id,
            ty: wrapper_entity_instance.ty.clone(),
            name: String::new(),
            description: String::new(),
            entity_instances: EntityInstances::new_with_instance(wrapper_entity_instance),
            relation_instances: RelationInstances::new(),
        }
    }
}

impl NamespacedTypeGetter for FlowInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for FlowInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }
}
impl PartialEq<Uuid> for FlowInstance {
    fn eq(&self, id: &Uuid) -> bool {
        self.id == *id
    }
}

impl PartialOrd<Self> for FlowInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlowInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Display for FlowInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.ty, self.id)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FlowInstances(DashMap<Uuid, FlowInstance>);

impl FlowInstances {
    pub fn new() -> Self {
        FlowInstances(DashMap::new())
    }

    pub fn new_with_instance<E: Into<FlowInstance>>(flow_instance: E) -> Self {
        let flow_instances = FlowInstances::new();
        flow_instances.push(flow_instance.into());
        flow_instances
    }

    pub fn push<E: Into<FlowInstance>>(&self, flow_instance: E) {
        let flow_instance = flow_instance.into();
        self.0.insert(flow_instance.id, flow_instance);
    }

    pub fn to_vec(&self) -> Vec<FlowInstance> {
        let mut items: Vec<_> = self.iter().map(|item| item.value().clone()).collect();
        items.sort();
        items
    }

    // TODO: deduplicate?
    // pub fn get_type_ids(&self) -> FlowTypeIds {
    //     self.iter().map(|flow_instance| flow_instance.ty.clone()).collect()
    // }
}

impl Deref for FlowInstances {
    type Target = DashMap<Uuid, FlowInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlowInstances {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for FlowInstances {
    type Item = (Uuid, FlowInstance);
    type IntoIter = OwningIter<Uuid, FlowInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for FlowInstances {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|self_flow_instance| other.contains_key(&self_flow_instance.id))
            && other.iter().all(|other_flow_instance| self.contains_key(&other_flow_instance.id))
    }
}

impl Eq for FlowInstances {}

impl Hash for FlowInstances {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl Serialize for FlowInstances {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de> Deserialize<'de> for FlowInstances {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<FlowInstance>::deserialize(deserializer)?.into())
    }
}

impl JsonSchema for FlowInstances {
    fn schema_name() -> Cow<'static, str> {
        "FlowInstances".into()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = gen.subschema_for::<FlowInstance>().into();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Flow Instances",
        })
    }
}

impl From<Vec<FlowInstance>> for FlowInstances {
    fn from(flow_instances: Vec<FlowInstance>) -> Self {
        Self(flow_instances.into_iter().map(|flow_instance| (flow_instance.id, flow_instance)).collect())
    }
}

impl From<FlowInstances> for Vec<FlowInstance> {
    fn from(flow_instances: FlowInstances) -> Self {
        flow_instances.to_vec()
    }
}

impl From<&FlowInstances> for Vec<FlowInstance> {
    fn from(flow_instances: &FlowInstances) -> Self {
        flow_instances.0.iter().map(|flow_instance| flow_instance.clone()).collect()
    }
}

impl From<DashMap<Uuid, FlowInstance>> for FlowInstances {
    fn from(flow_instances: DashMap<Uuid, FlowInstance>) -> Self {
        Self(flow_instances)
    }
}

impl From<&DashMap<Uuid, FlowInstance>> for FlowInstances {
    fn from(flow_instances: &DashMap<Uuid, FlowInstance>) -> Self {
        Self(flow_instances.clone())
    }
}

impl From<FlowInstances> for DashMap<Uuid, FlowInstance> {
    fn from(flow_instances: FlowInstances) -> Self {
        flow_instances.0
    }
}

impl FromIterator<FlowInstance> for FlowInstances {
    fn from_iter<I: IntoIterator<Item = FlowInstance>>(iter: I) -> Self {
        let flow_instances = Self::new();
        for flow_instance in iter {
            flow_instances.insert(flow_instance.id, flow_instance);
        }
        flow_instances
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for FlowInstance {
    fn default_test() -> Self {
        let entity_type = EntityType::default_test();
        let wrapper_entity_instance = EntityInstance::default_from(&entity_type);
        let id = wrapper_entity_instance.id;

        let entity_instances = EntityInstances::default_test();
        entity_instances.push(wrapper_entity_instance);

        FlowInstance::builder()
            .ty(entity_type.ty.clone())
            .id(id)
            .entity_instances(entity_instances)
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultFrom<EntityType> for FlowInstance {
    fn default_from(entity_type: &EntityType) -> Self {
        Self::default_from(&EntityInstance::default_from(entity_type))
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultFrom<EntityInstance> for FlowInstance {
    fn default_from(wrapper_entity_instance: &EntityInstance) -> Self {
        FlowInstance::from(wrapper_entity_instance.clone())
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for FlowInstances {
    fn default_test() -> Self {
        let flow_instances = FlowInstances::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            flow_instances.push(FlowInstance::default_test());
        }
        flow_instances
    }
}
