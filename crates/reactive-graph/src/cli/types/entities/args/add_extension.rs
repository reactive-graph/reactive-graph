use crate::cli::types::entities::args::type_id::EntityTypeIdArgs;
use crate::cli::types::extension::args::ExtensionDefinitionArgs;
use clap::Args;
use reactive_graph_client::schema_graphql::scalar::Json;
use reactive_graph_client::types::entities::add_extension::queries::AddExtensionVariables;
use reactive_graph_client::ExtensionDefinition;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeAddExtensionArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The extension.
    #[clap(flatten)]
    pub extension: ExtensionDefinitionArgs,
}

impl From<&EntityTypeAddExtensionArgs> for AddExtensionVariables {
    fn from(args: &EntityTypeAddExtensionArgs) -> Self {
        let extension: Json = args.extension.extension.clone().into();
        let ty: reactive_graph_graph::ExtensionTypeId = args.extension.ty.clone().into();
        let ty: reactive_graph_client::ExtensionTypeId = ty.into();
        AddExtensionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            extension: ExtensionDefinition {
                type_: ty,
                description: args.extension.description.clone(),
                extension,
            },
        }
    }
}
