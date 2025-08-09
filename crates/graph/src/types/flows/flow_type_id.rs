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

use crate::Namespace;
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
pub struct FlowTypeId(#[cfg_attr(any(test, feature = "table"), tabled(inline))] NamespacedType);

impl FlowTypeId {
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

impl NamespacedTypeConstructor for FlowTypeId {
    fn new<NT: Into<NamespacedType>>(nt: NT) -> Self {
        Self(nt.into())
    }
}

impl NamespacedTypeGetter for FlowTypeId {
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

impl TypeDefinitionGetter for FlowTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
    }

    fn type_id_type() -> TypeIdType {
        TypeIdType::FlowType
    }
}

impl From<&FlowTypeId> for FlowTypeId {
    fn from(ty: &FlowTypeId) -> Self {
        ty.clone()
    }
}

impl From<&FlowTypeId> for TypeDefinition {
    fn from(ty: &FlowTypeId) -> Self {
        TypeDefinition::new(TypeIdType::FlowType, ty.0.clone())
    }
}

impl From<&FlowTypeId> for NamespacedType {
    fn from(ty: &FlowTypeId) -> Self {
        ty.0.clone()
    }
}

impl From<NamespacedType> for FlowTypeId {
    fn from(nt: NamespacedType) -> Self {
        FlowTypeId(nt)
    }
}

// impl<N: Into<Namespace>, T: Into<String>> From<(N, T)> for FlowTypeId {
//     fn from(ty: (N, T)) -> Self {
//         Self(NamespacedType::from(ty))
//     }
// }

impl TryFrom<&TypeDefinition> for FlowTypeId {
    type Error = TypeDefinitionConversionError;

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::FlowType => Ok(FlowTypeId::new(type_definition.namespaced_type.clone())),
            _ => Err(TypeDefinitionConversionError::TypeIdTypeMatchError(
                type_definition.clone(),
                type_definition.type_id_type.clone(),
                TypeIdType::FlowType,
            )),
        }
    }
}

impl TryFrom<&str> for FlowTypeId {
    type Error = TypeIdParseError;

    fn try_from(type_definition: &str) -> Result<Self, Self::Error> {
        let type_definition = TypeDefinition::try_from(type_definition).map_err(TypeIdParseError::TypeDefinitionParseError)?;
        if TypeIdType::FlowType != type_definition.type_id_type {
            return Err(TypeIdParseError::InvalidTypeIdType(TypeIdType::FlowType, type_definition.type_id_type));
        }
        Ok(Self((&type_definition).into()))
    }
}

impl TryFrom<&String> for FlowTypeId {
    type Error = TypeIdParseError;

    fn try_from(type_definition: &String) -> Result<Self, Self::Error> {
        Self::try_from(type_definition.as_str())
    }
}

impl TryFrom<String> for FlowTypeId {
    type Error = TypeIdParseError;

    fn try_from(type_definition: String) -> Result<Self, Self::Error> {
        Self::try_from(type_definition.as_str())
    }
}

impl Display for FlowTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FlowTypeIds(DashSet<FlowTypeId>);

impl FlowTypeIds {
    pub fn new() -> Self {
        NamespacedTypeIdContainer::new()
    }

    pub fn with_namespace<N: Into<Namespace>>(namespace: N) -> Result<NamespacedTypeIds<Self>, NamespacedTypeIdsError> {
        <Self as NamespacedTypeIdContainer>::with_namespace(namespace)
    }

    pub fn flow_type<TypeId: Into<FlowTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl NamespacedTypeIdContainer for FlowTypeIds {
    type TypeId = FlowTypeId;
    type TypeIds = Self;

    fn new() -> Self {
        Self(DashSet::new())
    }

    fn with_namespace<N: Into<Namespace>>(namespace: N) -> Result<NamespacedTypeIds<Self>, NamespacedTypeIdsError> {
        NamespacedTypeIds::new(namespace)
    }
}

impl Deref for FlowTypeIds {
    type Target = DashSet<FlowTypeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FlowTypeIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for FlowTypeIds {
    type Item = FlowTypeId;
    type IntoIter = OwningIter<FlowTypeId, RandomState>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for FlowTypeIds {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_vec();
        let other = other.to_vec();
        this.eq(&other)
    }
}

impl Eq for FlowTypeIds {}

impl Hash for FlowTypeIds {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_vec().hash(state);
    }
}

impl JsonSchema for FlowTypeIds {
    fn schema_name() -> Cow<'static, str> {
        "FlowTypeIds".into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        let sub_schema: Schema = schema_generator.subschema_for::<FlowTypeId>();
        json_schema!({
            "type": "array",
            "items": sub_schema,
        })
    }
}

