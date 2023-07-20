use std::sync::Arc;

use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
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
    fn get_all(&self) -> Vec<Component> {
        self.component_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<Component> {
        self.component_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &ComponentTypeId) -> bool {
        self.component_manager.has(ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.component_manager.has_by_type(namespace, type_name)
    }

    fn get(&self, ty: &ComponentTypeId) -> Option<Component> {
        self.component_manager.get(ty)
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<Component> {
        self.component_manager.get_by_type(namespace, type_name)
    }

    fn find(&self, search: &str) -> Vec<Component> {
        self.component_manager.find(search)
    }

    fn count(&self) -> usize {
        self.component_manager.count()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.component_manager.count_by_namespace(namespace)
    }

    fn create(
        &self,
        ty: &ComponentTypeId,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<Component, ComponentCreationError> {
        self.component_manager
            .create(ty, description, properties, extensions)
            .map_err(|_| ComponentCreationError::Failed)
    }

    fn replace(&self, ty: &ComponentTypeId, component: Component) {
        self.component_manager.replace(ty, component)
    }

    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) {
        let _ = self.component_manager.add_property(ty, property);
    }

    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) {
        self.component_manager.remove_property(ty, property_name)
    }

    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) {
        let _ = self.component_manager.add_extension(ty, extension);
    }

    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId) {
        self.component_manager.remove_extension(component_ty, extension_ty)
    }

    fn delete(&self, ty: &ComponentTypeId) -> bool {
        self.component_manager.delete(ty)
    }

    fn import(&self, path: &str) -> Result<Component, ComponentImportError> {
        self.component_manager.import(path).map_err(|_| ComponentImportError::Failed)
    }

    fn export(&self, ty: &ComponentTypeId, path: &str) {
        self.component_manager.export(ty, path)
    }
}
