use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
#[cfg(any(test, feature = "test"))]
use rand_derive2::RandGen;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use schemars::schema::ArrayValidation;
use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use serde::Deserialize;
use serde::Serialize;

use crate::ComponentTypeIdContainer;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TYPE_ID_TYPE_SEPARATOR;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct ComponentTypeId(NamespacedType);

impl ComponentTypeId {
    pub fn new(nt: NamespacedType) -> Self {
        Self(nt)
    }

    pub fn new_from_type<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> Self {
        Self(NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for ComponentTypeId {
    fn namespace(&self) -> String {
        self.0.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.0.type_name.clone()
    }
}

impl TypeDefinitionGetter for ComponentTypeId {
    fn type_definition(&self) -> TypeDefinition {
        self.into()
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

impl <N: Into<String>, T: Into<String>> From<(N, T)> for ComponentTypeId {
    fn from(ty: (N, T)) -> Self {
        Self(NamespacedType::from(ty))
    }
}

impl TryFrom<&TypeDefinition> for ComponentTypeId {
    type Error = ();

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::Component => Ok(ComponentTypeId::new_from_type(type_definition.namespace.clone(), type_definition.type_name.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<&String> for ComponentTypeId {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type = s.next().ok_or(())?.try_into()?;
        if TypeIdType::Component == type_type {
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

impl Display for ComponentTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ComponentTypeIds(DashSet<ComponentTypeId>);

impl ComponentTypeIds {
    pub fn new() -> Self {
        ComponentTypeIds(DashSet::new())
    }

    pub fn to_vec(&self) -> Vec<ComponentTypeId> {
        let mut tys: Vec<ComponentTypeId> = self.0.iter().map(|ty| ty.clone()).collect();
        tys.sort();
        tys
    }

    pub fn component<C: Into<ComponentTypeId>>(self, ty: C) -> Self {
        self.insert(ty.into());
        self
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
    fn schema_name() -> String {
        "ComponentTypeIds".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Array.into()),
            array: Some(Box::new(ArrayValidation {
                items: Some(gen.subschema_for::<ComponentTypeId>().into()),
                ..Default::default()
            })),
            ..Default::default()
        }
            .into()
    }
}

impl From<Vec<ComponentTypeId>> for ComponentTypeIds {
    fn from(tys: Vec<ComponentTypeId>) -> Self {
        ComponentTypeIds(tys.into_iter().collect())
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
        ComponentTypeIds(tys)
    }
}

impl From<&DashSet<ComponentTypeId>> for ComponentTypeIds {
    fn from(tys: &DashSet<ComponentTypeId>) -> Self {
        ComponentTypeIds(tys.clone())
    }
}

impl From<ComponentTypeIds> for DashSet<ComponentTypeId> {
    fn from(tys: ComponentTypeIds) -> Self {
        tys.0
    }
}

impl FromIterator<ComponentTypeId> for ComponentTypeIds {
    fn from_iter<I: IntoIterator<Item=ComponentTypeId>>(iter: I) -> Self {
        let tys = ComponentTypeIds::new();
        for ty in iter {
            tys.insert(ty);
        }
        tys
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
        lazy_static::lazy_static! {
            pub static ref $component_type_id: $crate::ComponentTypeId = $crate::ComponentTypeId::new_from_type($namespace, $component_name_const);
        }
    };
}

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;

#[cfg(any(test, feature = "test"))]
impl DefaultTest for ComponentTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for ComponentTypeIds {
    fn default_test() -> Self {
        let tys = ComponentTypeIds::new();
        let mut rng = rand::thread_rng();
        for _ in 0 .. rng.gen_range(0..10) {
            tys.insert(ComponentTypeId::default_test());
        }
        tys
    }
}


#[cfg(test)]
mod tests {
    // use default_test::DefaultTest;
    // use rand::Rng;
    use schemars::schema_for;

    use crate::ComponentTypeId;
    // use crate::ComponentTypeIds;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::test_utils::r_string;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;

    #[test]
    fn component_type_id_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty = ComponentTypeId::new_from_type(&namespace, &type_name);
        assert_eq!(namespace, ty.namespace());
        assert_eq!(type_name, ty.type_name());
        assert_eq!(nt.namespace, ty.namespace());
        assert_eq!(nt.type_name, ty.type_name());
        assert_eq!(format!("c__{namespace}__{type_name}"), format!("{}", ty));
        let type_definition = ty.type_definition();
        assert_eq!(TypeIdType::Component, type_definition.type_id_type);
        assert_eq!(namespace, type_definition.namespace());
        assert_eq!(type_name, type_definition.type_name());

        let type_definition_3 = TypeDefinition::from(&ty);
        assert_eq!(TypeIdType::Component, type_definition_3.type_id_type);
        assert_eq!(namespace, type_definition_3.namespace());
        assert_eq!(type_name, type_definition_3.type_name());
    }

    #[test]
    fn component_type_id_new_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty2 = ComponentTypeId::new(nt.clone());
        assert_eq!(namespace, ty2.namespace());
        assert_eq!(type_name, ty2.type_name());

        let nt2 = NamespacedType::from(&ty2);
        assert_eq!(nt, nt2);
    }

    #[test]
    fn component_type_id_from_namespaced_type_test() {
        let namespace = r_string();
        let type_name = r_string();

        let nt = NamespacedType::new(&namespace, &type_name);
        let ty1 = ComponentTypeId::from(nt);
        assert_eq!(namespace, ty1.namespace());
        assert_eq!(type_name, ty1.type_name());
    }

    #[test]
    fn component_type_id_from_string_test() {
        let s1 = String::from("c__ns__ty");
        let ty1 = ComponentTypeId::try_from(&s1).unwrap();
        assert_eq!("ns", ty1.namespace());
        assert_eq!("ty", ty1.type_name());

        let s2 = String::from("e__ns__ty");
        let ty2 = ComponentTypeId::try_from(&s2);
        assert!(ty2.is_err());

        let s3 = String::from("c__");
        let ty3 = ComponentTypeId::try_from(&s3);
        assert!(ty3.is_err());

        let s4 = String::from("c__ns");
        let ty4 = ComponentTypeId::try_from(&s4);
        assert!(ty4.is_err());

        let s5 = String::from("c__ns__");
        let ty5 = ComponentTypeId::try_from(&s5);
        assert!(ty5.is_err());

        let s6 = String::from("c__ns__ty__");
        let ty6 = ComponentTypeId::try_from(&s6);
        assert!(ty6.is_err());

        let s7 = String::from("c__ns__ty__xx");
        let ty7 = ComponentTypeId::try_from(&s7);
        assert!(ty7.is_err());
    }

    #[test]
    fn component_type_id_json_schema() {
        let schema = schema_for!(ComponentTypeId);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}