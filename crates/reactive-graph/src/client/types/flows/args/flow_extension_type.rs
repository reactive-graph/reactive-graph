use crate::client::types::extension::args::ExtensionTypeIdArgs;
use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::extensions::variables::container::variables::ExtensionContainerVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowExtensionTypeIdArgs {
    /// The flow type.
    #[clap(flatten)]
    pub flow_ty: FlowTypeIdArgs,

    /// The extension type.
    #[clap(flatten)]
    pub extension_ty: ExtensionTypeIdArgs,
}

impl From<&FlowExtensionTypeIdArgs> for ExtensionContainerVariables {
    fn from(args: &FlowExtensionTypeIdArgs) -> Self {
        ExtensionContainerVariables {
            namespace: args.flow_ty.namespace.clone(),
            name: args.flow_ty.name.clone(),
            extension_namespace: args.extension_ty.extension_namespace.clone(),
            extension_name: args.extension_ty.extension_name.clone(),
        }
    }
}
