use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::iter::OwningIter;
use dashmap::mapref::multiple::RefMulti;
use dashmap::DashMap;
use schemars::gen::SchemaGenerator;
use schemars::schema::ArrayValidation;
use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
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
use crate::TypeIdType;
use crate::UpdateExtensionError;
use crate::UpdatePropertyError;

/// A component defines a set of properties to be applied to entity
/// types and relation types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct Component {
    /// The type definition of the component.
    #[serde(flatten)]
    #[builder(setter(into))]
    pub ty: ComponentTypeId,

    /// Textual description of the component.
    #[serde(default = "String::new")]
    #[builder(default, setter(into))]
    pub description: String,

    /// The properties which are applied on entity or relation instances.
    #[serde(default = "PropertyTypes::new")]
    #[builder(default, setter(into))]
    pub properties: PropertyTypes,

    /// Component specific extensions
    #[serde(default = "Extensions::new")]
    #[builder(default, setter(into))]
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
    pub fn new_from_type<S: Into<String>, T: Into<String>, D: Into<String>, P: Into<PropertyTypes>, E: Into<Extensions>>(
        namespace: S,
        type_name: T,
        description: D,
        properties: P,
        extensions: E,
    ) -> Component {
        Component {
            ty: ComponentTypeId::new_from_type(namespace.into(), type_name.into()),
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
        self.properties.merge_properties(properties_to_merge);
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
}

impl NamespacedTypeGetter for Component {
    fn namespace(&self) -> String {
        self.ty.namespace()
    }

    fn type_name(&self) -> String {
        self.ty.type_name()
    }
}

impl TypeDefinitionGetter for Component {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.type_definition()
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
        TypeDefinition {
            type_id_type: TypeIdType::Component,
            namespace: component.ty.namespace(),
            type_name: component.ty.type_name(),
        }
    }
}

