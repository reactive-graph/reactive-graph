use crate::client::types::entities::args::type_id::EntityTypeIdOptions;
use clap::Args;
use reactive_graph_client::client::instances::flows::variables::search::variables::SearchFlowInstancesVariables;
use uuid::Uuid;

/// CLI argument for searching entity instances.
#[derive(Args, Debug, Clone)]
pub(crate) struct SearchFlowInstancesArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdOptions,

    /// The id of the entity instance.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The label of the entity instance.
    #[clap(short, long)]
    pub label: Option<String>,
}

impl SearchFlowInstancesArgs {}

impl From<&SearchFlowInstancesArgs> for SearchFlowInstancesVariables {
    fn from(search: &SearchFlowInstancesArgs) -> Self {
        let ty: Option<reactive_graph_graph::EntityTypeId> = search.ty.clone().into();
        SearchFlowInstancesVariables::builder()
            .ty(ty.map(From::from))
            .id(search.id.map(From::from))
            .label(search.label.clone())
            .build()
    }
}
