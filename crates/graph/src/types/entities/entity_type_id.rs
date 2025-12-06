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

use crate::NAMESPACED_TYPE_REGEX;
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
use crate::namespace::Namespace;

#[cfg(any(test, feature = "table"))]
use tabled::Tabled;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "table"), derive(Tabled))]
pub struct EntityTypeId(#[cfg_attr(any(test, feature = "table"), tabled(inline))] NamespacedType);

impl EntityTypeId {
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

impl NamespacedTypeConstructor for EntityTypeId {
    fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        Self(nt.into())
    }
}

impl NamespacedTypeGetter for EntityTypeId {
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

impl TypeDefinitionGetter for EntityTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::EntityType
    }
}

impl AsRef<NamespacedType> for EntityTypeId {
    fn as_ref(&self) -> &NamespacedType {
        &self.0
    }
}

impl AsRef<Namespace> for EntityTypeId {
    fn as_ref(&self) -> &Namespace {
        &self.0.namespace
    }
}

impl From<&EntityTypeId> for EntityTypeId {
    fn from(ty: &EntityTypeId) -> Self {
        ty.clone()
    }
}

impl From<&EntityTypeId> for TypeDefinition {
    fn from(ty: &EntityTypeId) -> Self {
        TypeDefinition::new(TypeIdType::EntityType, ty.0.clone())
    }
}

impl From<&EntityTypeId> for NamespacedType {
    fn from(ty: &EntityTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for EntityTypeId {
    fn from(nt: NamespacedType) -> Self {
        EntityTypeId(nt)
    }
}

impl From<&NamespacedType> for EntityTypeId {
    fn from(nt: &NamespacedType) -> Self {
        EntityTypeId(nt.clone())
    }
}

impl FromStr for EntityTypeId {
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

impl TryFrom<&TypeDefinition> for EntityTypeId {
    type Error = TypeDefinitionConversionError;

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::EntityType => Ok(EntityTypeId::new(type_definition.namespaced_type.clone())),
            _ => Err(TypeDefinitionConversionError::TypeIdTypeMatchError(
                type_definition.clone(),
                type_definition.type_id_type.clone(),
                TypeIdType::EntityType,
            )),
        }
    }
}

impl Display for EntityTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntityTypeIds(DashSet<EntityTypeId>);

impl EntityTypeIds {
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

