use std::sync::Arc;

use crate::model::EntityType;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::plugins::EntityTypeCreationError;
use crate::plugins::EntityTypeImportError;
use crate::plugins::EntityTypeManager;

pub struct EntityTypeManagerImpl {
    entity_type_manager: Arc<dyn crate::api::EntityTypeManager>,
}

impl EntityTypeManagerImpl {
    pub fn new(entity_type_manager: Arc<dyn crate::api::EntityTypeManager>) -> Self {
        Self { entity_type_manager }
    }
}
impl EntityTypeManager for EntityTypeManagerImpl {
    fn get_entity_types(&self) -> Vec<EntityType> {
        self.entity_type_manager.get_entity_types()
    }

    fn get_entity_types_by_namespace(&self, namespace: &str) -> Vec<EntityType> {
        self.entity_type_manager.get_entity_types_by_namespace(namespace)
    }

    fn has(&self, name: &str) -> bool {
        self.entity_type_manager.has(name)
    }

    fn has_fully_qualified(&self, namespace: &str, name: &str) -> bool {
        self.entity_type_manager.has_fully_qualified(namespace, name)
    }

    fn get(&self, name: &str) -> Option<EntityType> {
        self.entity_type_manager.get(name)
    }

    fn get_fully_qualified(&self, namespace: &str, name: &str) -> Option<EntityType> {
        self.entity_type_manager.get_fully_qualified(namespace, name)
    }

    fn find(&self, search: &str) -> Vec<EntityType> {
        self.entity_type_manager.find(search)
    }

    fn count(&self) -> usize {
        self.entity_type_manager.count()
    }

    fn create(
        &self,
        namespace: &str,
        name: &str,
        description: &str,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<EntityType, EntityTypeCreationError> {
        self.entity_type_manager
            .create(namespace, name, description, components, properties, extensions)
            .map_err(|_| EntityTypeCreationError::Failed)
    }

    fn add_component(&self, name: &str, component_name: &str) {
        let _ = self.entity_type_manager.add_component(name, component_name);
    }

    fn remove_component(&self, name: &str, component_name: &str) {
        self.entity_type_manager.remove_component(name, component_name);
    }

    fn add_property(&self, name: &str, property: PropertyType) {
        let _ = self.entity_type_manager.add_property(name, property);
    }

    fn remove_property(&self, name: &str, property_name: &str) {
        self.entity_type_manager.remove_property(name, property_name)
    }

    fn add_extension(&self, name: &str, extension: Extension) {
        let _ = self.entity_type_manager.add_extension(name, extension);
    }

    fn remove_extension(&self, name: &str, extension_name: &str) {
        self.entity_type_manager.remove_extension(name, extension_name)
    }

    fn delete(&self, name: &str) {
        self.entity_type_manager.delete(name)
    }

    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        self.entity_type_manager.import(path).map_err(|_| EntityTypeImportError::Failed)
    }

    fn export(&self, name: &str, path: &str) {
        self.entity_type_manager.export(name, path)
    }
}
