use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

/// Grants access to the namespace and the type name of a type of types.
pub trait NamespacedTypeGetter {
    /// Returns the namespace of the type.
    fn namespace(&self) -> String;

    /// Returns the name of the type.
    fn type_name(&self) -> String;
}

/// Defines the namespace and the name of a type.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NamespacedType {
    /// The namespace the component belongs to.
    #[serde(default = "String::new")]
    pub namespace: String,

    /// The name of the type.
    #[serde(alias = "name")]
    pub type_name: String,
}

impl NamespacedType {
    /// Constructs a new namespaced type.
    pub fn new<S: Into<String>>(namespace: S, type_name: S) -> NamespacedType {
        NamespacedType {
            namespace: namespace.into(),
            type_name: type_name.into(),
        }
    }
}

impl NamespacedTypeGetter for NamespacedType {
    /// Returns the namespace of the namespaced type.
    fn namespace(&self) -> String {
        self.namespace.clone()
    }

    /// Returns the name of the namespaced type.
    fn type_name(&self) -> String {
        self.type_name.clone()
    }
}

impl Display for NamespacedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}__{}", &self.namespace, self.type_name)
    }
}
