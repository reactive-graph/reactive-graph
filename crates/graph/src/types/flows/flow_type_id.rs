use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::iter_set::OwningIter;
use dashmap::DashSet;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand_derive2::RandGen;
use schemars::gen::SchemaGenerator;
use schemars::schema::ArrayValidation;
use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeIdContainer;
use crate::NamespacedTypeIds;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct FlowTypeId(NamespacedType);

impl FlowTypeId {
    pub fn new(nt: NamespacedType) -> Self {
        Self(nt)
    }

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        Self(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for FlowTypeId {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for FlowTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
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

impl<N: Into<String>, T: Into<String>> From<(N, T)> for FlowTypeId {
    fn from(ty: (N, T)) -> Self {
        Self(NamespacedType::from(ty))
    }
}

impl TryFrom<&TypeDefinition> for FlowTypeId {
    type Error = ();

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::FlowType => Ok(FlowTypeId::new_from_type(type_definition.namespace.clone(), type_definition.type_name.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<&String> for FlowTypeId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::FlowType == type_type {
            let namespace = s.next().ok_or(())?;
            if namespace.is_empty() {
                return Err(());
            }
            let type_name = s.next().ok_or(())?;
            if type_name.is_empty() {
                return Err(());
            }
            if s.next().is_some() {
                return Err(());
            }
            return Ok(Self(NamespacedType::new(namespace, type_name)));
        }
        Err(())
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

    pub fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
        <Self as NamespacedTypeIdContainer>::with_namespace(namespace)
    }

    pub fn flow_type<TypeId: Into<FlowTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }
}

impl NamespacedTypeIdContainer for FlowTypeIds {
    type TypeId = FlowTypeId;
    type TypeIds = Self;

    fn new() -> Self {
        Self(DashSet::new())
    }

    fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
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
    fn schema_name() -> String {
        "FlowTypeIds".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<FlowTypeId>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
        .into()
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
        let mut rng = rand::thread_rng();
        for _ in 0..rng.gen_range(0..10) {
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
    use reactive_graph_test_utils::r_string;

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
