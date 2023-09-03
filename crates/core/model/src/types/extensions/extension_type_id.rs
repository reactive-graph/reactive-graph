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
use serde::Deserialize;
use serde::Serialize;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

#[cfg(any(test, feature = "test"))]
use rand_derive2::RandGen;
use schemars::gen::SchemaGenerator;
use schemars::schema::{ArrayValidation, InstanceType, Schema, SchemaObject};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct ExtensionTypeId(NamespacedType);

impl ExtensionTypeId {
    pub fn new(nt: NamespacedType) -> Self {
        Self(nt)
    }

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        Self(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for ExtensionTypeId {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for ExtensionTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
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

impl <N: Into<String>, T: Into<String>> From<(N, T)> for ExtensionTypeId {
    fn from(ty: (N, T)) -> Self {
        Self(NamespacedType::from(ty))
    }
}

impl TryFrom<&TypeDefinition> for ExtensionTypeId {
    type Error = ();

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::Extension => Ok(ExtensionTypeId::new_from_type(type_definition.namespace.clone(), type_definition.type_name.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<&String> for ExtensionTypeId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::Extension == type_type {
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

impl Display for ExtensionTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ExtensionTypeIds(DashSet<ExtensionTypeId>);

impl ExtensionTypeIds {
    pub fn new() -> Self {
        ExtensionTypeIds(DashSet::new())
    }

    pub fn to_vec(&self) -> Vec<ExtensionTypeId> {
        let mut tys: Vec<ExtensionTypeId> = self.0.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
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
    fn schema_name() -> String {
        "ExtensionTypeIds".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<ExtensionTypeId>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
            .into()
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
    fn from_iter<I: IntoIterator<Item=ExtensionTypeId>>(iter: I) -> Self {
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
        lazy_static::lazy_static! {
            pub static ref $extension_type_id: $crate::ExtensionTypeId = $crate::ExtensionTypeId::new_from_type($namespace, $extension_name_const);
        }
    };
}

#[cfg(test)]
mod tests {
    use default_test::DefaultTest;
    use rand::Rng;
    use schemars::schema_for;
    use crate::{ExtensionTypeId, ExtensionTypeIds, NamespacedType, NamespacedTypeGetter, TypeDefinition, TypeDefinitionGetter, TypeIdType};
    use crate::test_utils::r_string;

    impl DefaultTest for ExtensionTypeId {
        fn default_test() -> Self {
            NamespacedType::generate_random().into()
        }
    }

    impl DefaultTest for ExtensionTypeIds {
        fn default_test() -> Self {
            let tys = ExtensionTypeIds::new();
            let mut rng = rand::thread_rng();
            for _ in 0 .. rng.gen_range(0..10) {
                tys.insert(ExtensionTypeId::default_test());
            }
            tys
        }
    }

    #[test]
    fn extension_type_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty = ExtensionTypeId::new_from_type(&namespace, &type_name);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(type_name, ty.type_name());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(nt.type_name, ty.type_name());
        assert_eq!(format!("x__{namespace}__{type_name}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::Extension, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(type_name, type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::Extension, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(type_name, type_definition_3.type_name());
    }

    #[test]
    fn extension_type_id_new_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty2 = ExtensionTypeId::new(nt.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(type_name, ty2.type_name());

        let nt2 = NamespacedType::from(&ty2);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn extension_type_id_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty1 = ExtensionTypeId::from(nt);
        assert_eq!(namespace, ty1.namespace());
        assert_eq!(type_name, ty1.type_name());
    }

    #[test]
    fn component_type_id_from_string_test() {
        let s1 = String::from("x__ns__ty");
        let ty1 = ExtensionTypeId::try_from(&s1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.type_name());

        let s2 = String::from("c__ns__ty");
        let ty2 = ExtensionTypeId::try_from(&s2);
        assert!(ty2.is_err());

        let s3 = String::from("x__");
        let ty3 = ExtensionTypeId::try_from(&s3);
        assert!(ty3.is_err());

        let s4 = String::from("x__ns");
        let ty4 = ExtensionTypeId::try_from(&s4);
        assert!(ty4.is_err());

        let s5 = String::from("x__ns__");
        let ty5 = ExtensionTypeId::try_from(&s5);
        assert!(ty5.is_err());

        let s6 = String::from("x__ns__ty__");
        let ty6 = ExtensionTypeId::try_from(&s6);
        assert!(ty6.is_err());

        let s7 = String::from("x__ns__ty__xx");
        let ty7 = ExtensionTypeId::try_from(&s7);
        assert!(ty7.is_err());
    }

    #[test]
    fn extension_type_id_json_schema() {
        let schema = schema_for!(ExtensionTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

}