use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use clap::Args;
use reactive_graph_client::types::common::variables::type_id::variables::TypeIdVariables;
use reactive_graph_graph::FlowTypeId;

/// The flow type.
#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeIdArgs {
    /// The flow type namespace.
    pub namespace: String,

    /// The flow type name.
    pub name: String,
}

impl FlowTypeIdArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("FlowType {}__{} not found", &self.namespace, &self.name))
    }
}

impl From<FlowTypeIdArgs> for FlowTypeId {
    fn from(ty: FlowTypeIdArgs) -> Self {
        FlowTypeId::new_from_type(ty.namespace, ty.name)
    }
}

impl From<&FlowTypeIdArgs> for TypeIdVariables {
    fn from(ty: &FlowTypeIdArgs) -> Self {
        let ty: FlowTypeId = ty.clone().into();
        ty.into()
    }
}

/// The outbound flow type.
#[derive(Args, Debug, Clone)]
pub(crate) struct OutboundFlowTypeIdArgs {
    /// The outbound flow type namespace.
    pub outbound_type_namespace: String,

    /// The outbound flow type name.
    pub outbound_type_name: String,
}

/// The inbound flow type.
#[derive(Args, Debug, Clone)]
pub(crate) struct InboundFlowTypeIdArgs {
    /// The inbound flow type namespace.
    pub inbound_type_namespace: String,

    /// The inbound flow type name.
    pub inbound_type_name: String,
}

/// The flow type as option.
#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeIdOptions {
    /// The flow type namespace.
    #[clap(long)]
    pub namespace: Option<String>,

    /// The flow type name.
    #[clap(short, long)]
    pub name: Option<String>,
}

impl From<FlowTypeIdOptions> for Option<FlowTypeId> {
    fn from(ty: FlowTypeIdOptions) -> Self {
        if ty.namespace.is_none() && ty.name.is_none() {
            return None;
        }
        Some(FlowTypeId::new_from_type(ty.namespace.unwrap_or_default(), ty.name.unwrap_or_default()))
    }
}
