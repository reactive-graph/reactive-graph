use crate::cli::types::components::args::ComponentTypeIdArgs;
use crate::cli::types::extension::args::ExtensionTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::components::remove_extension::queries::RemoveExtensionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentRemoveExtensionArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The extension type.
    #[clap(flatten)]
    pub extension_ty: ExtensionTypeIdArgs,
}

impl From<&ComponentRemoveExtensionArgs> for RemoveExtensionVariables {
    fn from(args: &ComponentRemoveExtensionArgs) -> Self {
        RemoveExtensionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            extension_namespace: args.extension_ty.extension_namespace.clone(),
            extension_name: args.extension_ty.extension_name.clone(),
        }
    }
}
