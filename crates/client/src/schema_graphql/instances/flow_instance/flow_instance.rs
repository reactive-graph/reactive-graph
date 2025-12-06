use crate::schema_graphql::instances::entity_instance::EntityInstance;
use crate::schema_graphql::instances::entity_instance::EntityInstances;
use crate::schema_graphql::instances::relation_instance::RelationInstance;
use crate::schema_graphql::instances::relation_instance::RelationInstances;
use crate::schema_graphql::scalar::UUID;
use crate::schema_graphql::types::entity_type::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::InvalidFlowInstanceError;
use std::ops::Deref;
use std::str::FromStr;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(
    schema_path = "../../schema/graphql/reactive-graph-schema.graphql",
    schema_module = "crate::schema_graphql::schema"
)]
pub struct FlowInstance {
    #[cynic(rename = "type")]
    pub entity_type: EntityType,
    pub id: UUID,
    pub name: String,
    pub description: String,
    pub entities: Vec<EntityInstance>,
    pub relations: Vec<RelationInstance>,
}

impl FlowInstance {
    pub fn ty(&self) -> Result<EntityTypeId, InvalidFlowInstanceError> {
        EntityTypeId::from_str(&self.entity_type._type).map_err(InvalidFlowInstanceError::InvalidEntityType)
    }
}

impl TryFrom<FlowInstance> for reactive_graph_graph::FlowInstance {
    type Error = InvalidFlowInstanceError;

    fn try_from(flow_instance: FlowInstance) -> Result<Self, Self::Error> {
        Ok(reactive_graph_graph::FlowInstance {
            ty: EntityTypeId::from_str(&flow_instance.entity_type._type).map_err(InvalidFlowInstanceError::InvalidEntityType)?,
            id: flow_instance.id.into(),
            name: flow_instance.name.clone(),
            description: flow_instance.description.clone(),
            entity_instances: reactive_graph_graph::EntityInstances::try_from(EntityInstances(flow_instance.entities))
                .map_err(InvalidFlowInstanceError::InvalidEntityInstance)?,
            relation_instances: reactive_graph_graph::RelationInstances::try_from(RelationInstances(flow_instance.relations))
                .map_err(InvalidFlowInstanceError::InvalidRelationInstance)?,
        })
    }
}

pub struct FlowInstances(pub Vec<FlowInstance>);

impl Deref for FlowInstances {
    type Target = Vec<FlowInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<FlowInstances> for reactive_graph_graph::FlowInstances {
    type Error = InvalidFlowInstanceError;

    fn try_from(flows: FlowInstances) -> Result<Self, Self::Error> {
        let flow_instances = reactive_graph_graph::FlowInstances::new();
        for flow_instance in flows.0 {
            flow_instances.push(reactive_graph_graph::FlowInstance::try_from(flow_instance)?);
        }
        Ok(flow_instances)
    }
}
