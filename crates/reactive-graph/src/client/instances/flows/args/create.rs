use crate::client::types::flows::args::parse_flow_ty;
use clap::Args;
use reactive_graph_graph::FlowTypeId;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateFlowInstanceArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The entity instance id.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The name of the flow instance.
    #[clap(short, long)]
    pub name: Option<String>,

    /// Textual description of the flow instance.
    #[clap(short, long)]
    pub description: Option<String>,
}

impl CreateFlowInstanceArgs {
    // pub fn properties(&self) -> PropertyInstances {
    //     match &self.properties {
    //         None => PropertyInstances::new(),
    //         Some(properties) => properties.iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
    //     }
    // }
}
