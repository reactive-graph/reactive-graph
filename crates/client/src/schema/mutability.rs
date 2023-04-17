use std::fmt::Display;
use std::fmt::Formatter;
use tabled::Tabled;

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
#[derive(Tabled)]
pub enum Mutability {
    /// The property is mutable.
    Mutable,

    /// The property is immutable.
    Immutable,
}

impl From<Mutability> for crate::model::Mutability {
    fn from(mutability: Mutability) -> Self {
        match mutability {
            Mutability::Mutable => crate::model::Mutability::Mutable,
            Mutability::Immutable => crate::model::Mutability::Immutable,
        }
    }
}

impl From<crate::model::Mutability> for Mutability {
    fn from(mutability: crate::model::Mutability) -> Self {
        match mutability {
            crate::model::Mutability::Mutable => Mutability::Mutable,
            crate::model::Mutability::Immutable => Mutability::Immutable,
        }
    }
}

impl Display for Mutability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::model::Mutability::from(self.clone()))
    }
}
