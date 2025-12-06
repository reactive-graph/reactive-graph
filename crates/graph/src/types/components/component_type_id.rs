use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;

use crate::Component;
use crate::ComponentBuilder;
use crate::ComponentTypeIdContainer;
use crate::NAMESPACED_TYPE_REGEX;
use crate::Namespace;
use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeConstructor;
use crate::NamespacedTypeError;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeIdContainer;
use crate::NamespacedTypeIds;
use crate::NamespacedTypeIdsError;
use crate::NamespacedTypeParseError;
use crate::TYPE_DEFINITION_REGEX;
use crate::TypeDefinition;
use crate::TypeDefinitionConversionError;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

#[cfg(any(test, feature = "table"))]
use tabled::Tabled;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "table"), derive(Tabled))]
#[schemars(deny_unknown_fields)]
pub struct ComponentTypeId(#[cfg_attr(any(test, feature = "table"), tabled(inline))] NamespacedType);

impl ComponentTypeId {
    #[inline]
    pub fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        NamespacedTypeConstructor::new(nt)
    }

    #[inline]
    pub fn parse_str(namespace: &str) -> Result<Self, NamespacedTypeError> {
        NamespacedTypeConstructor::parse_namespace(namespace)
    }

    #[inline]
    pub fn parse_optional_namespace(namespace: Option<String>) -> Result<Option<Self>, NamespacedTypeError> {
        NamespacedTypeConstructor::parse_optional_namespace(namespace)
    }

    #[inline]
    pub fn into_builder(self) -> ComponentBuilder<((Self,), (), (), ())> {
        Component::builder().ty(self)
    }
}

impl NamespacedTypeConstructor for ComponentTypeId {
    fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        Self(nt.into())
    }
}

impl NamespacedTypeGetter for ComponentTypeId {
    fn namespaced_type(&self) -> NamespacedType {
        self.0.clone()
    }

    fn namespace(&self) -> Namespace {
        self.0.namespace.clone()
    }

    fn path(&self) -> Namespace {
        self.0.path.clone()
    }

    fn type_name(&self) -> NamespaceSegment {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for ComponentTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::Component
    }
}

impl AsRef<NamespacedType> for ComponentTypeId {
    fn as_ref(&self) -> &NamespacedType {
        &self.0
    }
}

impl AsRef<Namespace> for ComponentTypeId {
    fn as_ref(&self) -> &Namespace {
        &self.0.namespace
    }
}

impl From<&ComponentTypeId> for ComponentTypeId {
    fn from(ty: &ComponentTypeId) -> Self {
        ty.clone()
    }
}

impl From<&ComponentTypeId> for TypeDefinition {
    fn from(ty: &ComponentTypeId) -> Self {
        TypeDefinition::new(TypeIdType::Component, ty.0.clone())
    }
}

impl From<&ComponentTypeId> for NamespacedType {
    fn from(ty: &ComponentTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for ComponentTypeId {
    fn from(nt: NamespacedType) -> Self {
        Self(nt)
    }
}

impl From<&NamespacedType> for ComponentTypeId {
    fn from(nt: &NamespacedType) -> Self {
        Self(nt.clone())
    }
}

impl From<ComponentTypeId> for ComponentBuilder<((ComponentTypeId,), (), (), ())> {
    fn from(ty: ComponentTypeId) -> Self {
        ty.into_builder()
    }
}

impl FromStr for ComponentTypeId {
    type Err = NamespacedTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if NAMESPACED_TYPE_REGEX.is_match(s) {
            return NamespacedTypeConstructor::parse_namespace(s).map_err(NamespacedTypeParseError::NamespacedTypeError);
        }
        if TYPE_DEFINITION_REGEX.is_match(s) {
            let type_definition = TypeDefinition::from_str(s).map_err(NamespacedTypeParseError::TypeDefinitionParseError)?;
            return Self::try_from(&type_definition).map_err(NamespacedTypeParseError::TypeDefinitionConversionError);
        }
        NamespacedTypeConstructor::parse_namespace(s).map_err(NamespacedTypeParseError::NamespacedTypeError)
    }
}

impl TryFrom<&TypeDefinition> for ComponentTypeId {
    type Error = TypeDefinitionConversionError;

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::Component => Ok(ComponentTypeId::new(type_definition.namespaced_type.clone())),
            _ => Err(TypeDefinitionConversionError::TypeIdTypeMatchError(
                type_definition.clone(),
                type_definition.type_id_type.clone(),
                TypeIdType::Component,
            )),
        }
    }
}

