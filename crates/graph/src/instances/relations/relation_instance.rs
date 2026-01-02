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
use serde_json::Map;
use serde_json::Value;
use std::borrow::Cow;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::AddExtensionError;
use crate::ComponentTypeId;
use crate::ComponentTypeIdContainer;
use crate::ComponentTypeIds;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::InstanceId;
use crate::JSON_SCHEMA_ID_URI_PREFIX;
use crate::MutablePropertyInstanceSetter;
use crate::NamedInstanceContainer;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstances;
use crate::RelationInstanceId;
use crate::RelationInstanceTypeId;
use crate::RelationTypeId;
use crate::RelationTypeIds;
use crate::RemoveExtensionError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::namespace::Namespace;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::RandomInstance;
#[cfg(any(test, feature = "test"))]
use crate::RandomInstances;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypes;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

pub const JSON_SCHEMA_ID_RELATION_INSTANCE: &str = formatcp!("{}/relation-instance.schema.json", JSON_SCHEMA_ID_URI_PREFIX);

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore, the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In contrast to the relation type, the relation instance stores values/
/// documents in its properties.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/relation-instance.schema.json")]
#[schemars(
    title = "RelationInstance",
    rename = "RelationInstance",
    deny_unknown_fields,
    extend("$id" = JSON_SCHEMA_ID_RELATION_INSTANCE),
    transform = add_json_schema_id_property
)]
pub struct RelationInstance {
    /// The id of the outbound entity instance.
    pub outbound_id: Uuid,

    /// The relation instance id is unique and consists of the relation type and an instance_id.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub ty: RelationInstanceTypeId,

    /// The id of the inbound entity instance.
    pub inbound_id: Uuid,

    /// The name of the relation instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub name: String,

    /// Textual description of the relation instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The properties of then relation instance.
    ///
    /// Each property is represented by its name (String) and it's value. The value is
    /// a representation of a JSON. Therefore, the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// <https://docs.serde.rs/serde_json/value/enum.Value.html>
    #[serde(default = "PropertyInstances::new")]
    #[builder(default, setter(into))]
    pub properties: PropertyInstances,

    /// The components of the entity instance.
    #[serde(default = "ComponentTypeIds::new")]
    #[builder(default, setter(into))]
    pub components: ComponentTypeIds,

    /// Relation instance specific extensions.
    #[serde(default = "Extensions::new")]
    #[builder(default, setter(into))]
    pub extensions: Extensions,
}

impl RelationInstance {
    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new<T: Into<RelationInstanceTypeId>, P: Into<PropertyInstances>>(outbound_id: Uuid, ty: T, inbound_id: Uuid, properties: P) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: ty.into(),
            inbound_id,
            name: String::new(),
            description: String::new(),
            properties: properties.into(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id but without properties
    pub fn new_without_properties<T: Into<RelationInstanceTypeId>>(outbound_id: Uuid, ty: T, inbound_id: Uuid) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: ty.into(),
            inbound_id,
            name: String::new(),
            description: String::new(),
            properties: PropertyInstances::new(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }

    /// Returns the relation instance id.
    pub fn id(&self) -> RelationInstanceId {
        RelationInstanceId::builder()
            .outbound_id(self.outbound_id)
            .ty(self.ty.clone())
            .inbound_id(self.inbound_id)
            .build()
    }

    /// Returns the inner relation type id.
    pub fn relation_type_id(&self) -> RelationTypeId {
        self.ty.relation_type_id()
    }

    /// Returns the relation instance type id.
    pub fn instance_id(&self) -> InstanceId {
        self.ty.instance_id()
    }
}

impl NamedInstanceContainer for RelationInstance {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

impl PropertyInstanceGetter for RelationInstance {
    fn get(&self, property_name: &str) -> Option<Value> {
        self.properties.get(property_name.into())
    }

    fn as_bool(&self, property_name: &str) -> Option<bool> {
        self.properties.as_bool(property_name.into())
    }

    fn as_u64(&self, property_name: &str) -> Option<u64> {
        self.properties.as_u64(property_name.into())
    }

