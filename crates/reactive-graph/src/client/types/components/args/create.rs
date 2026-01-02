use crate::client::types::components::args::parse_component_ty;
use clap::Args;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::PropertyTypes;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateComponentArgs {
    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,

    /// The component description.
    #[clap(short, long)]
    pub description: Option<String>,
    // TODO: Create component with properties
    // TODO: Create component with extensions
}

impl From<CreateComponentArgs> for Component {
    fn from(args: CreateComponentArgs) -> Self {
        Self {
            ty: args.component_ty,
            description: args.description.clone().unwrap_or_default(),
            properties: PropertyTypes::new(),
            extensions: Extensions::new(),
        }
    }
}