    pub fn entity_type<TypeId: Into<EntityTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl NamespacedTypeIdContainer for EntityTypeIds {
    type TypeId = EntityTypeId;
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

impl Deref for EntityTypeIds {
    type Target = DashSet<EntityTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for EntityTypeIds {
    type Item = EntityTypeId;
    type IntoIter = OwningIter<EntityTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for EntityTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for EntityTypeIds {}

impl Hash for EntityTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for EntityTypeIds {
    fn schema_name() -> Cow<'static, str> {
        "EntityTypeIds".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<EntityTypeId>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
        })
    }
}

impl From<Vec<EntityTypeId>> for EntityTypeIds {
    fn from(tys: Vec<EntityTypeId>) -> Self {
        EntityTypeIds(tys.into_iter().collect())
    }
}

impl From<EntityTypeIds> for Vec<EntityTypeId> {
    fn from(tys: EntityTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&EntityTypeIds> for Vec<EntityTypeId> {
    fn from(tys: &EntityTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<EntityTypeId>> for EntityTypeIds {
    fn from(tys: DashSet<EntityTypeId>) -> Self {
        EntityTypeIds(tys)
    }
}

impl From<&DashSet<EntityTypeId>> for EntityTypeIds {
    fn from(tys: &DashSet<EntityTypeId>) -> Self {
        EntityTypeIds(tys.clone())
    }
}

impl From<EntityTypeIds> for DashSet<EntityTypeId> {
    fn from(tys: EntityTypeIds) -> Self {
        tys.0
    }
}

impl From<NamespacedTypeIds<EntityTypeIds>> for EntityTypeIds {
    fn from(tys: NamespacedTypeIds<Self>) -> Self {
        tys.deref().clone()
    }
}

impl FromIterator<EntityTypeId> for EntityTypeIds {
    fn from_iter<I: IntoIterator<Item = EntityTypeId>>(iter: I) -> Self {
        let tys = Self::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

impl FromIterator<Self> for EntityTypeIds {
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
macro_rules! entity_ty {
    (
        $entity_type_id: ident,
        $namespace: ident,
        $entity_type_name_const: ident,
        $entity_type_name: expr
    ) => {
        pub const $entity_type_name_const: &str = $entity_type_name;
        pub static $entity_type_id: std::sync::LazyLock<$crate::EntityTypeId> =
            std::sync::LazyLock::new(|| $crate::EntityTypeId::new_from_type($namespace, $entity_type_name_const));
    };
}

// #[cfg(any(test, feature = "test"))]
// impl RandomType for EntityTypeId {
//     type Error = NamespacedTypeError;
//
//     fn random_type() -> Result<Self, NamespacedTypeError> {
//         NamespacedType::random_type().map(Self::new)
//     }
// }
//
// #[cfg(any(test, feature = "test"))]
// impl RandomTypes for EntityTypeIds {
//     type Error = NamespacedTypeError;
//
//     fn random_types() -> Result<Self, NamespacedTypeError> {
//         let tys = Self::new();
//         let mut rng = rand::rng();
//         for _ in 0..rng.random_range(0..10) {
//             tys.insert(EntityTypeId::random_type()?);
//         }
//         Ok(tys)
//     }
// }

#[cfg(test)]
mod tests {
    use schemars::schema_for;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::EntityTypeId;
    use crate::EntityTypeIds;
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
    fn entity_type_id_test() {
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
        let ty = EntityTypeId::from_str(fully_qualified_namespace.as_str()).expect("Failed to parse entity type id from str");

        // Test parse from namespace
        let ty2 = EntityTypeId::from_str(&fully_qualified_namespace).expect("Failed to parse entity type id from string");
        assert_eq!(ty, ty2);

        // Test parse from optional namespace
        let ty3 = EntityTypeId::parse_optional_namespace(Some(fully_qualified_namespace.clone())).expect("Failed to parse entity type id from string");
        assert_eq!(Some(ty.clone()), ty3);
        assert_eq!(None, EntityTypeId::parse_optional_namespace(None).unwrap());

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
        assert_eq!(TypeIdType::EntityType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(path, type_definition.path());
        assert_eq!(type_name_segment, type_definition.type_name());

        // Convert into TypeDefinition
        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::EntityType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(path, type_definition_3.path());
        assert_eq!(type_name_segment, type_definition_3.type_name());
    }

    #[test]
    fn entity_type_id_new_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = EntityTypeId::new(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
        let nt2 = NamespacedType::from(&ty);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn entity_type_id_from_namespaced_type_test() {
        let nt = NamespacedType::random_type_id().unwrap();
        let ty = EntityTypeId::from(nt.clone());
        assert_eq!(nt.namespace(), ty.namespace());
        assert_eq!(nt.path(), ty.path());
        assert_eq!(nt.type_name(), ty.type_name());
    }

    #[test]
    fn entity_type_id_from_string_test() {
        let ty1 = EntityTypeId::from_str("namespace::Type").unwrap();
        assert_eq!("namespace::Type", ty1.namespace().to_string());
        assert_eq!("namespace", ty1.path().to_string());
        assert_eq!("Type", ty1.type_name().to_string());

        assert!(EntityTypeId::from_str("namespace::namespace::Type").is_ok());
        assert!(EntityTypeId::from_str("namespace::namespace::namespace::Type").is_ok());
        assert!(EntityTypeId::from_str("namespace::namespace::namespace::namespace::Type").is_ok());

        assert!(EntityTypeId::from_str("Namespace::Type").is_err());
        assert!(EntityTypeId::from_str("namespace::type").is_err());
        assert!(EntityTypeId::from_str("namespace::Namespace::Type").is_err());
        assert!(EntityTypeId::from_str("namespace::namespace::type").is_err());
        assert!(EntityTypeId::from_str("Namespace::namespace::type").is_err());
        assert!(EntityTypeId::from_str("namespace::Namespace::type").is_err());
        assert!(EntityTypeId::from_str("::Namespace::Type").is_err());
        assert!(EntityTypeId::from_str("::Namespace::Type::").is_err());
        assert!(EntityTypeId::from_str("Namespace::Type::").is_err());
    }

    #[test]
    fn entity_types_eq_test() {
        let ty1 = EntityTypeId::from_str("namespace::Type1").unwrap();
        let ty2 = EntityTypeId::from_str("namespace::Type2").unwrap();
        let tys1 = EntityTypeIds::new().entity_type(&ty1).entity_type(&ty2);
        let tys2 = EntityTypeIds::new().entity_type(&ty2).entity_type(&ty1);
        assert_eq!(tys1, tys2);
    }

    #[test]
    fn entity_type_id_json_schema() {
        let schema = schema_for!(EntityTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
