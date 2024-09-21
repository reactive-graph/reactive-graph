use crate::cli::instances::properties::args::parse_property;
use crate::cli::types::entities::args::type_id::EntityTypeIdArgs;
use clap::Args;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateEntityInstanceArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The entity instance id.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The entity instance description.
    #[clap(short, long)]
    pub description: Option<String>,

    /// The entity instance properties.
    #[clap(short, long, value_parser = parse_property)]
    pub properties: Option<Vec<(String, Value)>>,
}

impl CreateEntityInstanceArgs {
    pub fn properties(&self) -> PropertyInstances {
        match &self.properties {
            None => PropertyInstances::new(),
            Some(properties) => properties.iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }
}
