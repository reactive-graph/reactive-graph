use crate::client::types::entities::args::parse_entity_ty;
use clap::Args;
use reactive_graph_graph::EntityTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeUpdateDescriptionArgs {
    /// The fully qualified namespace of the entity type.
    #[clap(name = "entity_type", value_parser = parse_entity_ty)]
    pub entity_ty: EntityTypeId,

    /// The description to update.
    pub description: String,
}
