use std::convert::TryFrom;
use std::sync::Arc;

// use async_graphql::SimpleObject;
use async_graphql::scalar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{EntityInstance, ReactiveFlow, RelationInstance};

#[derive(Debug)]
pub struct FlowCreationError;

/// A flow is a container for entity instances and relation instances.
///
/// A flow is strictly associated with a wrapper entity instance. The properties
/// of the wrapper entity instance are the properties of the flow.
///
/// Additionally, flows can be nested -  from the perspective of the outer flow
/// the inner flow acts like an entity instance. The wrapper entity instance of
/// the inner flow is the interface which can be accessed by the outer flow.
///
/// Entity instances and relation instances can be shared with multiple flows.
///
/// It's even possible to connect entity instances from different flows with relation
/// instances.
///
// TODO: #[derive(Serialize, Deserialize, Clone, Debug, SimpleObject)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flow {
    /// The id of the flow corresponds to the id of the wrapper entity instance
    ///
    /// This means the vector of entity instances must contain an instance with
    /// the id of the flow.
    pub id: Uuid,

    /// The entity type of the flow.
    #[serde(alias = "type")]
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
    #[serde(alias = "entities")]
    pub entity_instances: Vec<EntityInstance>,

    /// The relation instances which are contained in this flow.
    ///
    /// By default, no relation instances are contained in this flow.
    #[serde(default = "Vec::new", alias = "relations")]
    pub relation_instances: Vec<RelationInstance>,
}
scalar!(Flow);
// TODO: ---scalar!(Flow);---

impl Flow {
    /// Constructs a new flow from the wrapper entity instance.
    pub fn from_instance_with_name(wrapper_entity_instance: EntityInstance, name: String) -> Flow {
        let id = wrapper_entity_instance.id;
        let type_name = wrapper_entity_instance.type_name.clone();
        let mut entity_instances: Vec<EntityInstance> = Vec::new();
        entity_instances.push(wrapper_entity_instance);
        Flow {
            id,
            type_name,
            name,
            description: String::new(),
            entity_instances,
            relation_instances: Vec::new(),
        }
    }
}

impl From<EntityInstance> for Flow {
    fn from(wrapper_entity_instance: EntityInstance) -> Flow {
        let id = wrapper_entity_instance.id;
        let type_name = wrapper_entity_instance.type_name.clone();
        let mut entity_instances: Vec<EntityInstance> = Vec::new();
        entity_instances.push(wrapper_entity_instance);
        Flow {
            id,
            type_name,
            name: String::new(),
            description: String::new(),
            entity_instances,
            relation_instances: Vec::new(),
        }
    }
}

impl TryFrom<ReactiveFlow> for Flow {
    type Error = FlowCreationError;

    fn try_from(reactive_flow: ReactiveFlow) -> Result<Self, FlowCreationError> {
        let wrapper = reactive_flow.get_entity(reactive_flow.id);
        if wrapper.is_none() {
            return Err(FlowCreationError.into());
        }
        let wrapper = wrapper.unwrap();
        let entity_instance: EntityInstance = wrapper.clone().into();
        let mut flow = Flow::from(entity_instance);
        flow.description = wrapper.description.clone();
        reactive_flow
            .entity_instances
            .read()
            .unwrap()
            .iter()
            .for_each(|(_, entity)| {
                if entity.id != reactive_flow.id {
                    flow.entity_instances.push(entity.clone().into());
                }
            });
        reactive_flow
            .relation_instances
            .read()
            .unwrap()
            .iter()
            .for_each(|(_, relation_instance)| {
                flow.relation_instances.push(relation_instance.clone().into());
            });
        Ok(flow)
    }
}

impl TryFrom<Arc<ReactiveFlow>> for Flow {
    type Error = FlowCreationError;

    fn try_from(reactive_flow: Arc<ReactiveFlow>) -> Result<Self, FlowCreationError> {
        let wrapper = reactive_flow.get_entity(reactive_flow.id);
        if wrapper.is_none() {
            return Err(FlowCreationError.into());
        }
        let wrapper = wrapper.unwrap();
        let entity_instance: EntityInstance = wrapper.clone().into();
        let mut flow = Flow::from(entity_instance);
        flow.description = wrapper.description.clone();
        reactive_flow
            .entity_instances
            .read()
            .unwrap()
            .iter()
            .for_each(|(_, entity)| {
                if entity.id != reactive_flow.id {
                    flow.entity_instances.push(entity.clone().into());
                }
            });
        reactive_flow
            .relation_instances
            .read()
            .unwrap()
            .iter()
            .for_each(|(_, relation_instance)| {
                flow.relation_instances.push(relation_instance.clone().into());
            });
        Ok(flow)
    }
}