impl Display for ComponentTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentTypeIds(DashSet<ComponentTypeId>);

impl ComponentTypeIds {
    #[inline]
    pub fn new() -> Self {
        NamespacedTypeIdContainer::new()
    }

    #[inline]
    pub fn parse_namespaces<I: IntoIterator<Item = NS>, NS: Into<String>>(namespaces: I) -> Result<Self, NamespacedTypeIdsError> {
        NamespacedTypeIdContainer::parse_namespaces(namespaces)
    }

    #[inline]
    pub fn parse_optional_namespaces(namespaces: Option<Vec<String>>) -> Result<Self, NamespacedTypeIdsError> {
        NamespacedTypeIdContainer::parse_optional_namespaces(namespaces)
    }

    pub fn with_namespace<N: Into<Namespace>>(namespace: N) -> Result<NamespacedTypeIds<Self>, NamespacedTypeIdsError> {
        <Self as NamespacedTypeIdContainer>::with_namespace(namespace)
    }

    pub fn component<TypeId: Into<ComponentTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl NamespacedTypeIdContainer for ComponentTypeIds {
    type TypeId = ComponentTypeId;
    type TypeIds = Self;

    fn new() -> Self {
        Self(DashSet::new())
    }

    fn insert(&self, ty: Self::TypeId) {
        self.0.insert(ty);
    }

    fn with_namespace<N: Into<Namespace>>(namespace: N) -> Result<NamespacedTypeIds<Self>, NamespacedTypeIdsError> {
        NamespacedTypeIds::new(namespace)
    }
}

impl ComponentTypeIdContainer for ComponentTypeIds {
    fn is_a(&self, ty: &ComponentTypeId) -> bool {
        self.0.contains(ty)
    }

    fn add_component<C: Into<ComponentTypeId>>(&self, ty: C) -> bool {
        self.0.insert(ty.into())
    }

    fn add_components<C: Into<ComponentTypeIds>>(&mut self, components_to_add: C) {
        let components_to_add = components_to_add.into();
        components_to_add.into_iter().for_each(|ty| {
            self.0.insert(ty);
        });
    }

    fn remove_component(&self, ty: &ComponentTypeId) -> Option<ComponentTypeId> {
        self.0.remove(ty)
    }

    fn remove_components<C: Into<ComponentTypeIds>>(&mut self, components_to_remove: C) {
        let components_to_remove = components_to_remove.into();
        components_to_remove.iter().for_each(|ty| {
            self.0.remove(&ty);
        });
    }

    fn get_components_cloned(&self) -> ComponentTypeIds {
        self.clone()
    }
}

impl Deref for ComponentTypeIds {
    type Target = DashSet<ComponentTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ComponentTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ComponentTypeIds {
    type Item = ComponentTypeId;
    type IntoIter = OwningIter<ComponentTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for ComponentTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        // This works because to_vec returns a sorted vector.
        this.eq(&other)
    }
}

impl Eq for ComponentTypeIds {}

impl Hash for ComponentTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for ComponentTypeIds {
    fn schema_name() -> Cow<'static, str> {
        "ComponentTypeIds".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<ComponentTypeId>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
        })
    }
}

