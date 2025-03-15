use async_graphql::*;

use crate::mutation::MutationComponents;
use crate::mutation::MutationEntityTypes;
use crate::mutation::MutationFlowTypes;
use crate::mutation::MutationRelationTypes;

#[derive(Default)]
pub struct MutationTypes;

/// Mutations for types (components, entity types, relation types and flow types).
#[Object]
impl MutationTypes {
    /// Mutations for components
    async fn components(&self) -> MutationComponents {
        MutationComponents
    }

    /// Mutations for entity types
    async fn entities(&self) -> MutationEntityTypes {
        MutationEntityTypes
    }

    /// Mutations for relation types
    async fn relations(&self) -> MutationRelationTypes {
        MutationRelationTypes
    }

    /// Mutations for flow types
    async fn flows(&self) -> MutationFlowTypes {
        MutationFlowTypes
    }
}
