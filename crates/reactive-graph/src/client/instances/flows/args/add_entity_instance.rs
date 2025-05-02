use crate::client::instances::properties::args::parse_property;
use clap::Args;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct AddEntityInstanceArgs {
    /// The entity type namespace.
    pub entity_type_namespace: String,

    /// The entity type name.
    pub entity_type_name: String,

    /// The entity instance id.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The entity instance description.
    #[clap(short, long)]
    pub description: Option<String>,

    /// The entity instance properties.
    #[clap(short, long, value_parser = parse_property)]
    pub properties: Option<Vec<(String, Value)>>,
    // TODO: The entity instance extensions.
    // #[clap(short, long, value_parser = parse_extension)]
    // pub extensions: Option<Vec<(String, Value)>>,
}

impl AddEntityInstanceArgs {
    pub fn properties(&self) -> PropertyInstances {
        match &self.properties {
            None => PropertyInstances::new(),
            Some(properties) => properties.iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }
}
