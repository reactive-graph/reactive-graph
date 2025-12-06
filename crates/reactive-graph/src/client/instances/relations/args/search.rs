use crate::client::instances::properties::args::parse_property;
use crate::client::types::components::args::parse_component_ty;
use crate::client::types::relations::args::parse_relation_ty;
use clap::Args;
use reactive_graph_client::PropertyInstanceDefinitions;
use reactive_graph_client::client::instances::relations::variables::search::variables::SearchRelationInstancesVariables;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::NamespacedTypeIdContainer;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::RelationTypeId;
use serde_json::Value;
use uuid::Uuid;

/// CLI argument for searching relation instances.
#[derive(Args, Debug, Clone)]
pub(crate) struct SearchRelationInstancesArgs {
    /// The id of the outbound entity instance.
    #[clap(long)]
    pub outbound_id: Option<Uuid>,

    /// The fully qualified namespace of the relation type.
    #[clap(name = "relation_type", value_parser = parse_relation_ty)]
    pub relation_ty: Option<RelationTypeId>,

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
    fn properties(&self) -> Option<PropertyInstances> {
        match &self.properties {
            None => None,
            Some(properties) => Some(properties.iter().map(|(name, value)| (name.clone(), value.clone())).collect()),
        }
    }

    fn components(&self) -> Option<ComponentTypeIds> {
        match &self.components {
            None => None,
            Some(components) => Some(ComponentTypeIds::from_iter(components.iter().map(|ty| ty.clone()))),
        }
        // self.components
        //     .clone()
        //     .map(|components| ComponentTypeIds::from_iter(components.iter().map(|ty| ty.clone())))
    }
}

impl From<&SearchRelationInstancesArgs> for SearchRelationInstancesVariables {
    fn from(args: &SearchRelationInstancesArgs) -> Self {
        SearchRelationInstancesVariables {
            outbound_id: args.outbound_id.map(From::from),
            _type: args.relation_ty.clone().map(|relation_ty| relation_ty.namespace().to_string()),
            inbound_id: args.inbound_id.map(From::from),
            properties: args.properties().map(|properties| PropertyInstanceDefinitions::from(properties).0),
            components: args.components().map(|components| components.into_fully_qualified_namespaces()),
        }
    }
}
