use crate::client::types::entities::args::parse_entity_ty;
use crate::client::types::extension::args::parse_extension_ty;
use clap::Args;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::entity_extension_type_id::EntityExtensionTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityExtensionTypeIdArgs {
    /// The fully qualified namespace of the entity type.
    #[clap(name = "entity_type", value_parser = parse_entity_ty)]
    pub entity_ty: EntityTypeId,

    /// The fully qualified namespace of the extension.
    #[clap(name = "entity_extension", value_parser = parse_extension_ty)]
    pub extension_ty: ExtensionTypeId,
}

impl From<&EntityExtensionTypeIdArgs> for EntityExtensionTypeId {
    fn from(args: &EntityExtensionTypeIdArgs) -> Self {
        Self {
            entity_ty: args.entity_ty.clone(),
            extension_ty: args.extension_ty.clone(),
        }
    }
}
