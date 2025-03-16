use crate::client::instances::properties::args::parse_property;
use crate::client::types::components::args::parse_component_ty;
use crate::client::types::relations::args::type_id::RelationTypeIdOptions;
use clap::Args;
use reactive_graph_client::ComponentTypeIds;
use reactive_graph_client::PropertyInstanceDefinitions;
use reactive_graph_client::client::instances::relations::variables::search::variables::SearchRelationInstancesVariables;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use uuid::Uuid;

/// CLI argument for searching relation instances.
#[derive(Args, Debug, Clone)]
pub(crate) struct SearchRelationInstancesArgs {
    /// The id of the outbound entity instance.
    #[clap(long)]
    pub outbound_id: Option<Uuid>,

    /// The relation type.
    #[clap(flatten)]
    pub ty: RelationTypeIdOptions,

    /// The id of the inbound entity instance.
    #[clap(short, long)]
    pub inbound_id: Option<Uuid>,

    /// The properties to search for.
    #[clap(short, long, value_parser = parse_property)]
    pub properties: Option<Vec<(String, Value)>>,

    /// The components to search for.
    #[clap(short, long, value_parser = parse_component_ty)]
    pub components: Option<Vec<ComponentTypeId>>,
}

impl SearchRelationInstancesArgs {
    pub fn properties(&self) -> PropertyInstances {
        match &self.properties {
            None => PropertyInstances::new(),
            Some(properties) => properties.iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }

    pub fn components(&self) -> ComponentTypeIds {
        match &self.components {
            None => ComponentTypeIds::new(),
            Some(components) => ComponentTypeIds(components.iter().map(|ty| ty.clone().into()).collect()),
        }
    }
}

impl From<&SearchRelationInstancesArgs> for SearchRelationInstancesVariables {
    fn from(search: &SearchRelationInstancesArgs) -> Self {
        let ty: Option<reactive_graph_graph::RelationTypeId> = search.ty.clone().into();
        let properties: PropertyInstanceDefinitions = search.properties().into();
        let components: ComponentTypeIds = search.components();
        SearchRelationInstancesVariables::builder()
            .outbound_id(search.outbound_id.map(From::from))
            .ty(ty.map(From::from))
            .inbound_id(search.inbound_id.map(From::from))
            .properties(Some(properties.0))
            .components(Some(components.0))
            .build()
    }
}
