use crate::cli::instances::properties::args::parse_property;
use crate::cli::instances::relations::args::id::RelationInstanceIdArgs;
use clap::Args;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateRelationInstanceArgs {
    /// The id of the relation instance.
    #[clap(flatten)]
    pub id: RelationInstanceIdArgs,

    /// The relation instance description.
    #[clap(short, long)]
    pub description: Option<String>,

    /// The relation instance properties.
    #[clap(short, long, value_parser = parse_property)]
    pub properties: Option<Vec<(String, Value)>>,
}

impl CreateRelationInstanceArgs {
    pub fn properties(&self) -> PropertyInstances {
        match &self.properties {
            None => PropertyInstances::new(),
            Some(properties) => properties.into_iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }
}
