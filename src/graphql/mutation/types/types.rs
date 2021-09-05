use async_graphql::*;

use crate::graphql::mutation::{MutationComponents, MutationEntityTypes, MutationRelationTypes};

#[derive(Default)]
pub struct MutationTypes;

/// Mutations for types (components, entity types, relation types).
#[Object]
impl MutationTypes {
    /// Mutations for components
    async fn components(&self) -> MutationComponents {
        MutationComponents::default()
    }

    /// Mutations for entity types
    async fn entities(&self) -> MutationEntityTypes {
        MutationEntityTypes::default()
    }

    /// Mutations for relation types
    async fn relations(&self) -> MutationRelationTypes {
        MutationRelationTypes::default()
    }
}