impl From<Vec<ComponentTypeId>> for ComponentTypeIds {
    fn from(tys: Vec<ComponentTypeId>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<ComponentTypeIds> for Vec<ComponentTypeId> {
    fn from(tys: ComponentTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&ComponentTypeIds> for Vec<ComponentTypeId> {
    fn from(tys: &ComponentTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<ComponentTypeId>> for ComponentTypeIds {
    fn from(tys: DashSet<ComponentTypeId>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<ComponentTypeId>> for ComponentTypeIds {
    fn from(tys: &DashSet<ComponentTypeId>) -> Self {
        Self(tys.clone())
    }
}

impl From<ComponentTypeIds> for DashSet<ComponentTypeId> {
    fn from(tys: ComponentTypeIds) -> Self {
        tys.0
    }
}

impl From<NamespacedTypeIds<ComponentTypeIds>> for ComponentTypeIds {
    fn from(tys: NamespacedTypeIds<Self>) -> Self {
        tys.deref().clone()
    }
}

impl FromIterator<ComponentTypeId> for ComponentTypeIds {
    fn from_iter<I: IntoIterator<Item = ComponentTypeId>>(iter: I) -> Self {
        let tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

impl FromIterator<Self> for ComponentTypeIds {
    fn from_iter<I: IntoIterator<Item = Self>>(iter: I) -> Self {
        let all_tys = Self::new();
        for tys in iter {
            for ty in tys {
                all_tys.insert(ty);
            }
        }
        all_tys
    }
}

#[macro_export]
macro_rules! component_ty {
    (
        $component_type_id: ident,
        $namespace: ident,
        $component_name_const: ident,
        $component_name: expr
    ) => {
        pub const $component_name_const: &str = $component_name;
        pub static $component_type_id: std::sync::LazyLock<$crate::ComponentTypeId> =
            std::sync::LazyLock::new(|| $crate::ComponentTypeId::new_from_type($namespace, $component_name_const));
    };
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use std::ops::Deref;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::ComponentTypeId;
    use crate::ComponentTypeIds;
    use crate::NAMESPACE_SEPARATOR;
    use crate::Namespace;
    use crate::NamespaceSegment;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::RELATIVE_PATH_SEPARATOR;
    use crate::RandomNamespacedTypeId;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_namespace_path_segment;
    use reactive_graph_utils_test::r_namespace_type_name;

    #[test]
    fn component_type_id_test() {
        let path_segment_1 = r_namespace_path_segment();
        let path_segment_2 = r_namespace_path_segment();
        let type_name = r_namespace_type_name();

        // Construct string representations
        let path_str = format!("{}{}{}", path_segment_1, NAMESPACE_SEPARATOR, path_segment_2);
        let fully_qualified_namespace = format!("{}{}{}", path_str, NAMESPACE_SEPARATOR, type_name);
        let relative_path = format!("{}{}{}{}{}", path_segment_1, RELATIVE_PATH_SEPARATOR, path_segment_2, RELATIVE_PATH_SEPARATOR, type_name);

        // Instantiate typed namespace and segments
        let namespace = Namespace::from_str(&fully_qualified_namespace).expect("Failed to parse fully qualified namespace into namespace with type");
        let path = Namespace::from_str(&path_str).expect("Failed to parse path namespace");
        let type_name_segment = NamespaceSegment::from_str(&type_name).expect("Failed to parse type name into namespace segment");

        // Test FromStr and instantiate type id
        let ty = ComponentTypeId::from_str(fully_qualified_namespace.as_str()).expect("Failed to parse component type id from str");

        // Test parse from namespace
        let ty2 = ComponentTypeId::from_str(&fully_qualified_namespace).expect("Failed to parse component type id from string");
        assert_eq!(ty, ty2);

        // Test parse from optional namespace
        let ty3 =
            ComponentTypeId::parse_optional_namespace(Some(fully_qualified_namespace.clone())).expect("Failed to parse component type id from optional string");
        assert_eq!(Some(ty.clone()), ty3);
        assert_eq!(None, ComponentTypeId::parse_optional_namespace(None).unwrap());

        // Test inner state and display trait
        assert_eq!(fully_qualified_namespace, ty.0.namespace.to_string());
        assert_eq!(path_str, ty.0.path.to_string());
        assert_eq!(type_name, ty.0.type_name.to_string());

        // Test NamespacedTypeGetter inlined
        assert_eq!(namespace, ty.namespace());
        assert_eq!(path, ty.path());
        assert_eq!(type_name_segment, ty.type_name());

        // Test namespace parents
        assert_eq!(namespace.parent().expect("Failed to get namespace parent"), ty.path());
        assert_eq!(path, ty.namespace().parent().expect("Failed to get namespace parent"));

        // Test NamespacedTypeGetter
        assert_eq!(namespace, NamespacedTypeGetter::namespace(&ty));
        assert_eq!(path, NamespacedTypeGetter::path(&ty));
        assert_eq!(type_name_segment, NamespacedTypeGetter::type_name(&ty));

        // Test NamespaceSegment: type of the segments
        assert!(ty.namespace().is_type());
        assert!(ty.path().is_path());
        assert!(ty.type_name().is_type());

        // Test relative url & relative path
        assert_eq!(relative_path, ty.namespace().relative_url());
        assert_eq!(PathBuf::from(relative_path), ty.namespace().relative_path());

        // Test type definition
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::Component, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(path, type_definition.path());
        assert_eq!(type_name_segment, type_definition.type_name());

        // Convert into TypeDefinition
        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::Component, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(path, type_definition_3.path());
        assert_eq!(type_name_segment, type_definition_3.type_name());
    }

    #[test]
    fn component_type_id_new_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = ComponentTypeId::new(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
        let nt2 = NamespacedType::from(&ty);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn component_type_id_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = ComponentTypeId::from(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
    }

    #[test]
    fn component_type_id_from_string_test() {
        let ty1 = ComponentTypeId::from_str("namespace::Type").unwrap();
        assert_eq!("namespace::Type", ty1.namespace().to_string());
        assert_eq!("namespace", ty1.path().to_string());
        assert_eq!("Type", ty1.type_name().to_string());

        assert!(ComponentTypeId::from_str("namespace::namespace::Type").is_ok());
        assert!(ComponentTypeId::from_str("namespace::namespace::namespace::Type").is_ok());
        assert!(ComponentTypeId::from_str("namespace::namespace::namespace::namespace::Type").is_ok());

        assert!(ComponentTypeId::from_str("Namespace::Type").is_err());
        assert!(ComponentTypeId::from_str("namespace::type").is_err());
        assert!(ComponentTypeId::from_str("namespace::Namespace::Type").is_err());
        assert!(ComponentTypeId::from_str("namespace::namespace::type").is_err());
        assert!(ComponentTypeId::from_str("Namespace::namespace::type").is_err());
        assert!(ComponentTypeId::from_str("namespace::Namespace::type").is_err());
        assert!(ComponentTypeId::from_str("::Namespace::Type").is_err());
        assert!(ComponentTypeId::from_str("::Namespace::Type::").is_err());
        assert!(ComponentTypeId::from_str("Namespace::Type::").is_err());
    }

    #[test]
    fn component_types_eq_test() {
        let ty1 = ComponentTypeId::from_str("namespace::Type1").unwrap();
        let ty2 = ComponentTypeId::from_str("namespace::Type2").unwrap();
        let tys1 = ComponentTypeIds::new().component(&ty1).component(&ty2);
        let tys2 = ComponentTypeIds::new().component(&ty2).component(&ty1);
        assert_eq!(tys1, tys2);
    }

    #[test]
    fn component_types_with_namespace_test() {
        let tys = ComponentTypeIds::with_namespace(Namespace::from_str("namespace::namespace").unwrap())
            .unwrap()
            .ty(NamespaceSegment::from_str("Type1").unwrap())
            .unwrap()
            .ty(NamespaceSegment::from_str("Type2").unwrap())
            .unwrap()
            .deref()
            .clone();
        assert!(tys.contains(&ComponentTypeId::from_str("namespace::namespace::Type1").unwrap()));
        assert!(tys.contains(&ComponentTypeId::from_str("namespace::namespace::Type2").unwrap()));
        assert!(!tys.contains(&ComponentTypeId::from_str("namespace::namespace::Type3").unwrap()));
    }

    #[test]
    fn component_type_id_json_schema() {
        let schema = schema_for!(ComponentTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn parse_component_type_id_from_vec_string() {
        let tys = ComponentTypeIds::parse_namespaces(vec!["test::Test1".to_string(), "test::Test2".to_string()]).unwrap();
        assert_eq!(tys.len(), 2);
        assert!(tys.contains(&ComponentTypeId::from_str("test::Test1").unwrap()));
        assert!(tys.contains(&ComponentTypeId::from_str("test::Test2").unwrap()));
    }

    #[test]
    fn parse_component_type_id_from_vec_str() {
        let tys = ComponentTypeIds::parse_namespaces(vec!["test::Test1", "test::Test2"]).unwrap();
        assert_eq!(tys.len(), 2);
        assert!(tys.contains(&ComponentTypeId::from_str("test::Test1").unwrap()));
        assert!(tys.contains(&ComponentTypeId::from_str("test::Test2").unwrap()));
    }
}
