use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::iter::OwningIter;
use dashmap::DashMap;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
use schemars::gen::SchemaGenerator;
use schemars::schema::ArrayValidation;
use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_json::Map;
use serde_json::Value;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[cfg(any(test, feature = "test"))]
use crate::test_utils::default_from::DefaultFrom;
use crate::AddExtensionError;
use crate::ComponentTypeId;
use crate::ComponentTypeIdContainer;
use crate::ComponentTypeIds;
#[cfg(any(test, feature = "test"))]
use crate::EntityType;
use crate::EntityTypeId;
use crate::EntityTypeIds;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::MutablePropertyInstanceSetter;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceGetter;
use crate::PropertyInstances;
use crate::RemoveExtensionError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::UpdateExtensionError;
#[cfg(any(test, feature = "test"))]
use reactive_graph_test_utils::r_string;

/// Entity instances represents an typed object which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in it's
/// properties.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct EntityInstance {
    /// The type definition of the entity type.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub ty: EntityTypeId,

    /// The unique identifier of the entity instance.
    #[builder(default=Uuid::new_v4())]
    pub id: Uuid,

    /// The description of the entity instance.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The properties of then entity instance.
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
            description: String::new(),
            properties: properties.into(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }

    /// Constructs a new entity instance with the given namespace, type_name, id and properties.
    pub fn new_from_type<N: Into<String>, T: Into<String>, P: Into<PropertyInstances>>(namespace: N, type_name: T, id: Uuid, properties: P) -> EntityInstance {
        EntityInstance {
            ty: EntityTypeId::new_from_type(namespace.into(), type_name.into()),
            id,
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
            description: String::new(),
            properties: PropertyInstances::new(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }
}

impl PropertyInstanceGetter for EntityInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(property_name.into())
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.properties.as_bool(property_name.into())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.properties.as_u64(property_name.into())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.properties.as_i64(property_name.into())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.properties.as_f64(property_name.into())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.properties.as_string(property_name.into())
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.properties.as_array(property_name.into())
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
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
}

