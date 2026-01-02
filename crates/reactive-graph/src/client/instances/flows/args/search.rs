use crate::client::types::entities::args::parse_entity_ty;
use clap::Args;
use reactive_graph_client::client::instances::flows::variables::search::variables::SearchFlowInstancesVariables;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use uuid::Uuid;

/// CLI argument for searching entity instances.
#[derive(Args, Debug, Clone)]
pub(crate) struct SearchFlowInstancesArgs {
    /// The fully qualified namespace of the entity type.
    #[clap(long, name = "entity_type", value_parser = parse_entity_ty)]
    pub entity_ty: Option<EntityTypeId>,

    /// The id of the entity instance.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The label of the entity instance.
    #[clap(short, long)]
    pub label: Option<String>,
}

// impl SearchFlowInstancesArgs {}
//
impl From<&SearchFlowInstancesArgs> for SearchFlowInstancesVariables {
    fn from(args: &SearchFlowInstancesArgs) -> Self {
        SearchFlowInstancesVariables {
            _type: args.entity_ty.clone().map(|relation_ty| relation_ty.namespace().to_string()),
            id: args.id.map(From::from),
            label: args.label.clone(),
        }
        // let ty: Option<reactive_graph_graph::EntityTypeId> = search.ty.clone().into();
        // SearchFlowInstancesVariables::builder()
        //     .ty(ty.map(From::from))
        //     .id(search.id.map(From::from))
        //     .label(search.label.clone())
        //     .build()
    }
}
