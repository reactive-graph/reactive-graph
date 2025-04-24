use crate::client::types::entities::args::type_id::EntityTypeIdArgs;
use crate::client::types::extension::args::ExtensionTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::extensions::variables::container::variables::ExtensionContainerVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityExtensionTypeIdArgs {
    /// The entity type.
    #[clap(flatten)]
    pub entity_ty: EntityTypeIdArgs,

    /// The extension type.
    #[clap(flatten)]
    pub extension_ty: ExtensionTypeIdArgs,
}

impl From<&EntityExtensionTypeIdArgs> for ExtensionContainerVariables {
    fn from(args: &EntityExtensionTypeIdArgs) -> Self {
        ExtensionContainerVariables {
            namespace: args.entity_ty.namespace.clone(),
            name: args.entity_ty.name.clone(),
            extension_namespace: args.extension_ty.extension_namespace.clone(),
            extension_name: args.extension_ty.extension_name.clone(),
        }
    }
}