impl NamespacedTypeGetter for EntityInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for EntityInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
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
        write!(f, "{}__{}", &self.ty, self.id)
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
    fn schema_name() -> String {
        "EntityInstances".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<EntityInstance>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
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
impl DefaultTest for EntityInstance {
    fn default_test() -> Self {
        EntityInstance::builder()
            .ty(EntityTypeId::default_test())
            .description(r_string())
            .properties(PropertyInstances::default_test())
            .extensions(Extensions::default_test())
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultFrom<EntityType> for EntityInstance {
    fn default_from(entity_type: &EntityType) -> Self {
        let properties = PropertyInstances::default_from(&entity_type.properties);
        EntityInstance::builder()
            .ty(&entity_type.ty)
            .description(&entity_type.description)
            .properties(properties)
            .extensions(Extensions::default_test())
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for EntityInstances {
    fn default_test() -> Self {
        let entity_instances = EntityInstances::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            entity_instances.push(EntityInstance::default_test());
        }
        entity_instances
    }
}

#[cfg(test)]
pub mod entity_instance_tests {
    use std::ops::Index;

    use default_test::DefaultTest;
    use schemars::schema_for;
    use serde_json::json;
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
    use crate::NamespacedTypeGetter;
    use crate::PropertyInstanceGetter;
    use crate::PropertyInstances;
    use reactive_graph_test_utils::r_string;

    pub fn create_entity_instance_with_property<S: Into<String>>(property_name: S) -> EntityInstance {
        let properties = PropertyInstances::new().property(property_name, json!(r_string()));
        // properties.insert(property_name.into(), json!(r_string()));
        EntityInstance::builder()
            .ty(EntityTypeId::default_test())
            .description(r_string())
            .properties(properties)
            .extensions(Extensions::default_test())
            .build()
    }

    pub fn create_entity_instance_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> EntityInstance {
        EntityInstance::builder()
            .ty(EntityTypeId::new_from_type(namespace.into(), type_name.into()))
            .build()
    }

    #[test]
    fn entity_instance_test() {
        let uuid = Uuid::new_v4();
        let namespace = r_string();
        let type_name = r_string();
        let description = r_string();
        let property_name = r_string();
        let property_value = json!(r_string());
        let properties = PropertyInstances::new().property(&property_name, property_value.clone());

        let component_namespace = r_string();
        let component_name = r_string();
        let component_ty = ComponentTypeId::new_from_type(&component_namespace, &component_name);
        let components = ComponentTypeIds::new().component(component_ty.clone());

        let extension_namespace = r_string();
        let extension_name = r_string();
        let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
        let extension_value = json!("extension_value");
        let extension = Extension {
            ty: extension_ty.clone(),
            description: r_string(),
            extension: extension_value.clone(),
        };

        let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
        let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());

        let extensions = Extensions::new().extension(extension.clone()).extension(other_extension.clone());
        // extensions.push(extension.clone());
        // extensions.push(other_extension);

        let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
        let entity_instance = EntityInstance {
            ty: ty.clone(),
            id: uuid.clone(),
            description: description.to_string(),
            properties: properties.clone(),
            components: components.clone(),
            extensions: extensions.clone(),
        };
        assert_eq!(namespace, entity_instance.namespace());
        assert_eq!(type_name, entity_instance.type_name());
        assert_eq!(uuid.clone(), entity_instance.id.clone());
        assert_eq!(description.clone(), entity_instance.description.clone());
        assert_eq!(properties.clone(), entity_instance.properties.clone());
        assert!(entity_instance.get(property_name.clone()).is_some());
        assert!(entity_instance.get(r_string()).is_none());
        assert_eq!(property_value.clone(), entity_instance.get(property_name.clone()).unwrap());
        assert!(entity_instance.components.contains(&component_ty.clone()));
        assert!(entity_instance.components.is_a(&component_ty));
        assert!(entity_instance.is_a(&component_ty));
        assert!(!entity_instance.is_a(&ComponentTypeId::generate_random()));
        assert!(entity_instance.extensions.has_own_extension(&extension_ty));
        assert!(entity_instance.has_own_extension(&extension_ty));
        let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
        assert!(!entity_instance.has_own_extension(&non_existing_extension));
        assert_eq!(extension.extension, entity_instance.get_own_extension(&extension_ty).unwrap().extension);
        assert_eq!(format!("{}__{}", entity_instance.ty, entity_instance.id), format!("{}", entity_instance));
    }

    #[test]
    fn create_entity_instance_test() {
        let uuid = Uuid::new_v4();
        let namespace = r_string();
        let type_name = r_string();
        let property_name = r_string();
        let property_value = json!(r_string());
        let properties = PropertyInstances::new().property(&property_name, property_value.clone());
        let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
        let entity_instance = EntityInstance::new(ty, uuid, properties.clone());
        assert_eq!(namespace, entity_instance.namespace());
        assert_eq!(type_name, entity_instance.type_name());
        assert_eq!(uuid, entity_instance.id.clone());
        assert_eq!(properties.clone(), properties.clone());
        assert!(entity_instance.get(property_name.clone()).is_some());
        assert!(entity_instance.get(r_string()).is_none());
        assert_eq!(property_value.clone(), entity_instance.get(property_name.clone()).unwrap());
    }

    #[test]
    fn create_entity_instance_without_properties_test() {
        let uuid = Uuid::new_v4();
        let namespace = r_string();
        let type_name = r_string();
        let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
        let entity_instance = EntityInstance::new_without_properties(ty, uuid);
        assert_eq!(namespace, entity_instance.namespace());
        assert_eq!(type_name, entity_instance.type_name());
        assert_eq!(uuid, entity_instance.id.clone());
        assert!(entity_instance.get(r_string()).is_none());
    }

    #[test]
    fn entity_instance_typed_getter_test() {
        let uuid = Uuid::new_v4();
        let namespace = r_string();
        let type_name = r_string();
        let property_name = r_string();
        let properties = PropertyInstances::new().property(&property_name, json!(false));
        let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
        let mut i = EntityInstance::new(ty, uuid, properties);
        i.set(property_name.clone(), json!(true));
        assert!(i.as_bool(property_name.clone()).unwrap());
        i.set(property_name.clone(), json!(false));
        assert!(!i.as_bool(property_name.clone()).unwrap());
        i.set(property_name.clone(), json!(123));
        assert_eq!(123, i.as_u64(property_name.clone()).unwrap());
        i.set(property_name.clone(), json!(-123));
        assert_eq!(-123, i.as_i64(property_name.clone()).unwrap());
        i.set(property_name.clone(), json!(1.23));
        assert_eq!(1.23, i.as_f64(property_name.clone()).unwrap());
        let s = r_string();
        i.set(property_name.clone(), json!(s.clone()));
        assert_eq!(s, i.as_string(property_name.clone()).unwrap());
        let a = json!([1, 2, 3]);
        i.set(property_name.clone(), a.clone());
        assert_eq!(json!(1), i.as_array(property_name.clone()).unwrap().index(0).clone());
        assert_eq!(json!(2), i.as_array(property_name.clone()).unwrap().index(1).clone());
        assert_eq!(json!(3), i.as_array(property_name.clone()).unwrap().index(2).clone());
        let o = json!({
            "k": "v"
        });
        i.set(property_name.clone(), o.clone());
        assert_eq!(json!("v"), i.as_object(property_name.clone()).unwrap().index("k").clone());
    }

    #[test]
    fn entity_instance_ser_test() {
        let uuid = Uuid::new_v4();
        let namespace = r_string();
        let type_name = r_string();
        let description = r_string();
        let property_name = r_string();
        let property_value = json!(r_string());
        let properties = PropertyInstances::new().property(&property_name, property_value.clone());

        let extension_namespace = r_string();
        let extension_name = r_string();
        let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
        let extension_value = json!("extension_value");
        let extension = Extension {
            ty: extension_ty.clone(),
            description: r_string(),
            extension: extension_value.clone(),
        };
        let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
        let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
        let extensions = Extensions::new().extension(extension.clone()).extension(other_extension.clone());
        let component_namespace = r_string();
        let component_name = r_string();
        let component_ty = ComponentTypeId::new_from_type(&component_namespace, &component_name);
        let components = ComponentTypeIds::new().component(component_ty.clone());

        let ty = EntityTypeId::new_from_type(namespace.clone(), type_name.clone());
        let entity_instance = EntityInstance {
            ty: ty.clone(),
            id: uuid.clone(),
            description: description.to_string(),
            properties: properties.clone(),
            components: components.clone(),
            extensions: extensions.clone(),
        };
        println!("{}", serde_json::to_string_pretty(&entity_instance).expect("Failed to serialize entity instance"));
    }

    #[test]
    fn entity_instance_de_test() {
        let s = r#"{
  "namespace": "XARPbZkHrU",
  "type_name": "zHMZhLUpeH",
  "id": "590f4446-b080-48d3-bd14-05e09de89e62",
  "description": "gDyZTYONjh",
  "properties": {
    "NaUPOBoqyp": "qEnGqwNeEL"
  },
  "components": [
    {
      "namespace": "c_namespace",
      "type_name": "c_name"
    }
  ],
  "extensions": [
    {
      "namespace": "ext_namespace",
      "type_name": "ext_name",
      "extension": "extension_value"
    },
    {
      "namespace": "other_ext_namespace",
      "type_name": "other_ext_name",
      "extension": "other_extension_value"
    }
  ]
}"#;
        let entity_instance: EntityInstance = serde_json::from_str(s).unwrap();
        assert_eq!("XARPbZkHrU", entity_instance.namespace());
        assert_eq!("zHMZhLUpeH", entity_instance.type_name());
        assert_eq!("e__XARPbZkHrU__zHMZhLUpeH", entity_instance.ty.to_string());
        assert_eq!("gDyZTYONjh", entity_instance.description);
        assert_eq!(1, entity_instance.properties.len());
        let property = entity_instance.properties.get("NaUPOBoqyp").expect("Missing property");
        assert_eq!("qEnGqwNeEL", property.as_str().unwrap());
        assert_eq!(1, entity_instance.components.len());
        assert!(entity_instance.components.contains(&ComponentTypeId::new_from_type("c_namespace", "c_name")));
        assert!(entity_instance.components.is_a(&ComponentTypeId::new_from_type("c_namespace", "c_name")));
        assert!(entity_instance.is_a(&ComponentTypeId::new_from_type("c_namespace", "c_name")));
        assert_eq!(2, entity_instance.extensions.len());
        assert!(entity_instance
            .extensions
            .has_own_extension(&ExtensionTypeId::new_from_type("ext_namespace", "ext_name")));
        assert_eq!(
            json!("extension_value"),
            entity_instance
                .extensions
                .get_own_extension(&ExtensionTypeId::new_from_type("ext_namespace", "ext_name"))
                .unwrap()
                .extension
        );
        assert!(entity_instance
            .extensions
            .has_own_extension(&ExtensionTypeId::new_from_type("other_ext_namespace", "other_ext_name")));
        assert_eq!(
            json!("other_extension_value"),
            entity_instance
                .extensions
                .get_own_extension(&ExtensionTypeId::new_from_type("other_ext_namespace", "other_ext_name"))
                .unwrap()
                .extension
        );
    }

    #[test]
    fn entity_instance_json_schema() {
        let schema = schema_for!(EntityInstance);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
