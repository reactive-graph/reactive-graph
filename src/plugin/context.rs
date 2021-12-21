use crate::plugins::{ComponentManager, EntityInstanceManager, EntityTypeManager, FlowManager, PluginContext, RelationInstanceManager, RelationTypeManager};
use std::sync::Arc;

pub struct PluginContextImpl {
    component_manager: Arc<dyn ComponentManager>,
    entity_type_manager: Arc<dyn EntityTypeManager>,
    relation_type_manager: Arc<dyn RelationTypeManager>,
    entity_instance_manager: Arc<dyn EntityInstanceManager>,
    relation_instance_manager: Arc<dyn RelationInstanceManager>,
    flow_manager: Arc<dyn FlowManager>,
}

impl PluginContextImpl {
    pub fn new(
        component_manager: Arc<dyn ComponentManager>,
        entity_type_manager: Arc<dyn EntityTypeManager>,
        relation_type_manager: Arc<dyn RelationTypeManager>,
        entity_instance_manager: Arc<dyn EntityInstanceManager>,
        relation_instance_manager: Arc<dyn RelationInstanceManager>,
        flow_manager: Arc<dyn FlowManager>,
    ) -> Self {
        PluginContextImpl {
            component_manager,
            entity_type_manager,
            relation_type_manager,
            entity_instance_manager,
            relation_instance_manager,
            flow_manager,
        }
    }
}

impl PluginContext for PluginContextImpl {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        self.component_manager.clone()
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager> {
        self.entity_type_manager.clone()
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager> {
        self.relation_type_manager.clone()
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager> {
        self.entity_instance_manager.clone()
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager> {
        self.relation_instance_manager.clone()
    }

    fn get_flow_manager(&self) -> Arc<dyn FlowManager> {
        self.flow_manager.clone()
    }
}
