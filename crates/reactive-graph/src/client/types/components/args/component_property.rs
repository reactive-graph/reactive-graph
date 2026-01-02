// use crate::client::types::components::args::ComponentTypeIdArgs;
use crate::client::types::components::args::parse_component_ty;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentPropertyArgs {
    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,
    // /// The component type.
    // #[clap(flatten)]
    // pub ty: ComponentTypeIdArgs,
    /// The name of the property.
    pub property_name: String,
}

// impl From<&ComponentPropertyArgs> for PropertyContainerVariables {
//     fn from(args: &ComponentPropertyArgs) -> Self {
//         let ty: ComponentTypeId = args.ty.clone().into();
//         PropertyContainerVariables::new(ty, args.property_name.clone())
//     }
// }
