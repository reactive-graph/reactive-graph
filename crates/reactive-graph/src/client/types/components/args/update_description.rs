// use crate::client::types::components::args::ComponentTypeIdArgs;
use crate::client::types::components::args::parse_component_ty;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentUpdateDescriptionArgs {
    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,
    // /// The component type.
    // #[clap(flatten)]
    // pub ty: ComponentTypeIdArgs,
    /// The description to update.
    pub description: String,
}

// impl From<&ComponentUpdateDescriptionArgs> for UpdateDescriptionVariables {
//     fn from(args: &ComponentUpdateDescriptionArgs) -> Self {
//         UpdateDescriptionVariables {
//             namespace: args.ty.namespace.clone(),
//             name: args.ty.name.clone(),
//             description: args.description.clone(),
//         }
//     }
// }
