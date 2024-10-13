use crate::client::types::extension::args::ExtensionDefinitionArgs;
use crate::client::types::relations::args::type_id::RelationTypeIdArgs;
use clap::Args;
use reactive_graph_client::schema_graphql::scalar::Json;
use reactive_graph_client::types::relations::add_extension::queries::AddExtensionVariables;
use reactive_graph_client::ExtensionDefinition;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationTypeAddExtensionArgs {
    /// The relation type.
    #[clap(flatten)]
    pub ty: RelationTypeIdArgs,

    /// The extension.
    #[clap(flatten)]
    pub extension: ExtensionDefinitionArgs,
}

impl From<&RelationTypeAddExtensionArgs> for AddExtensionVariables {
    fn from(args: &RelationTypeAddExtensionArgs) -> Self {
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
