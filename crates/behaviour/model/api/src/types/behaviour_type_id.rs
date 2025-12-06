use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
use reactive_graph_graph::namespace::Namespace;
use reactive_graph_graph::prelude::*;
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct BehaviourTypeId(NamespacedType);

impl BehaviourTypeId {
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
}

impl NamespacedTypeConstructor for BehaviourTypeId {
    fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        Self(nt.into())
    }
}

impl NamespacedTypeGetter for BehaviourTypeId {
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

impl TypeDefinitionGetter for BehaviourTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::Behaviour
    }
}

impl AsRef<NamespacedType> for BehaviourTypeId {
    fn as_ref(&self) -> &NamespacedType {
        &self.0
    }
}

impl AsRef<Namespace> for BehaviourTypeId {
    fn as_ref(&self) -> &Namespace {
        &self.0.namespace
    }
}

impl From<&BehaviourTypeId> for BehaviourTypeId {
    fn from(ty: &BehaviourTypeId) -> Self {
        ty.clone()
    }
}

impl From<&BehaviourTypeId> for TypeDefinition {
    fn from(ty: &BehaviourTypeId) -> Self {
        TypeDefinition::new(TypeIdType::Behaviour, ty.0.clone())
    }
}

impl From<&BehaviourTypeId> for NamespacedType {
    fn from(ty: &BehaviourTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for BehaviourTypeId {
    fn from(nt: NamespacedType) -> Self {
        BehaviourTypeId(nt)
    }
}

impl From<&NamespacedType> for BehaviourTypeId {
    fn from(nt: &NamespacedType) -> Self {
        BehaviourTypeId(nt.clone())
    }
}

impl FromStr for BehaviourTypeId {
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

impl TryFrom<&TypeDefinition> for BehaviourTypeId {
    type Error = TypeDefinitionConversionError;

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::Behaviour => Ok(BehaviourTypeId::new(type_definition.namespaced_type.clone())),
            _ => Err(TypeDefinitionConversionError::TypeIdTypeMatchError(
                type_definition.clone(),
                type_definition.type_id_type.clone(),
                TypeIdType::Behaviour,
            )),
        }
    }
}

impl Display for BehaviourTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BehaviourTypeIds(DashSet<BehaviourTypeId>);

impl BehaviourTypeIds {
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

    // pub fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
    //     NamespacedTypeIdContainer::with_namespace(namespace)
    // }

    pub fn behaviour<TypeId: Into<BehaviourTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }
}

impl NamespacedTypeIdContainer for BehaviourTypeIds {
    type TypeId = BehaviourTypeId;
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

impl Deref for BehaviourTypeIds {
    type Target = DashSet<BehaviourTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BehaviourTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for BehaviourTypeIds {
    type Item = BehaviourTypeId;
    type IntoIter = OwningIter<BehaviourTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for BehaviourTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for BehaviourTypeIds {}

impl Hash for BehaviourTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for BehaviourTypeIds {
    fn schema_name() -> Cow<'static, str> {
        "BehaviourTypeIds".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<BehaviourTypeId>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
        })
    }
}

impl From<Vec<BehaviourTypeId>> for BehaviourTypeIds {
    fn from(tys: Vec<BehaviourTypeId>) -> Self {
        Self(tys.into_iter().collect())
    }
}

