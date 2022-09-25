use std::sync::Arc;

use async_graphql::*;
use log::error;
use log::info;
use uuid::Uuid;

pub use instances::*;
pub use types::*;

use crate::api::DynamicGraph;
use crate::graphql::query::Instances;
use crate::graphql::query::Types;

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

    async fn random_uuid(&self, _context: &Context<'_>) -> String {
        Uuid::new_v4().to_string()
    }

    async fn dynamic_graph(&self, context: &Context<'_>) -> String {
        let dynamic_graph = context.data::<Arc<dyn DynamicGraph>>();
        if dynamic_graph.is_err() {
            error!("error: {:?}", dynamic_graph.err().unwrap());
            return "{}".to_owned();
        }
        let sdl = dynamic_graph.unwrap().create_sdl();
        info!("Schema\n\n{}\n\n", sdl.to_string());
        sdl.to_string()
    }
}
