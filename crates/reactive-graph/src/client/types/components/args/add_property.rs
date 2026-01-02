use crate::client::types::components::args::parse_component_ty;
use crate::client::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentAddPropertyArgs {
    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,
    /// The property.
    #[clap(flatten)]
    pub property_type: PropertyTypeDefinitionArgs,
}
