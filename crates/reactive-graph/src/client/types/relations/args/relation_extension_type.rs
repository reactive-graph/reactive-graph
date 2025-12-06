use crate::client::types::extension::args::parse_extension_ty;
use crate::client::types::relations::args::parse_relation_ty;
use clap::Args;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::RelationExtensionTypeId;
use reactive_graph_graph::RelationTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct RelationExtensionTypeIdArgs {
    /// The fully qualified namespace of the relation type.
    #[clap(name = "relation_type", value_parser = parse_relation_ty)]
    pub relation_ty: RelationTypeId,

    /// The fully qualified namespace of the extension.
    #[clap(name = "relation_extension", value_parser = parse_extension_ty)]
    pub extension_ty: ExtensionTypeId,
}

impl From<&RelationExtensionTypeIdArgs> for RelationExtensionTypeId {
    fn from(args: &RelationExtensionTypeIdArgs) -> Self {
        Self {
            relation_ty: args.relation_ty.clone(),
            extension_ty: args.extension_ty.clone(),
        }
    }
}
