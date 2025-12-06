use crate::client::types::components::args::parse_component_ty;
use crate::client::types::extension::args::ExtensionDefinitionArgs;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentAddExtensionArgs {
    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,

    /// The extension.
    #[clap(flatten)]
    pub extension: ExtensionDefinitionArgs,
}
