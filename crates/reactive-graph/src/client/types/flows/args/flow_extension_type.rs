use crate::client::types::extension::args::parse_extension_ty;
use crate::client::types::flows::args::parse_flow_ty;
use clap::Args;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::FlowExtensionTypeId;
use reactive_graph_graph::FlowTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowExtensionTypeIdArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The fully qualified namespace of the extension.
    #[clap(name = "flow_extension", value_parser = parse_extension_ty)]
    pub extension_ty: ExtensionTypeId,
}

impl From<&FlowExtensionTypeIdArgs> for FlowExtensionTypeId {
    fn from(args: &FlowExtensionTypeIdArgs) -> Self {
        Self {
            flow_ty: args.flow_ty.clone(),
            extension_ty: args.extension_ty.clone(),
        }
    }
}
