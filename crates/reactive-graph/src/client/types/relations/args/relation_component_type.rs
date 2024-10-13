use crate::client::types::components::args::type_id::ComponentContainerTypeIdArgs;
use crate::client::types::relations::args::type_id::RelationTypeIdArgs;
use clap::Args;
use reactive_graph_graph::RelationComponentTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationComponentTypeIdArgs {
    /// The relation type.
    #[clap(flatten)]
    pub ty: RelationTypeIdArgs,

    /// The component type.
    #[clap(flatten)]
    pub component_ty: ComponentContainerTypeIdArgs,
}

impl From<&RelationComponentTypeIdArgs> for RelationComponentTypeId {
    fn from(args: &RelationComponentTypeIdArgs) -> Self {
        let relation_ty: reactive_graph_graph::RelationTypeId = args.ty.clone().into();
        let component_ty: reactive_graph_graph::ComponentTypeId = args.component_ty.clone().into();
        RelationComponentTypeId::new(relation_ty, component_ty)
    }
}
