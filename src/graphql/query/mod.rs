use std::sync::Arc;

use async_graphql::*;
use uuid::Uuid;

pub use flows::*;
pub use instances::*;
pub use types::*;

use crate::api::ReactiveFlowManager;
use crate::graphql::query::Instances;
use crate::graphql::query::Types;

pub mod flows;
pub mod instances;
pub mod types;

pub struct InexorQuery;

/// Search queries for the type system, the instances and the flows.
#[Object(name = "Query")]
impl InexorQuery {
    /// Search for types (components, entity types, relation types).
    async fn types(&self) -> Types {
        Types::default()
    }

    /// Search for instances (entity instances, relation instances).
    async fn instances(&self) -> Instances {
        Instances::default()
    }

    /// Search for flows and their contained instances.
    // TODO: Add query filters (flow_type)
    async fn flows(&self, context: &Context<'_>, id: Option<Uuid>) -> Vec<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>();
        if flow_manager.is_ok() {
            let flow_manager = flow_manager.unwrap();
            if id.is_some() {
                let flow = flow_manager.get(id.unwrap()).map(|flow| flow.into());
                return if flow.is_some() { vec![flow.unwrap()] } else { Vec::new() };
            }
            return flow_manager
                .get_all()
                .iter()
                // TODO: Add query filters
                .map(|flow| {
                    let flow: GraphQLFlow = flow.clone().into();
                    flow
                })
                .collect();
        }
        Vec::new()
    }
}
