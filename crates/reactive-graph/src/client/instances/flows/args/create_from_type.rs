use crate::client::instances::properties::args::parse_property;
use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateFlowInstanceFromTypeArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The id of the flow instance and the wrapper entity instance.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The entity instance properties.
    #[clap(short, long, value_parser = parse_property)]
    pub variables: Option<Vec<(String, Value)>>,

    /// The entity instance properties.
    #[clap(short, long, value_parser = parse_property)]
    pub properties: Option<Vec<(String, Value)>>,
}

impl CreateFlowInstanceFromTypeArgs {
    pub fn variables(&self) -> PropertyInstances {
        match &self.variables {
            None => PropertyInstances::new(),
            Some(variables) => variables.iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }

    pub fn properties(&self) -> PropertyInstances {
        match &self.properties {
            None => PropertyInstances::new(),
            Some(properties) => properties.iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }
}
