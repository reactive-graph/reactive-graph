use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashMap;
use dashmap::iter::OwningIter;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
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
use crate::MutablePropertyInstanceSetter;
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
use crate::UpdateExtensionError;
use crate::instances::named::NamedInstanceContainer;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;

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
#[schemars(title = "RelationInstance", rename = "RelationInstance", deny_unknown_fields, extend("$id" = "https://schema.reactive-graph.io/schema/json/relation-instance.schema.json"))]
pub struct RelationInstance {
    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The relation instance id is unique and consists of the relation type and an instance_id.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub ty: RelationInstanceTypeId,

    /// The id of the inbound vertex.
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

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new_from_type_unique_id<S: Into<String>, P: Into<PropertyInstances>>(
        namespace: S,
        outbound_id: Uuid,
        type_name: S,
        inbound_id: Uuid,
        properties: P,
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: RelationInstanceTypeId::new_from_type_unique_id(namespace, type_name),
            inbound_id,
            name: String::new(),
            description: String::new(),
            properties: properties.into(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new_from_type_unique_for_instance_id<S: Into<String>, P: Into<PropertyInstances>>(
        namespace: S,
        outbound_id: Uuid,
        type_name: S,
        instance_id: S,
        inbound_id: Uuid,
        properties: P,
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: RelationInstanceTypeId::new_from_type_unique_for_instance_id(namespace, type_name, instance_id),
            inbound_id,
            name: String::new(),
            description: String::new(),
            properties: properties.into(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new_from_type_with_random_instance_id<S: Into<String>, P: Into<PropertyInstances>>(
        namespace: S,
        outbound_id: Uuid,
        type_name: S,
        inbound_id: Uuid,
        properties: P,
    ) -> RelationInstance {
        RelationInstance {
            outbound_id,
            ty: RelationInstanceTypeId::new_from_type_with_random_instance_id(namespace, type_name),
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
    pub fn instance_id(&self) -> String {
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
}

impl NamespacedTypeGetter for RelationInstance {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for RelationInstance {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
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
impl DefaultTest for RelationInstance {
    fn default_test() -> Self {
        RelationInstance::builder()
            .outbound_id(Uuid::new_v4())
            .ty(RelationInstanceTypeId::default_test())
            .inbound_id(Uuid::new_v4())
            .name(r_string())
            .description(r_string())
            .properties(PropertyInstances::default_test())
            .extensions(Extensions::default_test())
            .build()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for RelationInstances {
    fn default_test() -> Self {
        let relation_instances = RelationInstances::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            relation_instances.push(RelationInstance::default_test());
        }
        relation_instances
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use serde_json::json;
    use uuid::Uuid;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIdContainer;
    use crate::ComponentTypeIds;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::MutablePropertyInstanceSetter;
    use crate::NamespacedTypeGetter;
    use crate::PropertyInstanceGetter;
    use crate::PropertyInstances;
    use crate::RelationInstance;
    use crate::RelationInstanceId;
    use crate::RelationInstanceTypeId;
    use crate::RelationTypeId;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn relation_instance_builder_test() {
        let namespace = r_string();
        let type_name = r_string();
        let ty = RelationTypeId::new_from_type(&namespace, &type_name);

        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();

        let property_1_name = r_string();
        let property_1_value = r_string();
        let properties = PropertyInstances::new().property(&property_1_name, json!(property_1_value));

        let instance_ty = RelationInstanceTypeId::new_with_random_instance_id(&ty);
        let instance_id = instance_ty.instance_id();

        let id = RelationInstanceId::new(outbound_id, &instance_ty, inbound_id);

        let relation_instance = RelationInstance::builder()
            .outbound_id(outbound_id)
            .ty(instance_ty)
            .name(r_string())
            .description(r_string())
            .inbound_id(inbound_id)
            .properties(properties)
            .build();

        assert_eq!(namespace, relation_instance.namespace());
        assert_eq!(format!("{}__{}", type_name, instance_id), relation_instance.type_name());
        assert_eq!(ty, relation_instance.relation_type_id());
        assert_eq!(id, relation_instance.id());
        assert_eq!(property_1_value.clone().as_str(), relation_instance.get(property_1_name.clone()).unwrap().as_str().unwrap());
    }

    #[test]
    fn relation_instance_test() {
        let namespace = r_string();
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let type_name = r_string();
        let name = r_string();
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

        let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
        let relation_instance = RelationInstance {
            outbound_id,
            ty: ty.clone(),
            inbound_id,
            name: name.to_string(),
            description: description.to_string(),
            properties: properties.clone(),
            components: components.clone(),
            extensions: extensions.clone(),
        };
        assert_eq!(namespace, relation_instance.namespace());
        assert_eq!(outbound_id, relation_instance.outbound_id);
        assert_eq!(type_name.clone(), relation_instance.type_name());
        assert_eq!(inbound_id, relation_instance.inbound_id);
        assert_eq!(name, relation_instance.name);
        assert_eq!(description, relation_instance.description);
        assert_eq!(properties.clone(), relation_instance.properties.clone());
        assert!(relation_instance.get(property_name.clone()).is_some());
        assert!(relation_instance.get(r_string()).is_none());
        assert_eq!(property_value.clone(), relation_instance.get(property_name.clone()).unwrap());
        assert!(relation_instance.components.contains(&component_ty.clone()));
        assert!(relation_instance.components.is_a(&component_ty));
        assert!(relation_instance.is_a(&component_ty));
        assert!(!relation_instance.is_a(&ComponentTypeId::generate_random()));
        assert!(relation_instance.extensions.has_own_extension(&extension_ty));
        assert!(relation_instance.has_own_extension(&extension_ty));
        let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
        assert!(!relation_instance.has_own_extension(&non_existing_extension));
        assert_eq!(extension.extension, relation_instance.get_own_extension(&extension_ty).unwrap().extension);

        assert_eq!(
            format!("{}-[{}]->{}", relation_instance.outbound_id, relation_instance.ty, relation_instance.inbound_id),
            format!("{}", relation_instance)
        );
    }

    #[test]
    fn relation_instance_id_from_type_unique_id_test() {
        let namespace = r_string();
        let type_name = r_string();
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(type_name, ty.type_name());
        assert_eq!(format!("r__{}__{}", namespace, type_name), ty.type_definition().to_string());
        let relation_instance = RelationInstance {
            outbound_id,
            ty: ty.clone(),
            inbound_id,
            name: r_string(),
            description: r_string(),
            properties: PropertyInstances::new(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        };

        let rity = relation_instance.id();
        assert_eq!(namespace, rity.namespace());
        assert_eq!(type_name, rity.type_name());
        assert_eq!(format!("{outbound_id}-[r__{namespace}__{type_name}]->{inbound_id}"), format!("{rity}"));

        let rty = relation_instance.relation_type_id();
        assert_eq!(namespace, rty.namespace());
        assert_eq!(type_name, rty.type_name());
        assert_eq!(format!("r__{namespace}__{type_name}"), format!("{rty}"));
    }

    #[test]
    fn relation_instance_id_from_type_unique_for_instance_id_test() {
        let namespace = r_string();
        let type_name = r_string();
        let instance_id = r_string();
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let ty = RelationInstanceTypeId::new_from_type_unique_for_instance_id(&namespace, &type_name, &instance_id);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(type_name, ty.relation_type_id().type_name());
        assert_eq!(format!("{type_name}__{instance_id}"), ty.type_name());
        assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), ty.type_definition().to_string());
        let relation_instance = RelationInstance {
            outbound_id,
            ty: ty.clone(),
            inbound_id,
            name: r_string(),
            description: r_string(),
            properties: PropertyInstances::new(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        };

        let rity = relation_instance.id();
        assert_eq!(namespace, rity.namespace());
        assert_eq!(format!("{type_name}__{instance_id}"), rity.type_name());
        assert_eq!(format!("{outbound_id}-[r__{namespace}__{type_name}__{instance_id}]->{inbound_id}"), format!("{rity}"));

        let rty = relation_instance.relation_type_id();
        assert_eq!(namespace, rty.namespace());
        assert_eq!(type_name, rty.type_name());
        assert_eq!(format!("r__{namespace}__{type_name}"), format!("{rty}"));

        assert_eq!(format!("r__{namespace}__{type_name}__{instance_id}"), format!("{}", relation_instance.ty));
    }

    #[test]
    fn create_relation_instance_test() {
        let namespace = r_string();
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let type_name = r_string();
        let property_name = r_string();
        let property_value = json!(r_string());
        let properties = PropertyInstances::new().property(&property_name, property_value.clone());
        let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
        let relation_instance = RelationInstance::new(outbound_id, ty, inbound_id, properties.clone());
        assert_eq!(namespace, relation_instance.namespace());
        assert_eq!(outbound_id, relation_instance.outbound_id);
        assert_eq!(type_name, relation_instance.type_name());
        assert_eq!(inbound_id, relation_instance.inbound_id);
        assert_eq!(properties.clone(), relation_instance.properties.clone());
        assert!(relation_instance.get(property_name.clone()).is_some());
        assert!(relation_instance.get(r_string()).is_none());
        assert_eq!(property_value.clone(), relation_instance.get(property_name.clone()).unwrap());
    }

    #[test]
    fn create_relation_instance_without_properties_test() {
        let namespace = r_string();
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let type_name = r_string();
        let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
        let relation_instance = RelationInstance::new_without_properties(outbound_id, ty.clone(), inbound_id);
        assert_eq!(namespace, relation_instance.namespace());
        assert_eq!(outbound_id, relation_instance.outbound_id);
        assert_eq!(type_name, relation_instance.type_name());
        assert_eq!(inbound_id, relation_instance.inbound_id);
        assert_eq!(0, relation_instance.properties.len());
    }

    #[test]
    fn relation_instance_typed_getter_test() {
        let namespace = r_string();
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let type_name = r_string();
        let property_name = r_string();
        let properties = PropertyInstances::new().property(&property_name, json!(false));
        let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
        let mut i = RelationInstance::new(outbound_id, ty.clone(), inbound_id, properties.clone());
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
        i.set(property_name.clone(), json!([]));
        assert_eq!(0, i.as_array(property_name.clone()).unwrap().len());
        i.set(property_name.clone(), json!({}));
        assert_eq!(0, i.as_object(property_name.clone()).unwrap().len());
    }

    #[test]
    fn relation_instance_get_key_test() {
        let namespace = r_string();
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let type_name = r_string();
        let name = r_string();
        let description = r_string();
        let ty = RelationInstanceTypeId::new_from_type_unique_id(&namespace, &type_name);
        let relation_instance = RelationInstance {
            outbound_id,
            ty: ty.clone(),
            inbound_id,
            name: name.to_string(),
            description: description.to_string(),
            properties: PropertyInstances::new(),
            components: ComponentTypeIds::new(),
            extensions: Extensions::new(),
        };

        assert_eq!(ty, relation_instance.ty);
        assert_eq!(ty.relation_type_id(), relation_instance.relation_type_id());
    }

    #[test]
    fn relation_instance_ser_test() {
        let rty = RelationTypeId::new_from_type("rnr", "rtr");
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(rty.clone(), "result__lhs");
        let outbound_id = Uuid::new_v4();
        let inbound_id = Uuid::new_v4();
        let relation_instance = RelationInstance::new(outbound_id, ty, inbound_id, PropertyInstances::new());
        println!("{}", serde_json::to_string_pretty(&relation_instance).expect("Failed to serialize relation instance"));
    }
    #[test]
    fn relation_instance_de_test() {
        let s = r#"{
  "outbound_id": "d82cc81a-e0e5-4de8-8b87-9b5bed0de795",
  "namespace": "rnr",
  "type_name": "rtr",
  "instance_id": "result__lhs",
  "inbound_id": "3f13400e-9286-441d-b85f-ef5df2177e7c",
  "name": "BIVS93iu",
  "description": "B0IgcIiV",
  "components": [
    {
      "namespace": "mno",
      "type_name": "pqr"
    }
  ],
  "properties": {
      "property_name": "property_value"
  },
  "extensions": [
    {
      "namespace": "ext_namespace",
      "type_name": "ext_name",
      "extension": "ext_value"
    },
    {
      "namespace": "other_ext_namespace",
      "type_name": "other_ext_name",
      "extension": "other_extension_value"
    }
  ]
}"#;
        let relation_instance: RelationInstance = serde_json::from_str(s).unwrap();
        assert_eq!("d82cc81a-e0e5-4de8-8b87-9b5bed0de795", relation_instance.outbound_id.to_string());
        assert_eq!("3f13400e-9286-441d-b85f-ef5df2177e7c", relation_instance.inbound_id.to_string());
        assert_eq!("rnr", relation_instance.namespace());
        assert_eq!("rtr__result__lhs", relation_instance.type_name());
        assert_eq!("rtr", relation_instance.relation_type_id().type_name());
        assert_eq!("result__lhs", relation_instance.instance_id());
        assert_eq!("r__rnr__rtr__result__lhs", relation_instance.ty.to_string());
        assert_eq!(TypeIdType::RelationType, relation_instance.type_definition().type_id_type);
        assert_eq!("BIVS93iu", relation_instance.name);
        assert_eq!("B0IgcIiV", relation_instance.description);
        assert_eq!("property_value", relation_instance.properties.get("property_name").unwrap().as_str().unwrap());
        assert_eq!(2, relation_instance.extensions.len());
        assert!(
            relation_instance
                .extensions
                .has_own_extension(&ExtensionTypeId::new_from_type("ext_namespace", "ext_name"))
        );
        assert_eq!(
            json!("ext_value"),
            relation_instance
                .extensions
                .get_own_extension(&ExtensionTypeId::new_from_type("ext_namespace", "ext_name"))
                .unwrap()
                .extension
        );
        assert!(
            relation_instance
                .extensions
                .has_own_extension(&ExtensionTypeId::new_from_type("other_ext_namespace", "other_ext_name"))
        );
        assert_eq!(
            json!("other_extension_value"),
            relation_instance
                .extensions
                .get_own_extension(&ExtensionTypeId::new_from_type("other_ext_namespace", "other_ext_name"))
                .unwrap()
                .extension
        );
    }

    #[test]
    fn relation_instance_json_schema() {
        let schema = schema_for!(RelationInstance);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
