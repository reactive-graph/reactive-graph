use std::fmt::Display;
use std::fmt::Formatter;
use uuid::Uuid;

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(schema_module = "crate::schema_graphql::schema", graphql_type = "UUID")]
pub struct UUID(pub Uuid);

impl From<Uuid> for UUID {
    fn from(id: Uuid) -> Self {
        UUID(id)
    }
}

impl From<UUID> for Uuid {
    fn from(id: UUID) -> Self {
        id.0
    }
}

impl Display for UUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
