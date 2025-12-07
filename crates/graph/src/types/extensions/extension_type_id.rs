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

use crate::Extension;
use crate::ExtensionBuilder;
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
pub struct ExtensionTypeId(#[cfg_attr(any(test, feature = "table"), tabled(inline))] NamespacedType);

impl ExtensionTypeId {
    pub fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        NamespacedTypeConstructor::new(nt)
    }

    #[inline]
    pub fn parse_str(namespace: &str) -> Result<Self, NamespacedTypeError> {
        NamespacedTypeConstructor::parse_namespace(namespace)
    }

    pub fn parse_optional_namespace(namespace: Option<String>) -> Result<Option<Self>, NamespacedTypeError> {
        NamespacedTypeConstructor::parse_optional_namespace(namespace)
    }

    #[inline]
    pub fn into_builder(self) -> ExtensionBuilder<((Self,), (), (), ())> {
        Extension::builder().ty(self)
    }
}

impl NamespacedTypeConstructor for ExtensionTypeId {
    fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        Self(nt.into())
    }
}

impl NamespacedTypeGetter for ExtensionTypeId {
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

impl TypeDefinitionGetter for ExtensionTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::Extension
    }
}

impl AsRef<NamespacedType> for ExtensionTypeId {
    fn as_ref(&self) -> &NamespacedType {
        &self.0
    }
}

impl AsRef<Namespace> for ExtensionTypeId {
    fn as_ref(&self) -> &Namespace {
        &self.0.namespace
    }
}

impl From<&ExtensionTypeId> for ExtensionTypeId {
    fn from(ty: &ExtensionTypeId) -> Self {
        ty.clone()
    }
}

impl From<&ExtensionTypeId> for TypeDefinition {
    fn from(ty: &ExtensionTypeId) -> Self {
        TypeDefinition::new(TypeIdType::Extension, ty.0.clone())
    }
}

impl From<&ExtensionTypeId> for NamespacedType {
    fn from(ty: &ExtensionTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for ExtensionTypeId {
    fn from(nt: NamespacedType) -> Self {
        ExtensionTypeId(nt)
    }
}

impl From<&NamespacedType> for ExtensionTypeId {
    fn from(nt: &NamespacedType) -> Self {
        ExtensionTypeId(nt.clone())
    }
}

impl From<ExtensionTypeId> for ExtensionBuilder<((ExtensionTypeId,), (), (), ())> {
    fn from(ty: ExtensionTypeId) -> Self {
        ty.into_builder()
    }
}

impl FromStr for ExtensionTypeId {
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

impl TryFrom<&TypeDefinition> for ExtensionTypeId {
    type Error = TypeDefinitionConversionError;

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::Extension => Ok(ExtensionTypeId::new(type_definition.namespaced_type.clone())),
            _ => Err(TypeDefinitionConversionError::TypeIdTypeMatchError(
                type_definition.clone(),
                type_definition.type_id_type.clone(),
                TypeIdType::Extension,
            )),
        }
    }
}

impl Display for ExtensionTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExtensionTypeIds(DashSet<ExtensionTypeId>);

impl ExtensionTypeIds {
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

