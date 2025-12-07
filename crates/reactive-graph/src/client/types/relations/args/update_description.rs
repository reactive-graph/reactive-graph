use crate::client::types::relations::args::parse_relation_ty;
use clap::Args;
use reactive_graph_graph::RelationTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationTypeUpdateDescriptionArgs {
    /// The fully qualified namespace of the relation type.
    #[clap(name = "relation_type", value_parser = parse_relation_ty)]
    pub relation_ty: RelationTypeId,

    /// The description to update.
    pub description: String,
}
