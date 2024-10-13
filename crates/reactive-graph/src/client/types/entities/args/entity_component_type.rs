use crate::client::types::components::args::type_id::ComponentContainerTypeIdArgs;
use crate::client::types::entities::args::type_id::EntityTypeIdArgs;
use clap::Args;
use reactive_graph_graph::EntityComponentTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityComponentTypeIdArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdArgs,

    /// The component type.
    #[clap(flatten)]
    pub component_ty: ComponentContainerTypeIdArgs,
}

impl From<&EntityComponentTypeIdArgs> for EntityComponentTypeId {
    fn from(args: &EntityComponentTypeIdArgs) -> Self {
        let entity_ty: reactive_graph_graph::EntityTypeId = args.ty.clone().into();
        let component_ty: reactive_graph_graph::ComponentTypeId = args.component_ty.clone().into();
        EntityComponentTypeId::new(entity_ty, component_ty)
    }
}