impl From<&Component> for ComponentTypeId {
    fn from(component: &Component) -> Self {
        component.ty.clone()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Components(DashMap<ComponentTypeId, Component>);

impl Components {
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
        self.iter().map(|component| component.namespace()).collect()
    }

    pub fn get<T: Into<ComponentTypeId>>(&self, ty: T) -> Option<Component> {
        let ty = ty.into();
        self.0.iter().find(|component| component.ty == ty).map(|component| component.clone())
    }

    pub fn get_by_namespace<N: Into<String>>(&self, namespace: N) -> Components {
        let namespace = namespace.into();
        self.filter_by(|component| component.ty.namespace() == namespace)
    }

    pub fn get_by_types<C: Into<ComponentTypeIds>>(&self, tys: C) -> Components {
        let tys = tys.into();
        self.filter_by(|component| tys.contains(&component.ty))
    }

    pub fn find_by_type_name(&self, search: &str) -> Components {
        let matcher = WildMatch::new(search);
        self.filter_by(|component| matcher.matches(component.type_name().as_str()))
    }

    pub fn count_by_namespace<N: Into<String>>(&self, namespace: N) -> usize {
        let namespace = namespace.into();
        self.count_by(|component| component.ty.namespace() == namespace)
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
}

impl NamespacedTypeContainer for Components {
    type TypeId = ComponentTypeId;
    type TypeIds = ComponentTypeIds;
    type Type = Component;

    fn new() -> Self {
        Self(DashMap::new())
    }

    fn push<C: Into<Component>>(&self, component: C) {
        let component = component.into();
        self.insert(component.ty.clone(), component);
    }
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

impl Eq for Components {}

impl Hash for Components {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for Components {
    fn schema_name() -> String {
        "Components".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<Component>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
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
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use reactive_graph_test_utils::r_string;
#[cfg(any(test, feature = "test"))]
use rand::Rng;

#[cfg(any(test, feature = "test"))]
impl DefaultTest for Component {
    fn default_test() -> Self {
        Component::builder()
            .ty(ComponentTypeId::default_test())
            .description(r_string())
            .properties(PropertyTypes::default_test())
            .extensions(Extensions::default_test())
            .build()
    }
}
#[cfg(any(test, feature = "test"))]
impl DefaultTest for Components {
    fn default_test() -> Self {
        let components = Components::new();
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
            components.push(Component::default_test());
        }
        components
    }
}

#[cfg(test)]
mod component_type_tests {
    use schemars::schema_for;
    use serde_json::json;

    use crate::Component;
    use crate::ComponentTypeId;
    use crate::DataType;
    use crate::Extension;
    use crate::ExtensionContainer;
    use crate::ExtensionTypeId;
    use crate::Extensions;
    use crate::NamespacedTypeGetter;
    use crate::PropertyType;
    use crate::PropertyTypeContainer;
    use crate::PropertyTypes;
    use crate::TypeDefinitionGetter;
    use reactive_graph_test_utils::r_string;

    #[test]
    fn build_component() {
        let component = Component::builder()
            .ty(ComponentTypeId::new_from_type("x", "y"))
            .description("d")
            .properties(Vec::new())
            .extensions(Vec::new())
            .build();
        assert_eq!("x", component.namespace());
        assert_eq!("y", component.type_name());
        assert_eq!("d", component.description);
        assert_eq!(0, component.properties.len());
        assert_eq!(0, component.extensions.len());
    }

    #[test]
    fn component_test() {
        let namespace = r_string();
        let component_name = r_string();
        let description = r_string();
        let property_name = r_string();
        let properties = PropertyTypes::new_with_string_property(&property_name);
        // let property_type = PropertyType::new(&property_name, DataType::String);
        // property_types.push(property_type.clone());

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

        let extensions = Extensions::new().extension(extension).extension(other_extension);

        let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        let component = Component {
            ty,
            description: description.clone(),
            properties,
            extensions,
        };

        assert_eq!(namespace, component.namespace());
        assert_eq!(component_name, component.type_name());
        assert_eq!(format!("c__{}__{}", &namespace, &component_name), component.type_definition().to_string());
        assert_eq!(description, component.description);
        assert!(component.has_own_extension(&extension_ty));
        assert_eq!(extension_value, component.get_own_extension(&extension_ty).unwrap().extension);
        let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
        assert!(!component.has_own_extension(&non_existing_extension));

        let component_2 = component.clone();
        assert_eq!(component_2.type_name(), component.type_name());
    }

    #[test]
    fn create_new_component_test() {
        let namespace = r_string();
        let component_name = r_string();
        let description = r_string();
        let mut property_types = Vec::new();
        let property_name = r_string();
        let property_type = PropertyType::new(property_name.clone(), DataType::String);
        property_types.push(property_type.clone());

        let mut extensions = Vec::new();
        let extension_namespace = r_string();
        let extension_name = r_string();
        let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
        let extension_value = json!("extension_value");
        let extension = Extension {
            ty: extension_ty.clone(),
            description: r_string(),
            extension: extension_value.clone(),
        };
        extensions.push(extension);
        let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
        let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
        extensions.push(other_extension);

        let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        let component = Component::new(ty, description.clone(), property_types.clone(), extensions);
        assert_eq!(namespace, component.namespace());
        assert_eq!(component_name, component.type_name());
        assert!(component.has_own_property(&property_name));
        assert_eq!(property_type, component.get_own_property(&property_name).unwrap());
        // assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
        // assert_eq!(property_type.data_type, component.properties.first().unwrap().data_type);
        assert!(!component.properties.iter().filter(|p| p.key() == &property_name).collect::<Vec<_>>().is_empty());
        assert!(component.has_own_property(property_name.clone()));
        assert!(!component.has_own_property(r_string()));
    }

    #[test]
    fn create_new_component_without_properties_test() {
        let namespace = r_string();
        let component_name = r_string();

        let mut extensions = Vec::new();
        let extension_namespace = r_string();
        let extension_name = r_string();
        let extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &extension_name);
        let extension_value = json!("extension_value");
        let extension = Extension {
            ty: extension_ty.clone(),
            description: r_string(),
            extension: extension_value.clone(),
        };
        extensions.push(extension);
        let other_extension_ty = ExtensionTypeId::new_from_type(&extension_namespace, &r_string());
        let other_extension = Extension::new(&other_extension_ty, r_string(), extension_value.clone());
        extensions.push(other_extension);

        let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        let component = Component::new_without_properties(ty, r_string(), extensions.clone());
        assert_eq!(namespace, component.namespace());
        assert_eq!(component_name, component.type_name());

        assert!(component.extensions.contains_key(&extension_ty));
        assert!(component.has_own_extension(&extension_ty));
        assert_eq!(extension_value, component.get_own_extension(&extension_ty).unwrap().extension);

        let non_existing_extension = ExtensionTypeId::new_from_type(r_string(), r_string());
        assert!(!component.extensions.contains_key(&non_existing_extension));
        assert!(!component.has_own_extension(&non_existing_extension));
    }

    #[test]
    fn create_component_without_extensions_test() {
        let component_name = r_string();
        let namespace = r_string();

        let property_name = r_string();
        let mut property_types = Vec::new();
        let property_type = PropertyType::new(property_name.clone(), DataType::String);
        property_types.push(property_type.clone());

        let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        let component = Component::new_without_extensions(ty, r_string(), property_types);
        assert_eq!(namespace, component.namespace());
        assert_eq!(component_name, component.type_name());
        assert!(component.has_own_property(&property_name));
        assert_eq!(property_type, component.get_own_property(&property_name).unwrap());
        // assert_eq!(property_name.clone(), component.properties.first().unwrap().name);
        // assert_eq!(property_type.data_type, component.properties.first().unwrap().data_type);
        assert!(!component.properties.iter().filter(|p| p.key() == &property_name).collect::<Vec<_>>().is_empty());
    }

    #[test]
    fn component_has_property_test() {
        let namespace = r_string();
        let component_name = r_string();
        let mut property_types = Vec::new();
        let property_name = r_string();
        let property_type = PropertyType::new(property_name.clone(), DataType::String);
        property_types.push(property_type.clone());
        let ty = ComponentTypeId::new_from_type(&namespace, &component_name);
        let component = Component::new(ty, r_string(), property_types.clone(), Vec::new());
        assert!(component.has_own_property(property_name));
        assert!(!component.has_own_property(r_string()));
    }

    #[test]
    fn component_type_ser_test() {
        let ty = ComponentTypeId::new_from_type("cnc", "ctc");
        let c = Component::new(ty, "d", Vec::new(), Vec::new());
        println!("{}", serde_json::to_string_pretty(&c).expect("Failed to serialize component"));
    }

    #[test]
    fn component_ser_test() {
        // TODO: rename "type_name" to "name"
        // https://github.com/serde-rs/serde/pull/2160
        // https://github.com/serde-rs/serde/issues/1504)
        let s = r#"{
  "namespace": "abc",
  "type_name": "def",
  "description": "d",
  "properties": [
    {
      "name": "property_name",
      "data_type": "string",
      "socket_type": "input"
    }
  ],
  "extensions": []
}"#;
        let component: Component = serde_json::from_str(s).unwrap();
        assert_eq!("abc", component.namespace());
        assert_eq!("def", component.type_name());
        assert_eq!("c__abc__def", component.ty.to_string());
        assert_eq!("d", component.description);
        assert_eq!(1, component.properties.len());
        assert_eq!(0, component.extensions.len());
    }

    #[test]
    fn component_json_schema() {
        let schema = schema_for!(Component);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