impl From<Vec<FlowTypeId>> for FlowTypeIds {
    fn from(tys: Vec<FlowTypeId>) -> Self {
        FlowTypeIds(tys.into_iter().collect())
    }
}

impl From<FlowTypeIds> for Vec<FlowTypeId> {
    fn from(tys: FlowTypeIds) -> Self {
        tys.to_vec()
    }
}

impl From<&FlowTypeIds> for Vec<FlowTypeId> {
    fn from(tys: &FlowTypeIds) -> Self {
        tys.0.iter().map(|ty| ty.clone()).collect()
    }
}

impl From<DashSet<FlowTypeId>> for FlowTypeIds {
    fn from(tys: DashSet<FlowTypeId>) -> Self {
        FlowTypeIds(tys)
    }
}

impl From<&DashSet<FlowTypeId>> for FlowTypeIds {
    fn from(tys: &DashSet<FlowTypeId>) -> Self {
        FlowTypeIds(tys.clone())
    }
}

impl From<FlowTypeIds> for DashSet<FlowTypeId> {
    fn from(tys: FlowTypeIds) -> Self {
        tys.0
    }
}

impl From<NamespacedTypeIds<FlowTypeIds>> for FlowTypeIds {
    fn from(tys: NamespacedTypeIds<Self>) -> Self {
        tys.deref().clone()
    }
}

impl FromIterator<FlowTypeId> for FlowTypeIds {
    fn from_iter<I: IntoIterator<Item = FlowTypeId>>(iter: I) -> Self {
        let tys = FlowTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
    }
}

#[macro_export]
macro_rules! flow_ty {
    (
        $flow_type_id: ident,
        $namespace: ident,
        $flow_type_name_const: ident,
        $flow_type_name: expr
    ) => {
        pub const $flow_type_name_const: &str = $flow_type_name;
        pub static $flow_type_id: std::sync::LazyLock<$crate::FlowTypeId> =
            std::sync::LazyLock::new(|| $crate::FlowTypeId::new_from_type($namespace, $flow_type_name_const));
    };
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for FlowTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for FlowTypeIds {
    fn default_test() -> Self {
        let tys = FlowTypeIds::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            tys.insert(FlowTypeId::default_test());
        }
        tys
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::FlowTypeId;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn flow_type_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty = FlowTypeId::new_from_type(&namespace, &type_name);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(type_name, ty.type_name());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(nt.type_name, ty.type_name());
        assert_eq!(format!("f__{namespace}__{type_name}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::FlowType, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(type_name, type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::FlowType, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(type_name, type_definition_3.type_name());
    }

    #[test]
    fn flow_type_id_new_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty2 = FlowTypeId::new(nt.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(type_name, ty2.type_name());

        let nt2 = NamespacedType::from(&ty2);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn flow_type_id_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty1 = FlowTypeId::from(nt);
        assert_eq!(namespace, ty1.namespace());
        assert_eq!(type_name, ty1.type_name());
    }

    #[test]
    fn flow_type_id_from_string_test() {
        let s1 = String::from("f__ns__ty");
        let ty1 = FlowTypeId::try_from(&s1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.type_name());

        let s2 = String::from("r__ns__ty");
        let ty2 = FlowTypeId::try_from(&s2);
        assert!(ty2.is_err());

        let s3 = String::from("f__");
        let ty3 = FlowTypeId::try_from(&s3);
        assert!(ty3.is_err());

        let s4 = String::from("f__ns");
        let ty4 = FlowTypeId::try_from(&s4);
        assert!(ty4.is_err());

        let s5 = String::from("f__ns__");
        let ty5 = FlowTypeId::try_from(&s5);
        assert!(ty5.is_err());

        let s6 = String::from("f__ns__ty__");
        let ty6 = FlowTypeId::try_from(&s6);
        assert!(ty6.is_err());

        let s7 = String::from("f__ns__ty__xx");
        let ty7 = FlowTypeId::try_from(&s7);
        assert!(ty7.is_err());
    }

    #[test]
    fn flow_type_id_json_schema() {
        let schema = schema_for!(FlowTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
