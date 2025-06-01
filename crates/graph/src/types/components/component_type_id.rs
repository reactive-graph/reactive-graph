use std::collections::hash_map::RandomState;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::ComponentTypeIdContainer;
use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::NamespacedTypeIdContainer;
use crate::NamespacedTypeIds;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdParseError;
use crate::TypeIdType;
use dashmap::DashSet;
use dashmap::iter_set::OwningIter;
#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;
use schemars::JsonSchema;
use schemars::Schema;
use schemars::SchemaGenerator;
use schemars::json_schema;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
#[schemars(deny_unknown_fields)]
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

impl<N: Into<String>, T: Into<String>> From<(N, T)> for ComponentTypeId {
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
    type Error = TypeIdParseError;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let type_definition = TypeDefinition::try_from(s).map_err(TypeIdParseError::TypeDefinitionParseError)?;
        if TypeIdType::Component != type_definition.type_id_type {
            return Err(TypeIdParseError::InvalidTypeIdType(TypeIdType::Component, type_definition.type_id_type));
        }
        Ok(Self((&type_definition).into()))
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
        NamespacedTypeIdContainer::new()
    }

    pub fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
        <Self as NamespacedTypeIdContainer>::with_namespace(namespace)
    }

    pub fn component<TypeId: Into<ComponentTypeId>>(self, ty: TypeId) -> Self {
        self.ty(ty)
    }
}

impl NamespacedTypeIdContainer for ComponentTypeIds {
    type TypeId = ComponentTypeId;
    type TypeIds = Self;

    fn new() -> Self {
        Self(DashSet::new())
    }

    fn with_namespace<N: Into<String>>(namespace: N) -> NamespacedTypeIds<Self> {
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

#[cfg(any(test, feature = "test"))]
impl DefaultTest for ComponentTypeId {
    fn default_test() -> Self {
        NamespacedType::generate_random().into()
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for ComponentTypeIds {
    fn default_test() -> Self {
        let tys = Self::new();
        let mut rng = rand::rng();
        for _ in 0..rng.random_range(0..10) {
            tys.insert(ComponentTypeId::default_test());
        }
        tys
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::ComponentTypeId;
    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::TypeDefinition;
    use crate::TypeDefinitionGetter;
    use crate::TypeIdType;
    use reactive_graph_utils_test::r_string;

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
