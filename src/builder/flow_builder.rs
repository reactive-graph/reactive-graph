use std::sync::Arc;

use uuid::Uuid;

use crate::api::{FlowCreationError, FlowManager};
use crate::model::{EntityInstance, Flow, RelationInstance};

#[allow(dead_code)]
pub struct FlowBuilder {
    id: Uuid,
    name: String,
    description: String,
    wrapper: EntityInstance,
    entity_instances: Vec<EntityInstance>,
    relation_instances: Vec<RelationInstance>,
}

#[allow(dead_code)]
impl FlowBuilder {
    pub fn new(wrapper_entity_instance: EntityInstance) -> FlowBuilder {
        let mut entity_instances = Vec::new();
        entity_instances.push(wrapper_entity_instance.clone());
        FlowBuilder {
            id: wrapper_entity_instance.id,
            name: String::from(""),
            description: String::from(""),
            wrapper: wrapper_entity_instance.clone(),
            entity_instances,
            relation_instances: Vec::new(),
        }
    }

    pub fn name<'a, S: Into<String>>(&'a mut self, name: S) -> &'a mut FlowBuilder {
        self.name = name.into();
        self
    }

    pub fn description<'a, S: Into<String>>(&'a mut self, description: S) -> &'a mut FlowBuilder {
        self.description = description.into();
        self
    }

    pub fn entity<'a>(&'a mut self, entity_instance: EntityInstance) -> &'a mut FlowBuilder {
        self.entity_instances.push(entity_instance);
        self
    }

    pub fn relation<'a>(&'a mut self, relation_instance: RelationInstance) -> &'a mut FlowBuilder {
        self.relation_instances.push(relation_instance);
        self
    }

    pub fn create<'a>(
        &'a mut self,
        flow_manager: Arc<dyn FlowManager>,
    ) -> Result<Flow, FlowCreationError> {
        flow_manager.create(self.get())
    }

    pub fn get(&mut self) -> Flow {
        let mut flow = Flow::from(self.wrapper.clone());
        flow.name = self.name.clone();
        flow.description = self.description.clone();
        flow.entity_instances = self.entity_instances.clone();
        flow.relation_instances = self.relation_instances.clone();
        flow
    }
}
