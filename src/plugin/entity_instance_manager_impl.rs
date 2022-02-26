use crate::api::{EntityTypeManager, ReactiveEntityInstanceManager};
use crate::model::{EntityInstance, ReactiveEntityInstance};
use crate::plugins::entity_instance_manager::EntityInstanceCreationError;
use crate::plugins::EntityInstanceManager;
use std::sync::Arc;
use uuid::Uuid;

pub struct EntityInstanceManagerImpl {
    entity_type_manager: Arc<dyn EntityTypeManager>,
    reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>,
}

impl EntityInstanceManagerImpl {
    pub fn new(entity_type_manager: Arc<dyn EntityTypeManager>, reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>) -> Self {
        Self {
            entity_type_manager,
            reactive_entity_instance_manager,
        }
    }
}
impl EntityInstanceManager for EntityInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_entity_instance_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instance_manager.get(id)
    }

    fn get_by_label(&self, label: String) -> Option<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instance_manager.get_by_label(label)
    }

    fn get_all(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instance_manager.get_entity_instances()
    }

    fn get_ids(&self) -> Vec<Uuid> {
        self.reactive_entity_instance_manager.get_ids()
    }

    fn create(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError> {
        let entity_type = self.entity_type_manager.get(entity_instance.type_name.clone());
        match entity_type {
            Some(entity_type) => {
                let mut entity_instance = entity_instance;
                for property in entity_type.properties.iter() {
                    if !entity_instance.properties.contains_key(&property.name) {
                        entity_instance.properties.insert(property.name.clone(), property.data_type.default_value());
                    }
                }
                let reactive_entity_instance =
                    self.reactive_entity_instance_manager
                        .create_with_id(entity_instance.type_name, entity_instance.id, entity_instance.properties);
                match reactive_entity_instance {
                    Ok(reactive_entity_instance) => Ok(reactive_entity_instance),
                    Err(_) => Err(EntityInstanceCreationError::Failed),
                }
            }
            None => Err(EntityInstanceCreationError::Failed),
        }
    }

    fn add_component(&self, id: Uuid, component: String) {
        self.reactive_entity_instance_manager.add_component(id, component);
    }

    fn remove_component(&self, id: Uuid, component: String) {
        self.reactive_entity_instance_manager.remove_component(id, component);
    }

    fn delete(&self, id: Uuid) {
        self.reactive_entity_instance_manager.delete(id);
    }
}