    fn as_i64(&self, property_name: &str) -> Option<i64> {
        self.properties.as_i64(property_name.into())
    }

    fn as_f64(&self, property_name: &str) -> Option<f64> {
        self.properties.as_f64(property_name.into())
    }

    fn as_string(&self, property_name: &str) -> Option<String> {
        self.properties.as_string(property_name.into())
    }

    fn as_array(&self, property_name: &str) -> Option<Vec<Value>> {
        self.properties.as_array(property_name.into())
    }

    fn as_object(&self, property_name: &str) -> Option<Map<String, Value>> {
        self.properties.as_object(property_name.into())
    }
}

impl MutablePropertyInstanceSetter for RelationInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        self.properties.set(property_name.into(), value);
    }
}

impl ComponentTypeIdContainer for RelationInstance {
    fn is_a(&self, ty: &ComponentTypeId) -> bool {
        self.components.is_a(ty)
    }

    fn add_component<C: Into<ComponentTypeId>>(&self, ty: C) -> bool {
        self.components.add_component(ty)
    }

    fn add_components<C: Into<ComponentTypeIds>>(&mut self, components_to_add: C) {
        self.components.add_components(components_to_add)
    }

    fn remove_component(&self, ty: &ComponentTypeId) -> Option<ComponentTypeId> {
        self.components.remove_component(ty)
    }

    fn remove_components<C: Into<ComponentTypeIds>>(&mut self, components_to_remove: C) {
        self.components.remove_components(components_to_remove)
    }

    fn get_components_cloned(&self) -> ComponentTypeIds {
        self.components.clone()
    }
}

impl ExtensionContainer for RelationInstance {
    fn has_own_extension(&self, ty: &ExtensionTypeId) -> bool {
        self.extensions.has_own_extension(ty)
    }

    fn get_own_extension(&self, ty: &ExtensionTypeId) -> Option<Extension> {
        self.extensions.get_own_extension(ty)
    }

    fn add_extension<E: Into<Extension>>(&self, extension: E) -> Result<ExtensionTypeId, AddExtensionError> {
        self.extensions.add_extension(extension)
    }

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(&self, ty: T, extension: E) -> Result<Extension, UpdateExtensionError> {
        self.extensions.update_extension(ty, extension)
    }

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, ty: T) -> Result<Extension, RemoveExtensionError> {
        self.extensions.remove_extension(ty)
    }

    fn merge_extensions<E: Into<Extensions>>(&mut self, extensions_to_merge: E) {
        self.extensions.merge_extensions(extensions_to_merge)
    }

    fn get_own_extensions_cloned(&self) -> Extensions {
        self.extensions.clone()
    }
}

impl NamespacedTypeGetter for RelationInstance {
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

impl TypeDefinitionGetter for RelationInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::RelationType
    }
}

impl PartialEq<RelationInstanceId> for RelationInstance {
    fn eq(&self, id: &RelationInstanceId) -> bool {
        self.id() == *id
    }
}

impl PartialOrd<Self> for RelationInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelationInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        let id = self.id();
        let other_id = other.id();
        id.cmp(&other_id)
    }
}

impl Display for RelationInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
        // write!(f, "{}--[{}]-->{}", self.outbound_id, &self.ty, self.inbound_id)
    }
}

#[derive(Clone, Debug, Default)]
// #[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelationInstances(DashMap<RelationInstanceId, RelationInstance>);

impl RelationInstances {
    pub fn new() -> Self {
        RelationInstances(DashMap::new())
    }

    pub fn push<E: Into<RelationInstance>>(&self, relation_instance: E) {
        let relation_instance = relation_instance.into();
        self.0.insert(relation_instance.id(), relation_instance);
    }

    pub fn to_vec(&self) -> Vec<RelationInstance> {
        let mut items: Vec<_> = self.iter().map(|item| item.value().clone()).collect();
        items.sort();
        items
    }

