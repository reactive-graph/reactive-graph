use std::borrow::Cow;
use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;

use crate::NamespaceSegment;
use crate::NamespacedType;
use crate::NamespacedTypeConstructor;
use crate::NamespacedTypeError;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeIdContainer;
use crate::NamespacedTypeIds;
use crate::NamespacedTypeIdsError;
use crate::TypeDefinition;
use crate::TypeDefinitionConversionError;
use crate::TypeDefinitionGetter;
use crate::TypeIdParseError;
use crate::TypeIdType;

use crate::namespace::Namespace;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;
#[cfg(any(test, feature = "table"))]
use tabled::Tabled;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
#[cfg_attr(any(test, feature = "table"), derive(Tabled))]
pub struct EntityTypeId(#[cfg_attr(any(test, feature = "table"), tabled(inline))] NamespacedType);

impl EntityTypeId {
    pub fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        NamespacedTypeConstructor::new(nt)
    }

    pub fn parse_namespace(namespace: &String) -> Result<Self, NamespacedTypeError> {
        NamespacedTypeConstructor::parse_namespace(namespace)
    }

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

// impl<N: Into<Namespace>, T: Into<String>> From<(N, T)> for EntityTypeId {
//     fn from(ty: (N, T)) -> Self {
//         Self(NamespacedType::from(ty))
//     }
// }

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

impl TryFrom<&str> for EntityTypeId {
    type Error = TypeIdParseError;

    fn try_from(type_definition: &str) -> Result<Self, Self::Error> {
        let type_definition = TypeDefinition::try_from(type_definition).map_err(TypeIdParseError::TypeDefinitionParseError)?;
        if TypeIdType::EntityType != type_definition.type_id_type {
            return Err(TypeIdParseError::InvalidTypeIdType(TypeIdType::EntityType, type_definition.type_id_type));
        }
        Ok(Self((&type_definition).into()))
    }
}

impl TryFrom<&String> for EntityTypeId {
    type Error = TypeIdParseError;

    fn try_from(type_definition: &String) -> Result<Self, Self::Error> {
        Self::try_from(type_definition.as_str())
    }
}

impl TryFrom<String> for EntityTypeId {
    type Error = TypeIdParseError;

    fn try_from(type_definition: String) -> Result<Self, Self::Error> {
        Self::try_from(type_definition.as_str())
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
    pub fn new() -> Self {
        NamespacedTypeIdContainer::new()
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
        let tys = EntityTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
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

#[cfg(any(test, feature = "test"))]
impl DefaultTest for EntityTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for EntityTypeIds {
    fn default_test() -> Self {
        let tys = EntityTypeIds::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            tys.insert(EntityTypeId::default_test());
        }
        tys
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::EntityTypeId;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn entity_type_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty = EntityTypeId::new_from_type(&namespace, &type_name);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(type_name, ty.type_name());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(nt.type_name, ty.type_name());
        assert_eq!(format!("e__{namespace}__{type_name}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::EntityType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(type_name, type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::EntityType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(type_name, type_definition_3.type_name());
    }

    #[test]
    fn entity_type_id_new_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty2 = EntityTypeId::new(nt.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(type_name, ty2.type_name());

        let nt2 = NamespacedType::from(&ty2);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn entity_type_id_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty1 = EntityTypeId::from(nt);
        assert_eq!(namespace, ty1.namespace());
        assert_eq!(type_name, ty1.type_name());
    }

    #[test]
    fn entity_type_id_from_string_test() {
        let s1 = String::from("e__ns__ty");
        let ty1 = EntityTypeId::try_from(&s1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.type_name());

        let s2 = String::from("r__ns__ty");
        let ty2 = EntityTypeId::try_from(&s2);
        assert!(ty2.is_err());

        let s3 = String::from("e__");
        let ty3 = EntityTypeId::try_from(&s3);
        assert!(ty3.is_err());

        let s4 = String::from("e__ns");
        let ty4 = EntityTypeId::try_from(&s4);
        assert!(ty4.is_err());

        let s5 = String::from("e__ns__");
        let ty5 = EntityTypeId::try_from(&s5);
        assert!(ty5.is_err());

        let s6 = String::from("e__ns__ty__");
        let ty6 = EntityTypeId::try_from(&s6);
        assert!(ty6.is_err());

        let s7 = String::from("e__ns__ty__xx");
        let ty7 = EntityTypeId::try_from(&s7);
        assert!(ty7.is_err());
    }

    #[test]
    fn entity_type_id_json_schema() {
        let schema = schema_for!(EntityTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
