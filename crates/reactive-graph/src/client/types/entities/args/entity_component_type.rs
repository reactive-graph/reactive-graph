use crate::client::types::components::args::parse_component_ty;
use crate::client::types::entities::args::parse_entity_ty;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityComponentTypeId;
use reactive_graph_graph::EntityTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityComponentTypeIdArgs {
    /// The fully qualified namespace of the entity type.
    #[clap(name = "entity-type", value_parser = parse_entity_ty)]
    pub entity_ty: EntityTypeId,

    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,
}

impl From<&EntityComponentTypeIdArgs> for EntityComponentTypeId {
    fn from(args: &EntityComponentTypeIdArgs) -> Self {
        Self::new(args.entity_ty.clone(), args.component_ty.clone())
    }
}