    pub fn extension<TypeId: Into<ExtensionTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl NamespacedTypeIdContainer for ExtensionTypeIds {
    type TypeId = ExtensionTypeId;
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

impl Deref for ExtensionTypeIds {
    type Target = DashSet<ExtensionTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ExtensionTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for ExtensionTypeIds {
    type Item = ExtensionTypeId;
    type IntoIter = OwningIter<ExtensionTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for ExtensionTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for ExtensionTypeIds {}

impl Hash for ExtensionTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for ExtensionTypeIds {
    fn schema_name() -> Cow<'static, str> {
        "ExtensionTypeIds".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<ExtensionTypeId>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
        })
    }
}

impl From<Vec<ExtensionTypeId>> for ExtensionTypeIds {
    fn from(tys: Vec<ExtensionTypeId>) -> Self {
        ExtensionTypeIds(tys.into_iter().collect())
    }
}

impl From<ExtensionTypeIds> for Vec<ExtensionTypeId> {
    fn from(tys: ExtensionTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&ExtensionTypeIds> for Vec<ExtensionTypeId> {
    fn from(tys: &ExtensionTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<ExtensionTypeId>> for ExtensionTypeIds {
    fn from(tys: DashSet<ExtensionTypeId>) -> Self {
        ExtensionTypeIds(tys)
    }
}

impl From<&DashSet<ExtensionTypeId>> for ExtensionTypeIds {
    fn from(tys: &DashSet<ExtensionTypeId>) -> Self {
        ExtensionTypeIds(tys.clone())
    }
}

impl From<ExtensionTypeIds> for DashSet<ExtensionTypeId> {
    fn from(tys: ExtensionTypeIds) -> Self {
        tys.0
    }
}

impl FromIterator<ExtensionTypeId> for ExtensionTypeIds {
    fn from_iter<I: IntoIterator<Item = ExtensionTypeId>>(iter: I) -> Self {
        let tys = ExtensionTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

#[macro_export]
macro_rules! extension_ty {
    (
        $extension_type_id: ident,
        $namespace: ident,
        $extension_name_const: ident,
        $extension_name: expr
    ) => {
        pub const $extension_name_const: &str = $extension_name;
        pub static $extension_type_id: std::sync::LazyLock<$crate::ExtensionTypeId> =
            std::sync::LazyLock::new(|| $crate::ExtensionTypeId::new_from_type($namespace, $extension_name_const));
    };
}

// #[cfg(any(test, feature = "test"))]
// impl RandomType for ExtensionTypeId {
//     type Error = NamespacedTypeError;
//
//     fn random_type() -> Result<Self, NamespacedTypeError> {
//         NamespacedType::random_type().map(Self::new)
//     }
// }
//
// #[cfg(any(test, feature = "test"))]
// impl RandomTypes for ExtensionTypeIds {
//     type Error = NamespacedTypeError;
//
//     fn random_types() -> Result<Self, NamespacedTypeError> {
//         let tys = Self::new();
//         let mut rng = rand::rng();
//         for _ in 0..rng.random_range(0..10) {
//             tys.insert(ExtensionTypeId::random_type()?);
//         }
//         Ok(tys)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::ExtensionTypeId;
    use crate::ExtensionTypeIds;
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
    use schemars::schema_for;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn extension_type_id_test() {
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
        let ty = ExtensionTypeId::from_str(fully_qualified_namespace.as_str()).expect("Failed to parse entity type id from str");

        // Test parse from namespace
        let ty2 = ExtensionTypeId::from_str(&fully_qualified_namespace).expect("Failed to parse entity type id from string");
        assert_eq!(ty, ty2);

        // Test parse from optional namespace
        let ty3 = ExtensionTypeId::parse_optional_namespace(Some(fully_qualified_namespace.clone())).expect("Failed to parse entity type id from string");
        assert_eq!(Some(ty.clone()), ty3);
        assert_eq!(None, ExtensionTypeId::parse_optional_namespace(None).unwrap());

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
        assert_eq!(TypeIdType::Extension, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(path, type_definition.path());
        assert_eq!(type_name_segment, type_definition.type_name());

        // Convert into TypeDefinition
        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::Extension, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(path, type_definition_3.path());
        assert_eq!(type_name_segment, type_definition_3.type_name());
    }

    #[test]
    fn extension_type_id_new_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = ExtensionTypeId::new(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
        let nt2 = NamespacedType::from(&ty);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn extension_type_id_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = ExtensionTypeId::from(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
    }

    #[test]
    fn extension_type_id_from_string_test() {
        let ty1 = ExtensionTypeId::from_str("namespace::Type").unwrap();
        assert_eq!("namespace::Type", ty1.namespace().to_string());
        assert_eq!("namespace", ty1.path().to_string());
        assert_eq!("Type", ty1.type_name().to_string());

        assert!(ExtensionTypeId::from_str("namespace::namespace::Type").is_ok());
        assert!(ExtensionTypeId::from_str("namespace::namespace::namespace::Type").is_ok());
        assert!(ExtensionTypeId::from_str("namespace::namespace::namespace::namespace::Type").is_ok());

        assert!(ExtensionTypeId::from_str("Namespace::Type").is_err());
        assert!(ExtensionTypeId::from_str("namespace::type").is_err());
        assert!(ExtensionTypeId::from_str("namespace::Namespace::Type").is_err());
        assert!(ExtensionTypeId::from_str("namespace::namespace::type").is_err());
        assert!(ExtensionTypeId::from_str("Namespace::namespace::type").is_err());
        assert!(ExtensionTypeId::from_str("namespace::Namespace::type").is_err());
        assert!(ExtensionTypeId::from_str("::Namespace::Type").is_err());
        assert!(ExtensionTypeId::from_str("::Namespace::Type::").is_err());
        assert!(ExtensionTypeId::from_str("Namespace::Type::").is_err());
    }

    #[test]
    fn extension_type_types_eq_test() {
        let ty1 = ExtensionTypeId::from_str("namespace::Type1").unwrap();
        let ty2 = ExtensionTypeId::from_str("namespace::Type2").unwrap();
        let tys1 = ExtensionTypeIds::new().extension(&ty1).extension(&ty2);
        let tys2 = ExtensionTypeIds::new().extension(&ty2).extension(&ty1);
        assert_eq!(tys1, tys2);
    }

    #[test]
    fn extension_type_id_json_schema() {
        let schema = schema_for!(ExtensionTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
