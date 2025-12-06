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
use serde_json::Map;
use serde_json::Value;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::AddExtensionError;
use crate::ComponentTypeId;
use crate::ComponentTypeIdContainer;
use crate::ComponentTypeIds;
use crate::EntityTypeId;
use crate::EntityTypeIds;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::JSON_SCHEMA_ID_URI_PREFIX;
use crate::MutablePropertyInstanceSetter;
use crate::NamedInstanceContainer;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstances;
use crate::RemoveExtensionError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::namespace::NAMESPACE_SEPARATOR;
use crate::namespace::Namespace;

#[cfg(any(test, feature = "test"))]
use crate::EntityType;
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
use crate::test_utils::default_from::DefaultFrom;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::DefaultTryFrom;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

pub const JSON_SCHEMA_ID_ENTITY_INSTANCE: &str = formatcp!("{}/entity-instance.schema.json", JSON_SCHEMA_ID_URI_PREFIX);

/// Entity instances represents an typed object which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in its
/// properties.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/entity-instance.schema.json")]
#[schemars(
    title = "EntityInstance",
    rename = "EntityInstance",
    deny_unknown_fields,
    extend("$id" = JSON_SCHEMA_ID_ENTITY_INSTANCE),
    transform = add_json_schema_id_property
)]
pub struct EntityInstance {
    /// The type definition of the entity type.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub ty: EntityTypeId,

    /// The unique identifier of the entity instance.
    #[builder(default=Uuid::new_v4())]
    pub id: Uuid,

    /// The name of the entity instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub name: String,

    /// The description of the entity instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The properties of the entity instance.
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

    //
    // TODO: behaviours?
    //
    /// Entity instance specific extensions.
    #[serde(default = "Extensions::new")]
    #[builder(default, setter(into))]
    pub extensions: Extensions,
}

impl EntityInstance {
    /// Constructs a new entity instance with the given type.
    pub fn new<T: Into<EntityTypeId>, P: Into<PropertyInstances>>(ty: T, id: Uuid, properties: P) -> EntityInstance {
        EntityInstance {
            ty: ty.into(),
            id,
            name: String::new(),
            description: String::new(),
            properties: properties.into(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }

    /// Constructs a new entity instance with the given type and id but without properties.
    pub fn new_without_properties<T: Into<EntityTypeId>>(ty: T, id: Uuid) -> EntityInstance {
        EntityInstance {
            ty: ty.into(),
            id,
            name: String::new(),
            description: String::new(),
            properties: PropertyInstances::new(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }
}

impl NamedInstanceContainer for EntityInstance {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }
}

impl PropertyInstanceGetter for EntityInstance {
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

impl MutablePropertyInstanceSetter for EntityInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        self.properties.set(property_name.into(), value);
    }
}

impl ComponentTypeIdContainer for EntityInstance {
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

impl ExtensionContainer for EntityInstance {
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

impl NamespacedTypeGetter for EntityInstance {
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

impl TypeDefinitionGetter for EntityInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::EntityType
    }
}

impl PartialEq<Uuid> for EntityInstance {
    fn eq(&self, id: &Uuid) -> bool {
        self.id == *id
    }
}

impl PartialOrd<Self> for EntityInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EntityInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Display for EntityInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", &self.ty, NAMESPACE_SEPARATOR, self.id)
    }
}

#[derive(Clone, Debug, Default)]
pub struct EntityInstances(DashMap<Uuid, EntityInstance>);

impl EntityInstances {
    pub fn new() -> Self {
        EntityInstances(DashMap::new())
    }

    pub fn new_with_instance<E: Into<EntityInstance>>(entity_instance: E) -> Self {
        let entity_instances = EntityInstances::new();
        entity_instances.push(entity_instance.into());
        entity_instances
    }

    pub fn push<E: Into<EntityInstance>>(&self, entity_instance: E) {
        let entity_instance = entity_instance.into();
        self.0.insert(entity_instance.id, entity_instance);
    }

    pub fn to_vec(&self) -> Vec<EntityInstance> {
        let mut items: Vec<_> = self.iter().map(|item| item.value().clone()).collect();
        items.sort();
        items
    }

