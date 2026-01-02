use crate::client::types::components::args::parse_component_ty;
use crate::client::types::relations::args::parse_relation_ty;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::RelationComponentTypeId;
use reactive_graph_graph::RelationTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationComponentTypeIdArgs {
    /// The fully qualified namespace of the relation type.
    #[clap(name = "relation_type", value_parser = parse_relation_ty)]
    pub relation_ty: RelationTypeId,

    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,
}

impl From<&RelationComponentTypeIdArgs> for RelationComponentTypeId {
    fn from(args: &RelationComponentTypeIdArgs) -> Self {
        Self::new(args.relation_ty.clone(), args.component_ty.clone())
    }
}
