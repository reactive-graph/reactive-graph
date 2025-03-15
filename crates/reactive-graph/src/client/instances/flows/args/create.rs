use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateFlowInstanceArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

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
