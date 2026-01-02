use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityInstances;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::NamespacedTypeParseError;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceTypeIdError;
use reactive_graph_graph::RelationInstances;

use crate::mutation::GraphQLEntityInstanceDefinition;
use crate::mutation::GraphQLRelationInstanceDefinition;

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

    /// The fully qualified namespace of the entity type of the wrapper entity.
    #[graphql(name = "entityType")]
    pub _type: String,

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

impl GraphQLFlowInstanceDefinition {
    fn entity_instances(&self) -> Result<EntityInstances, NamespacedTypeParseError> {
        let entity_instances = EntityInstances::new();
        for entity_instance in self.entity_instances.iter() {
            entity_instances.push(EntityInstance::try_from(entity_instance.clone())?);
        }
        Ok(entity_instances)
    }

    fn relation_instances(&self) -> Result<RelationInstances, RelationInstanceTypeIdError> {
        let relation_instances = RelationInstances::new();
        for relation_instance in self.relation_instances.iter() {
            relation_instances.push(RelationInstance::try_from(relation_instance.clone())?);
        }
        Ok(relation_instances)
    }
}

#[derive(Debug, Error)]
pub enum GraphQLFlowInstanceDefinitionError {
    #[error("The namespaced type is not a valid namespace: {0}")]
    NamespacedTypeParseError(#[from] NamespacedTypeParseError),
    #[error("The relation instance type id is not valid: {0}")]
    RelationInstanceTypeIdError(#[from] RelationInstanceTypeIdError),
}

impl TryFrom<GraphQLFlowInstanceDefinition> for FlowInstance {
    type Error = GraphQLFlowInstanceDefinitionError;

    fn try_from(flow: GraphQLFlowInstanceDefinition) -> Result<Self, Self::Error> {
        let entity_ty = EntityTypeId::from_str(&flow._type)?;
        let entity_instances = flow.entity_instances()?;
        let relation_instances = flow.relation_instances()?;
        Ok(FlowInstance {
            id: flow.id,
            ty: entity_ty,
            name: flow.name.clone(),
            description: flow.description.clone(),
            entity_instances,
            relation_instances,
        })
    }
}
