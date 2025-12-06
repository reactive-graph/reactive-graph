use std::fmt::Display;

pub const ROOT_OBJECT_NAME_QUERY: &str = "Query";

pub const ROOT_OBJECT_NAME_MUTATION: &str = "Mutation";

#[derive(Debug, Copy, Clone)]
pub enum RootObjectType {
    Interface,
    Query,
    Mutation,
}

impl Display for RootObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootObjectType::Interface => write!(f, "Interface"),
            RootObjectType::Query => write!(f, "Query"),
            RootObjectType::Mutation => write!(f, "Mutation"),
        }
    }
}