    // TODO: deduplicate?
    pub fn get_type_ids(&self) -> RelationTypeIds {
        self.iter().map(|r| r.relation_type_id()).collect()
    }
}

impl Deref for RelationInstances {
    type Target = DashMap<RelationInstanceId, RelationInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RelationInstances {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for RelationInstances {
    type Item = (RelationInstanceId, RelationInstance);
    type IntoIter = OwningIter<RelationInstanceId, RelationInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for RelationInstances {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|self_entity_instance| other.contains_key(&self_entity_instance.id()))
            && other.iter().all(|other_entity_instance| self.contains_key(&other_entity_instance.id()))
    }
}

impl Eq for RelationInstances {}

impl Hash for RelationInstances {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl Serialize for RelationInstances {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de> Deserialize<'de> for RelationInstances {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<RelationInstance>::deserialize(deserializer)?.into())
    }
}

impl JsonSchema for RelationInstances {
    fn schema_name() -> Cow<'static, str> {
        "RelationInstances".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<RelationInstance>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Relation Instances",
        })
    }
}

impl From<Vec<RelationInstance>> for RelationInstances {
    fn from(relation_instances: Vec<RelationInstance>) -> Self {
        Self(
            relation_instances
                .into_iter()
                .map(|relation_instance| (relation_instance.id(), relation_instance))
                .collect(),
        )
    }
}

impl From<RelationInstances> for Vec<RelationInstance> {
    fn from(relation_instances: RelationInstances) -> Self {
        relation_instances.to_vec()
    }
}

impl From<&RelationInstances> for Vec<RelationInstance> {
    fn from(relation_instances: &RelationInstances) -> Self {
        relation_instances.0.iter().map(|relation_instance| relation_instance.clone()).collect()
    }
}

impl From<DashMap<RelationInstanceId, RelationInstance>> for RelationInstances {
    fn from(relation_instances: DashMap<RelationInstanceId, RelationInstance>) -> Self {
        Self(relation_instances)
    }
}

impl From<&DashMap<RelationInstanceId, RelationInstance>> for RelationInstances {
    fn from(relation_instances: &DashMap<RelationInstanceId, RelationInstance>) -> Self {
        Self(relation_instances.clone())
    }
}

impl From<RelationInstances> for DashMap<RelationInstanceId, RelationInstance> {
    fn from(relation_instances: RelationInstances) -> Self {
        relation_instances.0
    }
}

