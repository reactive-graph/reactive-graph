use std::sync::Arc;

use crate::model::Component;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::plugins::ComponentCreationError;
use crate::plugins::ComponentImportError;
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

    fn get_components_by_namespace(&self, namespace: &str) -> Vec<Component> {
        self.component_manager.get_components_by_namespace(namespace)
    }

    fn has(&self, name: &str) -> bool {
        self.component_manager.has(name)
    }

    fn has_fully_qualified(&self, namespace: &str, name: &str) -> bool {
        self.component_manager.has_fully_qualified(namespace, name)
    }

    fn get(&self, name: &str) -> Option<Component> {
        self.component_manager.get(name)
    }

    fn get_fully_qualified(&self, namespace: &str, name: &str) -> Option<Component> {
        self.component_manager.get_fully_qualified(namespace, name)
    }

    fn find(&self, search: &str) -> Vec<Component> {
        self.component_manager.find(search)
    }

    fn count(&self) -> usize {
        self.component_manager.count()
    }

    fn create(
        &self,
        namespace: &str,
        name: &str,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<Component, ComponentCreationError> {
        self.component_manager
            .create(namespace, name, description, properties, extensions)
            .map_err(|_| ComponentCreationError::Failed)
    }

    fn replace(&self, name: &str, component: Component) {
        self.component_manager.replace(name, component)
    }

    fn add_property(&self, name: &str, property: PropertyType) {
        let _ = self.component_manager.add_property(name, property);
    }

    fn remove_property(&self, name: &str, property_name: &str) {
        self.component_manager.remove_property(name, property_name)
    }

    fn add_extension(&self, name: &str, extension: Extension) {
        let _ = self.component_manager.add_extension(name, extension);
    }

    fn remove_extension(&self, name: &str, extension_name: &str) {
        self.component_manager.remove_extension(name, extension_name)
    }

    fn delete(&self, name: &str) {
        self.component_manager.delete(name)
    }

    fn import(&self, path: &str) -> Result<Component, ComponentImportError> {
        self.component_manager.import(path).map_err(|_| ComponentImportError::Failed)
    }

    fn export(&self, name: &str, path: &str) {
        self.component_manager.export(name, path)
    }
}
