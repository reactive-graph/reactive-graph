use crate::cli::instances::properties::args::parse_property;
use crate::cli::types::components::args::parse_component_ty;
use crate::cli::types::entities::args::type_id::EntityTypeIdOptions;
use clap::Args;
use reactive_graph_client::client::instances::entities::queries::search::queries::SearchEntityInstancesVariables;
use reactive_graph_client::ComponentTypeIds;
use reactive_graph_client::PropertyInstanceDefinitions;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::PropertyInstances;
use serde_json::Value;
use uuid::Uuid;

/// CLI argument for searching entity instances.
#[derive(Args, Debug, Clone)]
pub(crate) struct SearchEntityInstancesArgs {
    /// The entity type.
    #[clap(flatten)]
    pub ty: EntityTypeIdOptions,

    /// The id of the entity instance.
    #[clap(short, long)]
    pub id: Option<Uuid>,

    /// The label of the entity instance.
    #[clap(short, long)]
    pub label: Option<String>,

    /// The properties to search for.
    #[clap(short, long, value_parser = parse_property)]
    pub properties: Option<Vec<(String, Value)>>,

    /// The components to search for.
    #[clap(short, long, value_parser = parse_component_ty)]
    pub components: Option<Vec<ComponentTypeId>>,
}

impl SearchEntityInstancesArgs {
    pub fn properties(&self) -> PropertyInstances {
        match &self.properties {
            None => PropertyInstances::new(),
            Some(properties) => properties.into_iter().map(|(name, value)| (name.clone(), value.clone())).collect(),
        }
    }

    pub fn components(&self) -> ComponentTypeIds {
        match &self.components {
            None => ComponentTypeIds::new(),
            Some(components) => ComponentTypeIds(components.iter().map(|ty| ty.clone().into()).collect()),
        }
    }
}

impl From<&SearchEntityInstancesArgs> for SearchEntityInstancesVariables {
    fn from(search: &SearchEntityInstancesArgs) -> Self {
        let ty: Option<reactive_graph_graph::EntityTypeId> = search.ty.clone().into();
        let properties: PropertyInstanceDefinitions = search.properties().into();
        let properties = Some(properties.0);
        let components: ComponentTypeIds = search.components().into();
        let components = Some(components.0);
        SearchEntityInstancesVariables::builder()
            .ty(ty.map(From::from))
            .id(search.id.map(From::from))
            .label(search.label.clone())
            .properties(properties)
            .components(components)
            .build()
    }
}