impl FromIterator<RelationInstance> for RelationInstances {
    fn from_iter<I: IntoIterator<Item = RelationInstance>>(iter: I) -> Self {
        let relation_instances = Self::new();
        for relation_instance in iter {
            relation_instances.insert(relation_instance.id(), relation_instance);
        }
        relation_instances
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomInstance for RelationInstance {
    type Error = NamespacedTypeError;
    type TypeId = RelationInstanceTypeId;

    fn random_instance() -> Result<Self, NamespacedTypeError> {
        Self::random_instance_with_id(&RelationInstanceTypeId::random_type_id()?)
    }

    fn random_instance_with_id(ty: &Self::TypeId) -> Result<Self, Self::Error> {
        Ok(RelationInstance::builder()
            .outbound_id(Uuid::new_v4())
            .ty(ty)
            .inbound_id(Uuid::new_v4())
            .name(r_string())
            .description(r_string())
            .properties(PropertyInstances::default_test())
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomInstances for RelationInstances {
    type Error = NamespacedTypeError;

    fn random_instances() -> Result<Self, NamespacedTypeError> {
        let instances = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            instances.push(RelationInstance::random_instance()?);
        }
        Ok(instances)
    }
}

fn add_json_schema_id_property(schema: &mut Schema) {
    crate::json_schema::add_json_schema_id_property(schema, JSON_SCHEMA_ID_RELATION_INSTANCE);
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use serde_json::json;
    use std::ops::Index;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIdContainer;
    use crate::ComponentTypeIds;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::InstanceId;
    use crate::MutablePropertyInstanceSetter;
    use crate::PropertyInstanceGetter;
    use crate::PropertyInstances;
    use crate::RandomInstance;
    use crate::RandomNamespacedType;
    use crate::RandomNamespacedTypeId;
    use crate::RelationInstance;
    use crate::RelationInstanceId;
    use crate::RelationInstanceTypeId;
    use crate::RelationTypeId;
    use reactive_graph_utils_test::r_string;

    #[test]
    pub fn build_relation_instance() {
        let relation_ty = RelationTypeId::random_type_id().unwrap();
        let instance_id = InstanceId::Id(Uuid::from_str("7839aec8-07ea-41d1-84e5-f00b59d62c3e").unwrap());
        let relation_instance_ty = RelationInstanceTypeId::new(relation_ty.clone(), instance_id.clone());
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let name = r_string();
        let description = r_string();
        let property_name = r_string();
        let property_value = json!(r_string());
        let component_ty = ComponentTypeId::random_type_id().unwrap();
        let components = ComponentTypeIds::new().component(&component_ty);
        let properties = PropertyInstances::new().property(property_name.clone(), property_value.clone());
        let extension = Extension::random_type().unwrap();
        let extensions = Extensions::new().extension(extension.clone());
        let relation_instance = RelationInstance::builder()
            .outbound_id(outbound_id)
            .ty(&relation_instance_ty)
            .inbound_id(inbound_id)
            .name(&name)
            .description(&description)
            .components(components.clone())
            .properties(properties.clone())
            .extensions(extensions.clone())
            .build();
        assert_eq!(outbound_id, relation_instance.outbound_id);
        assert_eq!(relation_instance_ty, relation_instance.ty);
        assert_eq!(relation_ty, relation_instance.relation_type_id());
        assert_eq!(instance_id, relation_instance.instance_id());
        assert_eq!(inbound_id, relation_instance.inbound_id);
        assert_eq!(name, relation_instance.name);
        assert_eq!(description, relation_instance.description);
        assert_eq!(components, relation_instance.components);
        assert!(relation_instance.is_a(&component_ty));
        assert_eq!(properties, relation_instance.properties);
        assert_eq!(property_value, relation_instance.properties.get(&property_name).unwrap());
        assert_eq!(extensions, relation_instance.extensions);
        assert!(relation_instance.has_own_extension(&extension.ty));
        assert_eq!(extension, relation_instance.get_own_extension(&extension.ty).unwrap());
    }

    #[test]
    pub fn create_relation_instance() {
        let relation_ty = RelationTypeId::random_type_id().unwrap();
        let instance_id = InstanceId::Id(Uuid::from_str("7839aec8-07ea-41d1-84e5-f00b59d62c3e").unwrap());
        let relation_instance_ty = RelationInstanceTypeId::new(relation_ty.clone(), instance_id.clone());
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let relation_instance_id = RelationInstanceId::builder()
            .outbound_id(outbound_id)
            .ty(&relation_instance_ty)
            .inbound_id(inbound_id)
            .build();
        let property_name = r_string();
        let property_value = json!(r_string());
        let properties = PropertyInstances::new().property(property_name.clone(), property_value.clone());
        let relation_instance = RelationInstance::new(
            relation_instance_id.outbound_id,
            relation_instance_id.ty,
            relation_instance_id.inbound_id,
            PropertyInstances::new().property(&property_name, property_value.clone()),
        );
        assert_eq!(outbound_id, relation_instance.outbound_id);
        assert_eq!(relation_instance_ty, relation_instance.ty);
        assert_eq!(relation_ty, relation_instance.relation_type_id());
        assert_eq!(instance_id, relation_instance.instance_id());
        assert_eq!(inbound_id, relation_instance.inbound_id);
        assert_eq!(properties, relation_instance.properties);
        assert_eq!(property_value, relation_instance.properties.get(&property_name).unwrap());
    }

    #[test]
    fn relation_instance_typed_getter_test() {
        let property_name = r_string();
        let mut i = RelationInstance::new(
            Uuid::new_v4(),
            RelationInstanceTypeId::new(RelationTypeId::random_type_id().unwrap(), InstanceId::Id(Uuid::new_v4())),
            Uuid::new_v4(),
            PropertyInstances::new().property(&property_name, json!(false)),
        );
        i.set(property_name.clone(), json!(true));
        assert!(i.as_bool(&property_name).unwrap());
        i.set(property_name.clone(), json!(false));
        assert!(!i.as_bool(&property_name).unwrap());
        i.set(property_name.clone(), json!(123));
        assert_eq!(123, i.as_u64(&property_name).unwrap());
        i.set(property_name.clone(), json!(-123));
        assert_eq!(-123, i.as_i64(&property_name).unwrap());
        i.set(property_name.clone(), json!(1.23));
        assert_eq!(1.23, i.as_f64(&property_name).unwrap());
        let s = r_string();
        i.set(property_name.clone(), json!(s.clone()));
        assert_eq!(s, i.as_string(&property_name).unwrap());
        let a = json!([1, 2, 3]);
        i.set(property_name.clone(), a.clone());
        assert_eq!(json!(1), i.as_array(&property_name).unwrap().index(0).clone());
        assert_eq!(json!(2), i.as_array(&property_name).unwrap().index(1).clone());
        assert_eq!(json!(3), i.as_array(&property_name).unwrap().index(2).clone());
        let o = json!({
            "k": "v"
        });
        i.set(property_name.clone(), o.clone());
        assert_eq!(json!("v"), i.as_object(&property_name).unwrap().index("k").clone());
    }

    #[test]
    fn relation_instance_deserialize_fully_valid_test() {
        let outbound_id = Uuid::from_str("13104a11-96d3-4e24-aa87-e4037de7a28e").unwrap();
        let ty = RelationTypeId::from_str("fully::qualified::namespace::RelationType").unwrap();
        let instance_id = InstanceId::Id(Uuid::from_str("7839aec8-07ea-41d1-84e5-f00b59d62c3e").unwrap());
        let inbound_id = Uuid::from_str("0924e83a-52cf-4540-9a21-37cc87150a17").unwrap();
        let component_ty = ComponentTypeId::from_str("fully::qualified::namespace::Component").unwrap();
        let extension_ty = ExtensionTypeId::from_str("fully::qualified::namespace::Extension").unwrap();
        let relation_instance = serde_json::from_str::<RelationInstance>(
            r#"{
          "outbound_id": "13104a11-96d3-4e24-aa87-e4037de7a28e",
          "type": "fully::qualified::namespace::RelationType",
          "instance_id": "7839aec8-07ea-41d1-84e5-f00b59d62c3e",
          "inbound_id": "0924e83a-52cf-4540-9a21-37cc87150a17",
          "description": "d",
          "components": [
            "fully::qualified::namespace::Component"
          ],
          "properties": {
            "property_name": "property_value"
          },
          "extensions": [
            {
              "type": "fully::qualified::namespace::Extension",
              "extension": ""
            }
          ]
        }"#,
        )
        .expect("Failed to deserialize relation instance");
        assert_eq!(outbound_id, relation_instance.outbound_id);
        assert_eq!(ty, relation_instance.ty.relation_type_id());
        assert_eq!(instance_id, relation_instance.ty.instance_id());
        assert_eq!(inbound_id, relation_instance.inbound_id);
        assert_eq!("d", relation_instance.description);
        assert_eq!(1, relation_instance.components.len());
        assert!(relation_instance.is_a(&component_ty));
        assert_eq!(1, relation_instance.properties.len());
        assert_eq!("property_value", relation_instance.get("property_name").unwrap());
        assert_eq!(1, relation_instance.extensions.len());
        assert!(relation_instance.get_own_extension(&extension_ty).is_some());
    }

    #[test]
    fn relation_instance_ser_test() {
        let relation_instance = RelationInstance::random_instance().unwrap();
        println!("{}", serde_json::to_string_pretty(&relation_instance).expect("Failed to serialize relation instance"));
    }

    #[test]
    fn relation_instance_json_schema() {
        let schema = schema_for!(RelationInstance);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
