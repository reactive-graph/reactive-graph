use std::fmt::Display;
use std::fmt::Formatter;

use crate::TYPE_ID_TYPE_SEPARATOR;
#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// Grants access to the namespace and the type name of a type of types.
pub trait NamespacedTypeGetter {
    /// Returns the namespace of the type.
    fn namespace(&self) -> String;

    /// Returns the name of the type.
    fn type_name(&self) -> String;
}

/// Defines the namespace and the name of a type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema, TypedBuilder)]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
pub struct NamespacedType {
    /// The namespace of the type.
    #[schemars(required)]
    pub namespace: String,

    /// The name of the type.
    #[serde(alias = "name")]
    #[schemars(required)]
    pub type_name: String,
}

impl NamespacedType {
    /// Constructs a new namespaced type.
    pub fn new<N: Into<String>, T: Into<String>>(namespace: N, type_name: T) -> NamespacedType {
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
        write!(f, "{}{}{}", &self.namespace, TYPE_ID_TYPE_SEPARATOR, self.type_name)
    }
}

impl<N: Into<String>, T: Into<String>> From<(N, T)> for NamespacedType {
    fn from(ty: (N, T)) -> Self {
        NamespacedType::new(ty.0.into(), ty.1.into())
    }
}

#[cfg(test)]
pub mod tests {
    use default_test::DefaultTest;
    use schemars::schema_for;

    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use reactive_graph_utils_test::r_string;

    impl DefaultTest for NamespacedType {
        fn default_test() -> Self {
            NamespacedType::new(r_string(), r_string())
        }
    }

    #[test]
    fn namespaced_type_from_str_test() {
        let namespace = r_string();
        let type_name = r_string();
        let nt = NamespacedType::new(&namespace, &type_name);
        assert_eq!(namespace, nt.namespace());
        assert_eq!(type_name, nt.type_name());
        assert_eq!(format!("{namespace}__{type_name}"), format!("{}", nt));
    }

    #[test]
    fn namespaced_type_json_schema() {
        let schema = schema_for!(NamespacedType);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
