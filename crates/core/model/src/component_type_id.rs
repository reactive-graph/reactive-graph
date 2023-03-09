use std::fmt::Display;
use std::fmt::Formatter;

use indradb::Identifier;
use serde::Deserialize;
use serde::Serialize;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeDefinition;
use crate::TypeDefinitionGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComponentTypeId(NamespacedType);

impl ComponentTypeId {
    pub fn new(nt: NamespacedType) -> ComponentTypeId {
        ComponentTypeId(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> ComponentTypeId {
        ComponentTypeId(NamespacedType::new(namespace, type_name))
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
        ComponentTypeId(nt)
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

impl TryFrom<&Identifier> for ComponentTypeId {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
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
            return Ok(ComponentTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
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
            return Ok(ComponentTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for ComponentTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
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
