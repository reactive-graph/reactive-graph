use std::fmt::Display;
use std::fmt::Formatter;

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub enum Mutability {
    /// The property is mutable.
    Mutable,

    /// The property is immutable.
    Immutable,
}

impl From<Mutability> for reactive_graph_graph::Mutability {
    fn from(mutability: Mutability) -> Self {
        match mutability {
            Mutability::Mutable => reactive_graph_graph::Mutability::Mutable,
            Mutability::Immutable => reactive_graph_graph::Mutability::Immutable,
        }
    }
}

impl From<reactive_graph_graph::Mutability> for Mutability {
    fn from(mutability: reactive_graph_graph::Mutability) -> Self {
        match mutability {
            reactive_graph_graph::Mutability::Mutable => Mutability::Mutable,
            reactive_graph_graph::Mutability::Immutable => Mutability::Immutable,
        }
    }
}

impl Display for Mutability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", reactive_graph_graph::Mutability::from(*self))
    }
}
