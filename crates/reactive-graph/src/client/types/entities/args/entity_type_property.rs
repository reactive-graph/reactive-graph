use crate::client::types::entities::args::parse_entity_ty;
use clap::Args;
use reactive_graph_graph::EntityTypeId;

#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypePropertyArgs {
    /// The fully qualified namespace of the entity type.
    #[clap(name = "entity_type", value_parser = parse_entity_ty)]
    pub entity_ty: EntityTypeId,

    /// The name of the property.
    pub property_name: String,
}

// impl From<&EntityTypePropertyArgs> for PropertyContainerVariables {
//     fn from(args: &EntityTypePropertyArgs) -> Self {
//         let ty: EntityTypeId = args.ty.clone().into();
//         PropertyContainerVariables::new(ty, args.property_name.clone())
//     }
// }
