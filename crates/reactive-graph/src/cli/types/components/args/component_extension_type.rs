use crate::cli::types::components::args::ComponentTypeIdArgs;
use crate::cli::types::extension::args::ExtensionTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::extensions::container::queries::ExtensionContainerVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentExtensionTypeIdArgs {
    /// The component type.
    #[clap(flatten)]
    pub component_ty: ComponentTypeIdArgs,

    /// The extension type.
    #[clap(flatten)]
    pub extension_ty: ExtensionTypeIdArgs,
}

impl From<&ComponentExtensionTypeIdArgs> for ExtensionContainerVariables {
    fn from(args: &ComponentExtensionTypeIdArgs) -> Self {
        ExtensionContainerVariables {
            namespace: args.component_ty.namespace.clone(),
            name: args.component_ty.name.clone(),
            extension_namespace: args.extension_ty.extension_namespace.clone(),
            extension_name: args.extension_ty.extension_name.clone(),
        }
    }
}