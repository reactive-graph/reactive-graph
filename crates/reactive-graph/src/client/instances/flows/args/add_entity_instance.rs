use crate::client::instances::properties::args::parse_property;
use crate::client::types::entities::args::parse_entity_ty;
use clap::Args;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct AddEntityInstanceArgs {
    /// The fully qualified namespace of the entity type.
    #[clap(name = "entity_type", value_parser = parse_entity_ty)]
    pub entity_ty: EntityTypeId,

    /// The entity instance id.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The entity instance description.
    #[clap(short, long)]
    pub description: Option<String>,

    /// The entity instance properties.
    #[clap(short, long, value_parser = parse_property)]
    pub properties: Option<Vec<(String, Value)>>,
    // TODO: The entity instance extensions.
    // #[clap(short, long, value_parser = parse_extension)]
    // pub extensions: Option<Vec<(String, Value)>>,
}

impl AddEntityInstanceArgs {
    pub fn properties(&self) -> PropertyInstances {
        match &self.properties {
            None => PropertyInstances::new(),
            Some(properties) => properties.iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }
}

impl From<&AddEntityInstanceArgs> for EntityInstance {
    fn from(args: &AddEntityInstanceArgs) -> Self {
        Self {
            ty: args.entity_ty.clone(),
            id: args.id.unwrap_or_else(Uuid::new_v4),
            name: Default::default(),
            description: args.description.clone().unwrap_or_default(),
            properties: args.properties(),
            components: Default::default(),
            extensions: Default::default(),
        }
    }
}
