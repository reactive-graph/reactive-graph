use crate::client::types::components::args::ComponentTypeIdArgs;
use crate::client::types::extension::args::ExtensionDefinitionArgs;
use clap::Args;
use reactive_graph_client::ExtensionDefinition;
use reactive_graph_client::schema_graphql::scalar::Json;
use reactive_graph_client::types::extensions::variables::add_extension::variables::AddExtensionVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentAddExtensionArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The extension.
    #[clap(flatten)]
    pub extension: ExtensionDefinitionArgs,
}

impl From<&ComponentAddExtensionArgs> for AddExtensionVariables {
    fn from(args: &ComponentAddExtensionArgs) -> Self {
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
