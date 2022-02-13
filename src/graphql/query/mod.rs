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
    async fn flows(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by the id of the flow")] id: Option<Uuid>,
        #[graphql(name = "type", desc = "Filters by the flow type")] flow_type: Option<String>,
    ) -> Vec<GraphQLFlow> {
        if let Ok(flow_manager) = context.data::<Arc<dyn ReactiveFlowManager>>() {
            if id.is_some() {
                return match flow_manager.get(id.unwrap()).map(|flow| flow.into()) {
                    Some(flow) => vec![flow],
                    None => Vec::new(),
                };
            }
            return flow_manager
                .get_all()
                .iter()
                .filter(|flow| flow_type.is_none() || flow_type.clone().unwrap() == flow.type_name)
                .map(|flow| {
                    let flow: GraphQLFlow = flow.clone().into();
                    flow
                })
                .collect();
        }
        Vec::new()
    }
}
