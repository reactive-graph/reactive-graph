use std::sync::Arc;

use crate::model::EntityType;
use crate::model::Extension;
use crate::model::PropertyType;
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

    fn has(&self, name: &str) -> bool {
        self.entity_type_manager.has(name)
    }

    fn get(&self, name: &str) -> Option<EntityType> {
        self.entity_type_manager.get(name)
    }

    fn find(&self, search: &str) -> Vec<EntityType> {
        self.entity_type_manager.find(search)
    }

    fn count(&self) -> usize {
        self.entity_type_manager.count()
    }

    fn create(&self, name: String, namespace: String, components: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>) {
        self.entity_type_manager.create(name, namespace, components, properties, extensions)
    }

    fn delete(&self, name: &str) {
        self.entity_type_manager.delete(name)
    }

    fn import(&self, path: &str) {
        let _result = self.entity_type_manager.import(path);
    }

    fn export(&self, name: &str, path: &str) {
        self.entity_type_manager.export(name, path)
    }
}
