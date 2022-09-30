use crate::model::FlowType;
use crate::plugins::FlowTypeManager;
use inexor_rgf_core_model::{EntityInstance, Extension, PropertyType, RelationInstance};
use std::sync::Arc;

pub struct FlowTypeManagerImpl {
    flow_type_manager: Arc<dyn crate::api::FlowTypeManager>,
}

impl FlowTypeManagerImpl {
    pub fn new(flow_type_manager: Arc<dyn crate::api::FlowTypeManager>) -> Self {
        Self { flow_type_manager }
    }
}
impl FlowTypeManager for FlowTypeManagerImpl {
    fn get_flow_types(&self) -> Vec<FlowType> {
        self.flow_type_manager.get_flow_types()
    }

    fn has(&self, name: &str) -> bool {
        self.flow_type_manager.has(name)
    }

    fn get(&self, name: &str) -> Option<FlowType> {
        self.flow_type_manager.get(name)
    }

    fn find(&self, search: &str) -> Vec<FlowType> {
        self.flow_type_manager.find(search)
    }

    fn count(&self) -> usize {
        self.flow_type_manager.count()
    }

    fn create(
        &self,
        namespace: &str,
        name: &str,
        type_name: &str,
        description: &str,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.flow_type_manager
            .create(namespace, name, type_name, description, entity_instances, relation_instances, variables, extensions);
    }

    fn delete(&self, name: &str) {
        self.flow_type_manager.delete(name)
    }

    fn import(&self, path: &str) {
        let _result = self.flow_type_manager.import(path);
    }

    fn export(&self, name: &str, path: &str) {
        self.flow_type_manager.export(name, path)
    }
}