impl From<BehaviourTypeIds> for Vec<BehaviourTypeId> {
    fn from(tys: BehaviourTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&BehaviourTypeIds> for Vec<BehaviourTypeId> {
    fn from(tys: &BehaviourTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<BehaviourTypeId>> for BehaviourTypeIds {
    fn from(tys: DashSet<BehaviourTypeId>) -> Self {
        Self(tys)
    }
}

impl From<&DashSet<BehaviourTypeId>> for BehaviourTypeIds {
    fn from(tys: &DashSet<BehaviourTypeId>) -> Self {
        Self(tys.clone())
    }
}

impl From<BehaviourTypeIds> for DashSet<BehaviourTypeId> {
    fn from(tys: BehaviourTypeIds) -> Self {
        tys.0
    }
}

impl From<NamespacedTypeIds<BehaviourTypeIds>> for BehaviourTypeIds {
    fn from(tys: NamespacedTypeIds<Self>) -> Self {
        tys.deref().clone()
    }
}

impl FromIterator<BehaviourTypeId> for BehaviourTypeIds {
    fn from_iter<I: IntoIterator<Item = BehaviourTypeId>>(iter: I) -> Self {
        let tys = BehaviourTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

// pub struct NamespacedTypeIds<T> {
//     namespace: String,
//     tys: T,
// }
//
// impl<T> NamespacedTypeIds<T> {
//     pub fn new<N: Into<String>>(namespace: N, tys: T) -> Self {
//         Self { namespace, tys }
//     }
//
//     pub fn ty<S: Into<String>>(self, type_name: S) -> Self {
//         self.insert(NamespacedType::new(self.namespace.clone(), type_name.into()).into());
//         self
//     }
// }
//
// impl<T> Deref for NamespacedTypeIds<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.tys
//     }
// }
//
// impl<T> From<NamespacedTypeIds<T>> for T {
//     fn from(tys: NamespacedTypeIds<T>) -> Self {
//         tys.tys
//     }
// }

#[macro_export]
macro_rules! behaviour_ty {
    (
        $behaviour_type_id: ident,
        $namespace: ident,
        $behaviour_name_const: ident,
        $behaviour_name: expr
    ) => {
        pub const $behaviour_name_const: &str = $behaviour_name;
        pub static $behaviour_type_id: std::sync::LazyLock<$crate::BehaviourTypeId> =
            std::sync::LazyLock::new(|| $crate::BehaviourTypeId::new_from_type($namespace, $behaviour_name_const));
    };
}

// #[cfg(any(test, feature = "test"))]
// impl RandomType for BehaviourTypeId {
//     type Error = NamespacedTypeError;
//
//     fn random_type() -> Result<Self, NamespacedTypeError> {
//         NamespacedType::random_type().map(Self::new)
//     }
// }
//
// #[cfg(any(test, feature = "test"))]
// impl RandomTypes for BehaviourTypeIds {
//     type Error = NamespacedTypeError;
//
//     fn random_types() -> Result<Self, NamespacedTypeError> {
//         let tys = Self::new();
//         let mut rng = rand::rng();
//         for _ in 0..rng.random_range(0..10) {
//             tys.insert(BehaviourTypeId::random_type()?);
//         }
//         Ok(tys)
//     }
// }

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::BehaviourTypeId;
    use crate::BehaviourTypeIds;
    use reactive_graph_graph::NAMESPACE_SEPARATOR;
    use reactive_graph_graph::Namespace;
    use reactive_graph_graph::NamespaceSegment;
    use reactive_graph_graph::NamespacedType;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RELATIVE_PATH_SEPARATOR;
    use reactive_graph_graph::RandomNamespacedTypeId;
    use reactive_graph_graph::TypeDefinition;
    use reactive_graph_graph::TypeDefinitionGetter;
    use reactive_graph_graph::TypeIdType;
    use reactive_graph_utils_test::r_namespace_path_segment;
    use reactive_graph_utils_test::r_namespace_type_name;

    #[test]
    fn behaviour_type_id_test() {
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
        let ty = BehaviourTypeId::from_str(fully_qualified_namespace.as_str()).expect("Failed to parse behaviour type id from str");

        // Test parse from namespace
        let ty2 = BehaviourTypeId::from_str(&fully_qualified_namespace).expect("Failed to parse behaviour type id from string");
        assert_eq!(ty, ty2);

        // Test parse from optional namespace
        let ty3 =
            BehaviourTypeId::parse_optional_namespace(Some(fully_qualified_namespace.clone())).expect("Failed to parse behaviour type id from optional string");
        assert_eq!(Some(ty.clone()), ty3);
        assert_eq!(None, BehaviourTypeId::parse_optional_namespace(None).unwrap());

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
        assert_eq!(TypeIdType::Behaviour, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(path, type_definition.path());
        assert_eq!(type_name_segment, type_definition.type_name());

        // Convert into TypeDefinition
        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::Behaviour, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(path, type_definition_3.path());
        assert_eq!(type_name_segment, type_definition_3.type_name());
    }

    #[test]
    fn behaviour_type_id_new_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = BehaviourTypeId::new(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
        let nt2 = NamespacedType::from(&ty);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn behaviour_type_id_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = BehaviourTypeId::from(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
    }

    #[test]
    fn behaviour_type_id_from_string_test() {
        let ty1 = BehaviourTypeId::from_str("namespace::Type").unwrap();
        assert_eq!("namespace::Type", ty1.namespace().to_string());
        assert_eq!("namespace", ty1.path().to_string());
        assert_eq!("Type", ty1.type_name().to_string());

        assert!(BehaviourTypeId::from_str("namespace::namespace::Type").is_ok());
        assert!(BehaviourTypeId::from_str("namespace::namespace::namespace::Type").is_ok());
        assert!(BehaviourTypeId::from_str("namespace::namespace::namespace::namespace::Type").is_ok());

        assert!(BehaviourTypeId::from_str("Namespace::Type").is_err());
        assert!(BehaviourTypeId::from_str("namespace::type").is_err());
        assert!(BehaviourTypeId::from_str("namespace::Namespace::Type").is_err());
        assert!(BehaviourTypeId::from_str("namespace::namespace::type").is_err());
        assert!(BehaviourTypeId::from_str("Namespace::namespace::type").is_err());
        assert!(BehaviourTypeId::from_str("namespace::Namespace::type").is_err());
        assert!(BehaviourTypeId::from_str("::Namespace::Type").is_err());
        assert!(BehaviourTypeId::from_str("::Namespace::Type::").is_err());
        assert!(BehaviourTypeId::from_str("Namespace::Type::").is_err());
    }

    #[test]
    fn behaviour_types_eq_test() {
        let ty1 = BehaviourTypeId::from_str("namespace::Type1").unwrap();
        let ty2 = BehaviourTypeId::from_str("namespace::Type2").unwrap();
        let tys1 = BehaviourTypeIds::new().behaviour(&ty1).behaviour(&ty2);
        let tys2 = BehaviourTypeIds::new().behaviour(&ty2).behaviour(&ty1);
        assert_eq!(tys1, tys2);
    }

    #[test]
    fn behaviour_type_id_json_schema() {
        let schema = schema_for!(BehaviourTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
