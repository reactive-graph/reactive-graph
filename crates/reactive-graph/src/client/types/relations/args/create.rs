use crate::client::types::entities::args::parse_entity_ty;
use crate::client::types::relations::args::parse_relation_ty;
use clap::Args;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::RelationTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateRelationTypeArgs {
    /// The fully qualified namespace of the outbound entity type.
    #[clap(name = "outbound", value_parser = parse_entity_ty)]
    pub outbound_ty: EntityTypeId,

    /// The fully qualified namespace of the relation type.
    #[clap(name = "relation_type", value_parser = parse_relation_ty)]
    pub relation_ty: RelationTypeId,

    /// The fully qualified namespace of the inbound entity type.
    #[clap(name = "inbound", value_parser = parse_entity_ty)]
    pub inbound_ty: EntityTypeId,

    /// The relation type description.
    #[clap(short, long)]
    pub description: Option<String>,
}
