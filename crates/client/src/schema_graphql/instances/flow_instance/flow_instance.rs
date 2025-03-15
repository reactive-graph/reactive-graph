use crate::schema_graphql::instances::entity_instance::EntityInstance;
use crate::schema_graphql::instances::entity_instance::EntityInstances;
use crate::schema_graphql::instances::relation_instance::RelationInstance;
use crate::schema_graphql::instances::relation_instance::RelationInstances;
use crate::schema_graphql::scalar::UUID;
use crate::schema_graphql::types::entity_type::EntityType;
use reactive_graph_graph::EntityTypeId;
use std::ops::Deref;

#[derive(cynic::QueryFragment, Clone, Debug)]
#[cynic(schema_path = "schema_graphql.graphql", schema_module = "crate::schema_graphql::schema")]
pub struct FlowInstance {
    pub id: UUID,
    #[cynic(rename = "type")]
    pub ty: EntityType,
    pub name: String,
    pub description: String,
    pub entities: Vec<EntityInstance>,
    pub relations: Vec<RelationInstance>,
}

impl FlowInstance {
    pub fn ty(&self) -> EntityTypeId {
        EntityTypeId::new_from_type(self.ty.namespace.clone(), self.ty.name.clone())
    }
}

impl From<FlowInstance> for reactive_graph_graph::FlowInstance {
    fn from(flow_instance: FlowInstance) -> Self {
        let ty = flow_instance.ty();
        let id = flow_instance.id.into();
        let entity_instances = EntityInstances(flow_instance.entities).into();
        let relation_instances = RelationInstances(flow_instance.relations).into();
        reactive_graph_graph::FlowInstance {
            id,
            ty,
            name: flow_instance.name.clone(),
            description: flow_instance.description.clone(),
            entity_instances,
            relation_instances,
        }
    }
}

pub struct FlowInstances(pub Vec<FlowInstance>);

impl Deref for FlowInstances {
    type Target = Vec<FlowInstance>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<FlowInstances> for Vec<reactive_graph_graph::FlowInstance> {
    fn from(flows: FlowInstances) -> Self {
        flows.0.into_iter().map(From::from).collect()
    }
}

// impl From<FlowInstances> for reactive_graph_graph::FlowInstances {
//     fn from(flows: FlowInstances) -> Self {
//         flows.0.into_iter().map(From::from).collect()
//     }
// }
