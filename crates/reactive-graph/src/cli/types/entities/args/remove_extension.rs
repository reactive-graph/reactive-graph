use crate::cli::types::entities::args::type_id::EntityTypeIdArgs;
use crate::cli::types::extension::args::ExtensionTypeIdArgs;
use clap::Args;
use reactive_graph_client::types::entity_types::remove_extension::queries::RemoveExtensionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeRemoveExtensionArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The extension type.
    #[clap(flatten)]
    pub extension_ty: ExtensionTypeIdArgs,
}

impl From<&EntityTypeRemoveExtensionArgs> for RemoveExtensionVariables {
    fn from(args: &EntityTypeRemoveExtensionArgs) -> Self {
        RemoveExtensionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            extension_namespace: args.extension_ty.extension_namespace.clone(),
            extension_name: args.extension_ty.extension_name.clone(),
        }
    }
}
