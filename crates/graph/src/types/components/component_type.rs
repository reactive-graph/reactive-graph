use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use const_format::formatcp;
use dashmap::DashMap;
use dashmap::iter::OwningIter;
use dashmap::mapref::multiple::RefMulti;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use typed_builder::TypedBuilder;
use wildmatch::WildMatch;

use crate::AddExtensionError;
use crate::AddPropertyError;
use crate::ComponentMergeError;
use crate::ComponentTypeId;
use crate::ComponentTypeIds;
use crate::Extension;
use crate::ExtensionContainer;
use crate::ExtensionTypeId;
use crate::Extensions;
use crate::JSON_SCHEMA_ID_URI_PREFIX;
use crate::Namespace;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeContainer;
use crate::NamespacedTypeGetter;
use crate::Namespaces;
use crate::PropertyType;
use crate::PropertyTypeContainer;
use crate::PropertyTypes;
use crate::RemoveExtensionError;
use crate::RemovePropertyError;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeDefinitionJsonSchema;
use crate::TypeDefinitionJsonSchemaGetter;
use crate::TypeDescriptionGetter;
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;
use crate::divergent::DivergentPropertyTypes;

#[cfg(any(test, feature = "test"))]
use crate::NamespacedTypeError;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildType;
#[cfg(any(test, feature = "test"))]
use crate::RandomChildTypeId;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedType;
#[cfg(any(test, feature = "test"))]
use crate::RandomNamespacedTypes;
#[cfg(any(test, feature = "test"))]
use reactive_graph_utils_test::r_string;
#[cfg(any(test, feature = "test", feature = "table"))]
use tabled::Tabled;

pub const JSON_SCHEMA_ID_COMPONENT: &str = formatcp!("{}/component.schema.json", JSON_SCHEMA_ID_URI_PREFIX);

/// A component defines a set of properties to be applied to entity
/// types and relation types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[cfg_attr(any(test, feature = "table"), derive(Tabled))]
#[serde(tag = "$id", rename = "https://schema.reactive-graph.io/schema/json/component.schema.json")]
#[schemars(
    title = "Component",
    deny_unknown_fields,
    extend("$id" = JSON_SCHEMA_ID_COMPONENT),
    transform = add_json_schema_id_property
)]
pub struct Component {
    /// The type definition of the component.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub ty: ComponentTypeId,

    /// Textual description of the component.
    #[serde(default = "String::new")]
    #[schemars(required)]
    #[builder(default, setter(into))]
    pub description: String,

    /// The properties which are applied on entity or relation instances.
    #[serde(default = "PropertyTypes::new")]
    #[schemars(required)]
    #[builder(default, setter(into))]
    #[cfg_attr(any(test, feature = "table"), tabled(skip))]
    pub properties: PropertyTypes,

    /// Component specific extensions
    #[serde(default = "Extensions::new")]
    #[schemars(required)]
    #[builder(default, setter(into))]
    #[cfg_attr(any(test, feature = "table"), tabled(skip))]
    pub extensions: Extensions,
}

impl Component {
    pub fn new<T: Into<ComponentTypeId>, D: Into<String>, P: Into<PropertyTypes>, E: Into<Extensions>>(
        ty: T,
        description: D,
        properties: P,
        extensions: E,
    ) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties: properties.into(),
            extensions: extensions.into(),
        }
    }

    /// Constructs a new component with the given name and properties
    pub fn new_without_extensions<T: Into<ComponentTypeId>, D: Into<String>, P: Into<PropertyTypes>>(ty: T, description: D, properties: P) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties: properties.into(),
            extensions: Extensions::new(),
        }
    }

    /// Constructs an component with the given name but without properties
    pub fn new_without_properties<T: Into<ComponentTypeId>, D: Into<String>, E: Into<Extensions>>(ty: T, description: D, extensions: E) -> Component {
        Component {
            ty: ty.into(),
            description: description.into(),
            properties: PropertyTypes::new(),
            extensions: extensions.into(),
        }
    }
}

