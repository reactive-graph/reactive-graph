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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtensionTypeId(NamespacedType);

impl ExtensionTypeId {
    pub fn new(nt: NamespacedType) -> ExtensionTypeId {
        ExtensionTypeId(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> ExtensionTypeId {
        ExtensionTypeId(NamespacedType::new(namespace, type_name))
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

impl TryFrom<&TypeDefinition> for ExtensionTypeId {
    type Error = ();

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::Extension => Ok(ExtensionTypeId::new_from_type(type_definition.namespace.clone(), type_definition.type_name.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Identifier> for ExtensionTypeId {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
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
            return Ok(ExtensionTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
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
            return Ok(ExtensionTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for ExtensionTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
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
