use std::sync::Arc;

use crate::model::Component;
use crate::model::PropertyType;
use crate::plugins::ComponentManager;

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

    fn has(&self, name: &str) -> bool {
        self.component_manager.has(name)
    }

    fn get(&self, name: &str) -> Option<Component> {
        self.component_manager.get(name)
    }

    fn find(&self, search: &str) -> Vec<Component> {
        self.component_manager.find(search)
    }

    fn count(&self) -> usize {
        self.component_manager.count()
    }

    fn create(&self, name: &str, properties: Vec<PropertyType>) {
        self.component_manager.create(name, properties)
    }

    fn delete(&self, name: &str) {
        self.component_manager.delete(name)
    }

    fn import(&self, path: &str) {
        self.component_manager.import(path)
    }

    fn export(&self, name: &str, path: &str) {
        self.component_manager.export(name, path)
    }
}
