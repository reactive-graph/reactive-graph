use crate::client::types::extension::args::ExtensionTypeIdArgs;
use crate::client::types::relations::args::type_id::RelationTypeIdArgs;
use clap::Args;
use reactive_graph_client::client::types::extensions::container::queries::ExtensionContainerVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationExtensionTypeIdArgs {
    /// The relation type.
    #[clap(flatten)]
    pub relation_ty: RelationTypeIdArgs,

    /// The extension type.
    #[clap(flatten)]
    pub extension_ty: ExtensionTypeIdArgs,
}

impl From<&RelationExtensionTypeIdArgs> for ExtensionContainerVariables {
    fn from(args: &RelationExtensionTypeIdArgs) -> Self {
        ExtensionContainerVariables {
            namespace: args.relation_ty.namespace.clone(),
            name: args.relation_ty.name.clone(),
            extension_namespace: args.extension_ty.extension_namespace.clone(),
            extension_name: args.extension_ty.extension_name.clone(),
        }
    }
}