    pub fn get_type_ids(&self) -> EntityTypeIds {
        self.iter().map(|entity_instance| entity_instance.ty.clone()).collect()
    }
}

impl Deref for EntityInstances {
    type Target = DashMap<Uuid, EntityInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityInstances {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for EntityInstances {
    type Item = (Uuid, EntityInstance);
    type IntoIter = OwningIter<Uuid, EntityInstance>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for EntityInstances {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|self_entity_instance| other.contains_key(&self_entity_instance.id))
            && other.iter().all(|other_entity_instance| self.contains_key(&other_entity_instance.id))
    }
}

impl Eq for EntityInstances {}

impl Hash for EntityInstances {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl Serialize for EntityInstances {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.iter())
    }
}

impl<'de> Deserialize<'de> for EntityInstances {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<EntityInstance>::deserialize(deserializer)?.into())
    }
}

impl JsonSchema for EntityInstances {
    fn schema_name() -> Cow<'static, str> {
        "EntityInstances".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<EntityInstance>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Entity Instances",
        })
    }
}

impl From<Vec<EntityInstance>> for EntityInstances {
    fn from(entity_instances: Vec<EntityInstance>) -> Self {
        Self(
            entity_instances
                .into_iter()
                .map(|entity_instance| (entity_instance.id, entity_instance))
                .collect(),
        )
    }
}

impl From<EntityInstances> for Vec<EntityInstance> {
    fn from(entity_instances: EntityInstances) -> Self {
        entity_instances.to_vec()
    }
}

impl From<&EntityInstances> for Vec<EntityInstance> {
    fn from(entity_instances: &EntityInstances) -> Self {
        entity_instances.0.iter().map(|entity_instance| entity_instance.clone()).collect()
    }
}

impl From<DashMap<Uuid, EntityInstance>> for EntityInstances {
    fn from(entity_instances: DashMap<Uuid, EntityInstance>) -> Self {
        Self(entity_instances)
    }
}

impl From<&DashMap<Uuid, EntityInstance>> for EntityInstances {
    fn from(entity_instances: &DashMap<Uuid, EntityInstance>) -> Self {
        Self(entity_instances.clone())
    }
}

impl From<EntityInstances> for DashMap<Uuid, EntityInstance> {
    fn from(entity_instances: EntityInstances) -> Self {
        entity_instances.0
    }
}

