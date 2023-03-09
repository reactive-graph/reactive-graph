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
pub struct FlowTypeId(NamespacedType);

impl FlowTypeId {
    pub fn new(nt: NamespacedType) -> FlowTypeId {
        FlowTypeId(nt)
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> FlowTypeId {
        FlowTypeId(NamespacedType::new(namespace, type_name))
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

impl TryFrom<&TypeDefinition> for FlowTypeId {
    type Error = ();

    fn try_from(type_definition: &TypeDefinition) -> Result<Self, Self::Error> {
        match type_definition.type_id_type {
            TypeIdType::FlowType => Ok(FlowTypeId::new_from_type(type_definition.namespace.clone(), type_definition.type_name.clone())),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Identifier> for FlowTypeId {
    type Error = ();

    fn try_from(t: &Identifier) -> Result<Self, Self::Error> {
        let s = t.to_string();
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
            return Ok(FlowTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
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
            return Ok(FlowTypeId(NamespacedType::new(namespace, type_name)));
        }
        Err(())
    }
}

impl Display for FlowTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.type_definition().to_string())
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
        lazy_static::lazy_static! {
            pub static ref $flow_type_id: $crate::FlowTypeId = $crate::FlowTypeId::new_from_type($namespace, $flow_type_name_const);
        }
    };
}
