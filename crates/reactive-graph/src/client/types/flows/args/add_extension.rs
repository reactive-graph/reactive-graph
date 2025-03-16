use crate::client::types::extension::args::ExtensionDefinitionArgs;
use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::ExtensionDefinition;
use reactive_graph_client::schema_graphql::scalar::Json;
use reactive_graph_client::types::flows::add_extension::queries::AddExtensionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeAddExtensionArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The extension.
    #[clap(flatten)]
    pub extension: ExtensionDefinitionArgs,
}

impl From<&FlowTypeAddExtensionArgs> for AddExtensionVariables {
    fn from(args: &FlowTypeAddExtensionArgs) -> Self {
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
