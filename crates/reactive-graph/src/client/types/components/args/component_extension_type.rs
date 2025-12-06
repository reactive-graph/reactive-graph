use crate::client::types::components::args::parse_component_ty;
use crate::client::types::extension::args::parse_extension_ty;
use clap::Args;
use reactive_graph_graph::ComponentExtensionTypeId;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ExtensionTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentExtensionTypeIdArgs {
    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,

    /// The fully qualified namespace of the extension.
    #[clap(name = "component_extension", value_parser = parse_extension_ty)]
    pub extension_ty: ExtensionTypeId,
}

impl From<&ComponentExtensionTypeIdArgs> for ComponentExtensionTypeId {
    fn from(args: &ComponentExtensionTypeIdArgs) -> Self {
        Self {
            component_ty: args.component_ty.clone(),
            extension_ty: args.extension_ty.clone(),
        }
    }
}