impl FromIterator<EntityInstance> for EntityInstances {
    fn from_iter<I: IntoIterator<Item = EntityInstance>>(iter: I) -> Self {
        let entity_instances = Self::new();
        for entity_instance in iter {
            entity_instances.insert(entity_instance.id, entity_instance);
        }
        entity_instances
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomInstance for EntityInstance {
    type Error = NamespacedTypeError;
    type TypeId = EntityTypeId;

    fn random_instance() -> Result<Self, NamespacedTypeError> {
        Ok(EntityInstance::builder()
            .ty(EntityTypeId::random_type_id()?)
            .name(r_string())
            .description(r_string())
            .properties(PropertyInstances::default_test())
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }

    fn random_instance_with_id(ty: &EntityTypeId) -> Result<Self, Self::Error> {
        Ok(EntityInstance::builder()
            .ty(ty)
            .name(r_string())
            .description(r_string())
            .properties(PropertyInstances::default_test())
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTryFrom<&EntityType> for EntityInstance {
    type Error = NamespacedTypeError;

    fn default_try_from(entity_type: &EntityType) -> Result<Self, Self::Error> {
        let properties = PropertyInstances::default_from(&entity_type.properties);
        Ok(EntityInstance::builder()
            .ty(&entity_type.ty)
            .name(r_string())
            .description(&entity_type.description)
            .properties(properties)
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomInstances for EntityInstances {
    type Error = NamespacedTypeError;

    fn random_instances() -> Result<Self, NamespacedTypeError> {
        let instances = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            instances.push(EntityInstance::random_instance()?);
        }
        Ok(instances)
    }
}

fn add_json_schema_id_property(schema: &mut Schema) {
    crate::json_schema::add_json_schema_id_property(schema, JSON_SCHEMA_ID_ENTITY_INSTANCE);
}

#[cfg(test)]
pub mod entity_instance_tests {
    use schemars::schema_for;
    use serde_json::json;
    use std::ops::Index;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIdContainer;
    use crate::ComponentTypeIds;
    use crate::EntityInstance;
    use crate::EntityTypeId;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::MutablePropertyInstanceSetter;
    use crate::PropertyInstanceGetter;
    use crate::PropertyInstances;
    use crate::RandomInstance;
    use crate::RandomNamespacedType;
    use crate::RandomNamespacedTypeId;
    use reactive_graph_utils_test::r_string;

    #[test]
    pub fn build_entity_instance() {
        let ty = EntityTypeId::random_type_id().unwrap();
        let name = r_string();
        let description = r_string();
        let property_name = r_string();
        let property_value = json!(r_string());
        let component_ty = ComponentTypeId::random_type_id().unwrap();
        let components = ComponentTypeIds::new().component(&component_ty);
        let properties = PropertyInstances::new().property(property_name.clone(), property_value.clone());
        let extension = Extension::random_type().unwrap();
        let extensions = Extensions::new().extension(extension.clone());
        let entity_instance = EntityInstance::builder()
            .ty(&ty)
            .name(&name)
            .description(&description)
            .components(components.clone())
            .properties(properties.clone())
            .extensions(extensions.clone())
            .build();
        assert_eq!(ty, entity_instance.ty);
        assert_eq!(name, entity_instance.name);
        assert_eq!(description, entity_instance.description);
        assert_eq!(components, entity_instance.components);
        assert!(entity_instance.is_a(&component_ty));
        assert_eq!(properties, entity_instance.properties);
        assert_eq!(property_value, entity_instance.properties.get(&property_name).unwrap());
        assert_eq!(extensions, entity_instance.extensions);
        assert!(entity_instance.has_own_extension(&extension.ty));
        assert_eq!(extension, entity_instance.get_own_extension(&extension.ty).unwrap());
    }

    #[test]
    pub fn create_entity_instance() {
        let ty = EntityTypeId::random_type_id().unwrap();
        let property_name = r_string();
        let property_value = json!(r_string());
        let properties = PropertyInstances::new().property(property_name.clone(), property_value.clone());
        let entity_instance = EntityInstance::new(&ty, Uuid::new_v4(), properties.clone());
        assert_eq!(ty, entity_instance.ty);
        assert_eq!(properties, entity_instance.properties);
        assert_eq!(property_value, entity_instance.properties.get(&property_name).unwrap());
    }

    #[test]
    fn entity_instance_typed_getter_test() {
        let property_name = r_string();
        let properties = PropertyInstances::new().property(&property_name, json!(false));
        let mut i = EntityInstance::new(EntityTypeId::random_type_id().unwrap(), Uuid::new_v4(), properties);
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
    fn entity_instance_deserialize_fully_valid_test() {
        let id = Uuid::from_str("f3ef93a4-a384-40de-8075-83b92287fcba").unwrap();
        let ty = EntityTypeId::from_str("fully::qualified::namespace::EntityType").unwrap();
        let component_ty = ComponentTypeId::from_str("fully::qualified::namespace::Component").unwrap();
        let extension_ty = ExtensionTypeId::from_str("fully::qualified::namespace::Extension").unwrap();
        let entity_instance = serde_json::from_str::<EntityInstance>(
            r#"{
          "type": "fully::qualified::namespace::EntityType",
          "id": "f3ef93a4-a384-40de-8075-83b92287fcba",
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
        .expect("Failed to deserialize entity instance");
        assert_eq!(ty, entity_instance.ty);
        assert_eq!(id, entity_instance.id);
        assert_eq!("d", entity_instance.description);
        assert_eq!(1, entity_instance.components.len());
        assert!(entity_instance.is_a(&component_ty));
        assert_eq!(1, entity_instance.properties.len());
        assert_eq!("property_value", entity_instance.get("property_name").unwrap());
        assert_eq!(1, entity_instance.extensions.len());
        assert!(entity_instance.get_own_extension(&extension_ty).is_some());
    }

    #[test]
    fn entity_instance_ser_test() {
        let entity_instance = EntityInstance::random_instance().unwrap();
        println!("{}", serde_json::to_string_pretty(&entity_instance).expect("Failed to serialize entity instance"));
    }

    #[test]
    fn entity_instance_json_schema() {
        let schema = schema_for!(EntityInstance);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
