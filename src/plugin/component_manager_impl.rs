use crate::model::{Component, PropertyType};
use crate::plugins::component_manager::ComponentCreationError;
use crate::plugins::ComponentManager;
use std::sync::Arc;

pub struct ComponentManagerImpl {
    component_manager: Arc<dyn crate::api::ComponentManager>,
}

impl ComponentManagerImpl {
    pub fn new(component_manager: Arc<dyn crate::api::ComponentManager>) -> Self {
        Self { component_manager }
    }
}
impl ComponentManager for ComponentManagerImpl {
    fn get_components(&self) -> Vec<Component> {
        self.component_manager.get_components()
    }

    fn has(&self, name: String) -> bool {
        self.component_manager.has(name)
    }

    fn get(&self, name: String) -> Option<Component> {
        self.component_manager.get(name)
    }

    fn create(&self, name: String, properties: Vec<PropertyType>) {
        self.component_manager.create(name, properties)
    }

    fn delete(&self, name: String) {
        self.component_manager.delete(name)
    }
}
