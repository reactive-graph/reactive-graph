use core::fmt;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "docs"))]
use documented::DocumentedVariants;
#[cfg(any(test, feature = "test"))]
use rand_derive3::RandGen;
#[cfg(any(test, feature = "table"))]
use tabled::Tabled;

/// The mutability of a property.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(any(test, feature = "test"), derive(RandGen))]
#[cfg_attr(any(test, feature = "table"), derive(Tabled))]
#[cfg_attr(any(test, feature = "docs"), derive(DocumentedVariants))]
pub enum Mutability {
    /// The property is mutable.
    Mutable,

    /// The property is immutable.
    Immutable,
}

impl Mutability {
    pub fn mutable() -> Self {
        Mutability::Mutable
    }
    pub fn immutable() -> Self {
        Mutability::Immutable
    }
}

impl From<&str> for Mutability {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "mutable" => Self::Mutable,
            "immutable" => Self::Immutable,
            _ => Self::Mutable,
        }
    }
}

impl Display for Mutability {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultTest for Mutability {
    fn default_test() -> Self {
        Mutability::generate_random()
    }
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::Mutability;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn mutability_should_be_created_using_static_method_call() {
        assert_eq!(Mutability::Mutable, Mutability::mutable());
        assert_eq!(Mutability::Immutable, Mutability::immutable());
    }

    #[test]
    fn mutability_from_str() {
        assert_eq!(Mutability::Mutable, Mutability::from("mutable"));
        assert_eq!(Mutability::Mutable, Mutability::from("Mutable"));
        assert_eq!(Mutability::Mutable, Mutability::from("MUTABLE"));

        assert_eq!(Mutability::Immutable, Mutability::from("immutable"));
        assert_eq!(Mutability::Immutable, Mutability::from("Immutable"));
        assert_eq!(Mutability::Immutable, Mutability::from("IMMUTABLE"));

        // Fallback to String
        assert_eq!(Mutability::Mutable, Mutability::from(r_string().as_str()));
    }

    #[test]
    fn mutability_display() {
        assert_eq!("Mutable", format!("{}", Mutability::Mutable));
        assert_eq!("Immutable", format!("{}", Mutability::Immutable));
    }

    #[test]
    fn data_type_json_schema() {
        let schema = schema_for!(Mutability);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
