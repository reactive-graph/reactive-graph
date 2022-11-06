use async_graphql::*;
use inexor_rgf_core_model::EntityTypeId;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::graphql::mutation::GraphQLEntityInstanceDefinition;
use crate::graphql::mutation::GraphQLRelationInstanceDefinition;
use crate::model::FlowInstance;

/// Represents a flow with entity instances and relation instances.
///
/// The entity type of the flow and the entity types of each provided entity instance must exist.
/// The relation types of each provided relation instance must exist.
#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "FlowInstanceDefinition")]
pub struct GraphQLFlowInstanceDefinition {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The namespace the entity type belongs to.
    pub namespace: String,

    /// The name of the entity type.
    pub type_name: String,

    /// The name of the flow.
    #[serde(default = "String::new")]
    pub name: String,

    /// Textual description of the flow.
    #[serde(default = "String::new")]
    pub description: String,

    /// The entity instances which are contained in this flow.
    ///
    /// It can't have a default because the wrapper entity instance must be
    /// present in the list of entities.
    pub entity_instances: Vec<GraphQLEntityInstanceDefinition>,

    /// The relation instances which are contained in this flow.
    #[serde(default = "Vec::new")]
    pub relation_instances: Vec<GraphQLRelationInstanceDefinition>,
}

impl From<GraphQLFlowInstanceDefinition> for FlowInstance {
    fn from(flow: GraphQLFlowInstanceDefinition) -> Self {
        FlowInstance {
            id: flow.id,
            ty: EntityTypeId::new_from_type(flow.namespace, flow.type_name),
            name: flow.name.clone(),
            description: flow.description.clone(),
            entity_instances: flow.entity_instances.iter().map(|entity_instance| entity_instance.clone().into()).collect(),
            relation_instances: flow
                .relation_instances
                .iter()
                .map(|relation_instance| relation_instance.clone().into())
                .collect(),
        }
    }
}