impl PropertyTypeContainer for Component {
    fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
        self.properties.has_own_property(property_name)
    }

    fn get_own_property<S: Into<String>>(&self, property_name: S) -> Option<PropertyType> {
        self.properties.get_own_property(property_name)
    }

    fn add_property<S: Into<PropertyType>>(&self, property_type: S) -> Result<PropertyType, AddPropertyError> {
        self.properties.add_property(property_type)
    }

    fn update_property<N: Into<String>, S: Into<PropertyType>>(&self, property_name: N, property_type: S) -> Result<PropertyType, UpdatePropertyError> {
        self.properties.update_property(property_name, property_type)
    }

    fn remove_property<S: Into<String>>(&self, property_name: S) -> Result<PropertyType, RemovePropertyError> {
        self.properties.remove_property(property_name)
    }

    fn merge_properties<P: Into<PropertyTypes>>(&mut self, properties_to_merge: P) {
        self.properties.merge_properties(properties_to_merge)
    }

    fn merge_non_existent_properties<P: Into<PropertyTypes>>(&self, properties_to_merge: P) -> DivergentPropertyTypes {
        self.properties.merge_non_existent_properties(properties_to_merge)
    }

    fn get_own_properties_cloned(&self) -> PropertyTypes {
        self.properties.clone()
    }
}

