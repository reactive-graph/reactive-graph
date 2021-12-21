use crate::model::{EntityType, Extension, PropertyType};
use crate::plugins::entity_type_manager::EntityTypeCreationError;
use crate::plugins::EntityTypeManager;
use std::sync::Arc;

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

    fn has(&self, name: String) -> bool {
        self.entity_type_manager.has(name)
    }

    fn get(&self, name: String) -> Option<EntityType> {
        self.entity_type_manager.get(name)
    }

    fn create(&self, name: String, group: String, components: Vec<String>, behaviours: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>) {
        self.entity_type_manager.create(name, group, components, behaviours, properties, extensions)
    }

    fn delete(&self, name: String) {
        self.entity_type_manager.delete(name)
    }
}
