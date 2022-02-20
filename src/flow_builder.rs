use uuid::Uuid;

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
        let entity_instances = vec![wrapper_entity_instance.clone()];
        FlowBuilder {
            id: wrapper_entity_instance.id,
            name: String::from(""),
            description: String::from(""),
            wrapper: wrapper_entity_instance,
            entity_instances,
            relation_instances: Vec::new(),
        }
    }

    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut FlowBuilder {
        self.name = name.into();
        self
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut FlowBuilder {
        self.description = description.into();
        self
    }

    pub fn entity(&mut self, entity_instance: EntityInstance) -> &mut FlowBuilder {
        self.entity_instances.push(entity_instance);
        self
    }

    pub fn relation(&mut self, relation_instance: RelationInstance) -> &mut FlowBuilder {
        self.relation_instances.push(relation_instance);
        self
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