impl ExtensionContainer for Component {
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

impl NamespacedTypeGetter for Component {
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

impl TypeDefinitionGetter for Component {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::Component
    }
}

impl TypeDescriptionGetter for Component {
    fn description(&self) -> String {
        self.description.clone()
    }
}

impl TypeDefinitionJsonSchemaGetter for Component {
    fn json_schema(&self) -> Schema {
        TypeDefinitionJsonSchema::new(self).description(&self.description).into()
    }
}

impl AsRef<ComponentTypeId> for Component {
    fn as_ref(&self) -> &ComponentTypeId {
        &self.ty
    }
}

impl PartialEq<ComponentTypeId> for Component {
    fn eq(&self, ty: &ComponentTypeId) -> bool {
        self.ty == *ty
    }
}

impl PartialOrd<Self> for Component {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Component {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty.cmp(&other.ty)
    }
}

impl From<Component> for TypeDefinition {
    fn from(component: Component) -> Self {
        component.type_definition()
    }
}

impl From<&Component> for ComponentTypeId {
    fn from(component: &Component) -> Self {
        component.ty.clone()
    }
}

impl From<&Component> for Schema {
    fn from(component: &Component) -> Self {
        component.json_schema()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Components(DashMap<ComponentTypeId, Component>);

impl Components {
    #[inline]
    pub fn new() -> Self {
        NamespacedTypeContainer::new()
    }

    #[inline]
    pub fn component<C: Into<Component>>(self, component: C) -> Self {
        NamespacedTypeContainer::push(&self, component);
        self
    }

    #[inline]
    pub fn push<C: Into<Component>>(&self, component: C) -> Option<Component> {
        NamespacedTypeContainer::push(self, component)
    }

    pub fn merge<C: Into<Component>>(&self, component_to_merge: C) -> Result<Component, ComponentMergeError> {
        let component_to_merge = component_to_merge.into();
        let Some(mut component) = self.get_mut(&component_to_merge.ty) else {
            return Err(ComponentMergeError::ComponentDoesNotExist(component_to_merge.ty));
        };
        component.description = component_to_merge.description;
        component.merge_properties(component_to_merge.properties);
        component.merge_extensions(component_to_merge.extensions);
        Ok(component.clone())
    }

    pub fn namespaces(&self) -> Namespaces {
        self.iter().map(|component| component.path()).collect()
    }

    pub fn get<T: Into<ComponentTypeId>>(&self, ty: T) -> Option<Component> {
        let ty = ty.into();
        self.0.iter().find(|component| component.ty == ty).map(|component| component.clone())
    }

    pub fn get_by_namespace<N: Into<Namespace>>(&self, namespace: N) -> Components {
        let namespace = namespace.into();
        self.filter_by(|component| component.ty.path() == namespace)
    }

    pub fn get_by_types<C: Into<ComponentTypeIds>>(&self, tys: C) -> Components {
        let tys = tys.into();
        self.filter_by(|component| tys.contains(&component.ty))
    }

    pub fn find_by_type_name(&self, search: &str) -> Components {
        let matcher = WildMatch::new(search);
        self.filter_by(|component| matcher.matches(component.type_name().as_ref()))
    }

    pub fn count_by_namespace<N: Into<Namespace>>(&self, namespace: N) -> usize {
        let namespace = namespace.into();
        self.count_by(|component| component.ty.path() == namespace)
    }

    #[inline]
    fn filter_by<P>(&self, predicate: P) -> Components
    where
        P: FnMut(&RefMulti<ComponentTypeId, Component>) -> bool,
    {
        self.0.iter().filter(predicate).map(|component| component.clone()).collect()
    }

    #[inline]
    fn count_by<P>(&self, predicate: P) -> usize
    where
        P: FnMut(&RefMulti<ComponentTypeId, Component>) -> bool,
    {
        self.0.iter().filter(predicate).count()
    }

    #[inline]
    pub fn push_all<C: Into<Self>>(&self, components: C) {
        NamespacedTypeContainer::push_all(self, components.into())
    }
}

impl NamespacedTypeContainer for Components {
    type TypeId = ComponentTypeId;
    type TypeIds = ComponentTypeIds;
    type Type = Component;
}

impl Deref for Components {
    type Target = DashMap<ComponentTypeId, Component>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Components {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Components {
    type Item = (ComponentTypeId, Component);
    type IntoIter = OwningIter<ComponentTypeId, Component>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for Components {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().all(|self_component| other.contains_key(&self_component.ty)) && other.iter().all(|other_component| self.contains_key(&other_component.ty))
    }
}

impl Hash for Components {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl Eq for Components {}

impl JsonSchema for Components {
    fn schema_name() -> Cow<'static, str> {
        "Components".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<Component>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
            "description": "Components",
        })
    }
}

impl From<Vec<Component>> for Components {
    fn from(components: Vec<Component>) -> Self {
        Self(components.into_iter().map(|component| (component.ty.clone(), component)).collect())
    }
}

impl From<Components> for Vec<Component> {
    fn from(tys: Components) -> Self {
        tys.to_vec()
    }
}

impl From<&Components> for Vec<Component> {
    fn from(tys: &Components) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashMap<ComponentTypeId, Component>> for Components {
    fn from(components: DashMap<ComponentTypeId, Component>) -> Self {
        Self(components)
    }
}

impl From<&DashMap<ComponentTypeId, Component>> for Components {
    fn from(components: &DashMap<ComponentTypeId, Component>) -> Self {
        Self(components.clone())
    }
}

impl From<Components> for DashMap<ComponentTypeId, Component> {
    fn from(tys: Components) -> Self {
        tys.0
    }
}

impl FromIterator<Component> for Components {
    fn from_iter<I: IntoIterator<Item = Component>>(iter: I) -> Self {
        let components = Components::new();
        for component in iter {
            components.insert(component.ty.clone(), component);
        }
        components
    }
}

#[macro_export]
macro_rules! component_model {
    (
        $ident: ident
        $(,
            $accessor_type: tt
            $(
            $accessor_name: ident
            $accessor_data_type: tt
            )?
        )*
        $(,)?
    ) => {
        pub trait $ident: $crate::PropertyInstanceGetter + $crate::PropertyInstanceSetter {
            $(
                $crate::rx_accessor!($accessor_type $($accessor_name $accessor_data_type)?);
            )*
        }
    };
}

#[cfg(any(test, feature = "test"))]
impl RandomNamespacedType for Component {
    type Error = NamespacedTypeError;
    type TypeId = ComponentTypeId;

    fn random_type() -> Result<Self, NamespacedTypeError> {
        Self::random_child_type(&Namespace::random_path()?)
    }

    fn random_type_with_id(ty: &ComponentTypeId) -> Result<Self, Self::Error> {
        Ok(Self::builder()
            .ty(ty)
            .description(r_string())
            .properties(PropertyTypes::random_types(0..10)?)
            .extensions(Extensions::random_types(0..10)?)
            .build())
    }
}

#[cfg(any(test, feature = "test"))]
impl RandomChildType for Component {
    type Error = NamespacedTypeError;

    fn random_child_type(namespace: &Namespace) -> Result<Self, Self::Error> {
        Self::random_type_with_id(&NamespacedType::random_child_type_id(namespace)?.into())
    }
}

fn add_json_schema_id_property(schema: &mut Schema) {
    crate::json_schema::add_json_schema_id_property(schema, JSON_SCHEMA_ID_COMPONENT);
}

#[cfg(test)]
mod component_type_tests {
    use schemars::schema_for;
    use std::str::FromStr;

    use crate::Component;
    use crate::ComponentTypeId;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::NamespacedTypeGetter;
    use crate::PropertyType;
    use crate::PropertyTypeContainer;
    use crate::PropertyTypes;
    use crate::RandomNamespacedType;
    use crate::RandomNamespacedTypeId;
    use crate::RandomNamespacedTypes;
    use crate::TypeDefinitionJsonSchemaGetter;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn build_component() {
        let description = r_string();
        let ty = ComponentTypeId::random_type_id().unwrap();
        let component = Component::builder()
            .ty(&ty)
            .description(&description)
            .properties(Vec::new())
            .extensions(Vec::new())
            .build();
        assert_eq!(ty.namespace(), component.namespace());
        assert_eq!(ty.path(), component.path());
        assert_eq!(ty.type_name(), component.type_name());
        assert_eq!(description, component.description);
        assert_eq!(0, component.properties.len());
        assert_eq!(0, component.extensions.len());
    }

    #[test]
    fn create_component() {
        let ty = ComponentTypeId::random_type_id().unwrap();
        let description = r_string();
        let properties = PropertyTypes::random_types_no_extensions();
        let extensions = Extensions::random_types(1..10).unwrap();
        let component = Component::new(&ty, &description, properties.clone(), extensions.clone());
        assert_eq!(ty.namespace(), component.namespace());
        assert_eq!(ty.path(), component.path());
        assert_eq!(ty.type_name(), component.type_name());
        assert_eq!(description, component.description);
        assert_eq!(properties.len(), component.properties.len());
        assert_eq!(extensions.len(), component.extensions.len());
        assert!(!component.has_own_property(r_string()));
        assert!(!component.has_own_extension(&ExtensionTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn create_component_without_properties_test() {
        let ty = ComponentTypeId::random_type_id().unwrap();
        let description = r_string();
        let extensions = Extensions::random_types(1..10).unwrap();
        let component = Component::new_without_properties(&ty, &description, extensions.clone());
        assert_eq!(ty.namespace(), component.namespace());
        assert_eq!(ty.path(), component.path());
        assert_eq!(ty.type_name(), component.type_name());
        assert_eq!(description, component.description);
        assert_eq!(0, component.properties.len());
        assert_eq!(extensions.len(), component.extensions.len());
        assert!(!component.has_own_property(r_string()));
        assert!(!component.has_own_extension(&ExtensionTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn create_component_without_extensions_test() {
        let ty = ComponentTypeId::random_type_id().unwrap();
        let description = r_string();
        let properties = PropertyTypes::random_types_no_extensions();
        let component = Component::new_without_extensions(&ty, &description, properties.clone());
        assert_eq!(ty.namespace(), component.namespace());
        assert_eq!(ty.path(), component.path());
        assert_eq!(ty.type_name(), component.type_name());
        assert_eq!(description, component.description);
        assert_eq!(properties.len(), component.properties.len());
        assert_eq!(0, component.extensions.len());
        assert!(!component.has_own_property(r_string()));
        assert!(!component.has_own_extension(&ExtensionTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn component_has_property_test() {
        let property_name = r_string();
        let properties = PropertyTypes::new().property(PropertyType::string(&property_name));
        let component = Component::builder()
            .ty(ComponentTypeId::random_type_id().unwrap())
            .properties(properties)
            .build();
        assert_eq!(1, component.properties.len());
        assert!(component.has_own_property(property_name));
        assert!(!component.has_own_property(r_string()));
    }

    #[test]
    fn component_has_extension_test() {
        let extension = Extension::random_type().unwrap();
        let extensions = Extensions::new().extension(extension.clone());
        let component = Component::builder()
            .ty(ComponentTypeId::random_type_id().unwrap())
            .extensions(extensions)
            .build();
        assert_eq!(1, component.extensions.len());
        assert!(component.has_own_extension(&extension.ty));
        assert!(!component.has_own_extension(&ExtensionTypeId::random_type_id().unwrap()));
    }

    #[test]
    fn component_type_deserialize_fully_valid_test() {
        let component_ty = ComponentTypeId::from_str("fully::qualified::namespace::Component").unwrap();
        let extension_ty = ExtensionTypeId::from_str("fully::qualified::namespace::Extension").unwrap();
        let component = serde_json::from_str::<Component>(
            r#"{
          "type": "fully::qualified::namespace::Component",
          "description": "d",
          "properties": [
                {
                  "name": "property_name",
                  "data_type": "string",
                  "socket_type": "input"
                }
          ],
          "extensions": [
                {
                  "type": "fully::qualified::namespace::Extension",
                  "extension": ""
                }
          ]
        }"#,
        )
        .expect("Failed to deserialize component");
        assert_eq!(component_ty, component.ty);
        assert_eq!("d", component.description);
        assert_eq!(1, component.properties.len());
        assert!(component.get_own_property("property_name").is_some());
        assert_eq!(1, component.extensions.len());
        assert!(component.get_own_extension(&extension_ty).is_some());
    }

    #[test]
    fn component_type_deserialize_description_optional_test() {
        assert!(
            serde_json::from_str::<Component>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "properties": [],
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize component")
            .description
            .is_empty()
        );
    }

    #[test]
    fn component_type_deserialize_properties_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<Component>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "extensions": []
        }"#
            )
            .expect("Failed to deserialize component")
            .properties
            .len()
        );
    }

    #[test]
    fn component_type_deserialize_invalid_property_test() {
        assert!(
            serde_json::from_str::<Component>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "properties": [
                {
                  "name": "property_name",
                  "data_type": "strng",
                  "socket_type": "put"
                }
          ],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn component_type_deserialize_extensions_optional_test() {
        assert_eq!(
            0,
            serde_json::from_str::<Component>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "properties": []
        }"#
            )
            .expect("Failed to deserialize component")
            .extensions
            .len()
        );
    }

    #[test]
    fn component_type_deserialize_invalid_extension_test() {
        assert!(
            serde_json::from_str::<Component>(
                r#"{
          "type": "fully::qualified::namespace::Type",
          "description": "d",
          "properties": [],
          "extensions": [
                {
                  "type": "InvalidTypeName::namespace",
                  "extension": ""
                }
          ]
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn component_type_deserialize_invalid_namespace_test() {
        assert!(
            serde_json::from_str::<Component>(
                r#"{
          "type": "invalid::namespace",
          "description": "d",
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn component_type_deserialize_invalid_type_test() {
        assert!(
            serde_json::from_str::<Component>(
                r#"{
          "type": "InvalidTypeName",
          "description": "d",
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn component_type_deserialize_missing_type_test() {
        assert!(
            serde_json::from_str::<Component>(
                r#"{
          "description": "d",
          "properties": [],
          "extensions": []
        }"#
            )
            .inspect_err(|e| println!("{}", e))
            .is_err()
        );
    }

    #[test]
    fn component_type_ser_test() {
        let component = Component::random_type().unwrap();
        println!("{}", serde_json::to_string_pretty(&component).expect("Failed to serialize component"));
    }

    #[test]
    fn component_json_schema() {
        let schema = schema_for!(Component);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn component_dynamic_json_schema() {
        let component = Component::random_type().unwrap();
        let schema = component.json_schema();
        println!("{}", serde_json::to_string_pretty(schema.as_value()).unwrap());
    }
}
