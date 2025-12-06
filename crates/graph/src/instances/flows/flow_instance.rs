use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use const_format::formatcp;
use dashmap::DashMap;
use dashmap::iter::OwningIter;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::EntityInstance;
use crate::EntityInstances;
use crate::EntityTypeId;
use crate::JSON_SCHEMA_ID_URI_PREFIX;
use crate::NAMESPACE_SEPARATOR;
use crate::NamedInstanceContainer;
use crate::Namespace;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::RelationInstances;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

#[cfg(any(test, feature = "test"))]
use crate::EntityType;
#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::RandomInstance;
#[cfg(any(test, feature = "test"))]
use crate::RandomInstances;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::DefaultFrom;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::DefaultTryFrom;

pub const JSON_SCHEMA_ID_FLOW_INSTANCE: &str = formatcp!("{}/flow-instance.schema.json", JSON_SCHEMA_ID_URI_PREFIX);

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
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/flow-instance.schema.json")]
#[schemars(
    title = "FlowInstance",
    deny_unknown_fields,
    extend("$id" = JSON_SCHEMA_ID_FLOW_INSTANCE),
    transform = add_json_schema_id_property
)]
pub struct FlowInstance {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The type definition of the entity type of the wrapper entity instance.
    #[serde(rename = "type")]
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
    fn namespaced_type(&self) -> NamespacedType {
        self.ty.namespaced_type()
    }

    fn namespace(&self) -> Namespace {
        self.ty.namespace()
    }

    fn path(&self) -> Namespace {
        self.ty.path()
    }

    fn type_name(&self) -> NamespaceSegment {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for FlowInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::FlowType
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
        write!(f, "{}{}{}", &self.ty, NAMESPACE_SEPARATOR, self.id)
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

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<FlowInstance>();
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
impl RandomInstance for FlowInstance {
    type Error = NamespacedTypeError;
    type TypeId = EntityTypeId;

    fn random_instance() -> Result<Self, NamespacedTypeError> {
        Self::random_instance_with_id(&EntityTypeId::random_type_id()?)
    }

    fn random_instance_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error> {
        let entity_type = EntityType::random_type_with_id(ty)?;
        let wrapper_entity_instance = EntityInstance::default_try_from(&entity_type)?;
        let id = wrapper_entity_instance.id;

        let entity_instances = EntityInstances::random_instances()?;
        entity_instances.push(wrapper_entity_instance);

        Ok(FlowInstance::builder()
            .ty(entity_type.ty.clone())
            .id(id)
            .entity_instances(entity_instances)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTryFrom<&EntityType> for FlowInstance {
    type Error = NamespacedTypeError;

    fn default_try_from(entity_type: &EntityType) -> Result<Self, NamespacedTypeError> {
        let wrapper_entity_instance = EntityInstance::default_try_from(entity_type)?;
        Ok(Self::default_from(&wrapper_entity_instance))
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultFrom<&EntityInstance> for FlowInstance {
    fn default_from(wrapper_entity_instance: &EntityInstance) -> Self {
        FlowInstance::from(wrapper_entity_instance.clone())
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomInstances for FlowInstances {
    type Error = NamespacedTypeError;

    fn random_instances() -> Result<Self, NamespacedTypeError> {
        let instances = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            instances.push(FlowInstance::random_instance()?);
        }
        Ok(instances)
    }
}

fn add_json_schema_id_property(schema: &mut Schema) {
    crate::json_schema::add_json_schema_id_property(schema, JSON_SCHEMA_ID_FLOW_INSTANCE);
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use uuid::Uuid;

    use crate::EntityInstance;
    use crate::EntityInstances;
    use crate::EntityTypeId;
    use crate::FlowInstance;
    use crate::NamespacedTypeGetter;
    use crate::RandomInstance;
    use crate::RandomNamespacedTypeId;
    use crate::RelationInstances;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn flow_instance_test() {
        let flow_id = Uuid::new_v4();
        let flow_name = r_string();
        let flow_description = r_string();

        let ty = EntityTypeId::random_type_id().unwrap();
        let flow_instance = FlowInstance {
            id: flow_id,
            ty: ty.clone(),
            name: flow_name.clone(),
            description: flow_description.to_string(),
            entity_instances: EntityInstances::new(),
            relation_instances: RelationInstances::new(),
        };

        assert_eq!(ty.namespace(), flow_instance.namespace());
        assert_eq!(ty.path(), flow_instance.path());
        assert_eq!(ty.type_name(), flow_instance.type_name());
        assert_eq!(flow_id.clone(), flow_instance.id.clone());
        assert_eq!(flow_name.clone(), flow_instance.name.clone());
        assert_eq!(flow_description.clone(), flow_instance.description.clone());
        assert_eq!(0, flow_instance.entity_instances.len());
        assert_eq!(0, flow_instance.relation_instances.len());
    }

    #[test]
    fn flow_instance_from_entity_instance_test() {
        let wrapper_entity_instance = EntityInstance::random_instance().unwrap();
        let wrapper_type = wrapper_entity_instance.ty.clone();
        let flow_instance = FlowInstance::from(wrapper_entity_instance.clone());
        assert_eq!(wrapper_type, flow_instance.ty);
        assert_eq!(wrapper_type.namespace(), flow_instance.namespace());
        assert_eq!(wrapper_type.path(), flow_instance.path());
        assert_eq!(wrapper_type.type_name(), flow_instance.type_name());
        assert_eq!(wrapper_entity_instance.id, flow_instance.id);
        assert_eq!(String::new(), flow_instance.name);
    }

    #[test]
    fn flow_instance_from_entity_instance_with_name_test() {
        let wrapper_entity_instance = EntityInstance::random_instance().unwrap();
        let wrapper_type = wrapper_entity_instance.ty.clone();
        let flow_name = r_string();
        let flow_instance = FlowInstance::from_instance_with_name(wrapper_entity_instance.clone(), flow_name.clone());
        assert_eq!(wrapper_type, flow_instance.ty);
        assert_eq!(wrapper_type.namespace(), flow_instance.namespace());
        assert_eq!(wrapper_type.path(), flow_instance.path());
        assert_eq!(wrapper_type.type_name(), flow_instance.type_name());
        assert_eq!(wrapper_entity_instance.id, flow_instance.id);
        assert_eq!(flow_name, flow_instance.name);
    }

    #[test]
    fn flow_instance_json_schema() {
        let schema = schema_for!(FlowInstance);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
