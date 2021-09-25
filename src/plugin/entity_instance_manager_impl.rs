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
    pub fn new(
        entity_type_manager: Arc<dyn EntityTypeManager>,
        reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>,
    ) -> Self {
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

    fn create(
        &self,
        entity_instance: EntityInstance,
    ) -> Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError> {
        let entity_type = self
            .entity_type_manager
            .get(entity_instance.type_name.clone());
        match entity_type {
            Some(entity_type) => {
                let mut entity_instance = entity_instance.clone();
                for property in entity_type.properties.iter() {
                    if !entity_instance.properties.contains_key(&property.name) {
                        entity_instance
                            .properties
                            .insert(property.name.clone(), property.data_type.default_value());
                    }
                }
                let reactive_entity_instance = self
                    .reactive_entity_instance_manager
                    .create_reactive_instance(entity_instance);
                match reactive_entity_instance {
                    Ok(reactive_entity_instance) => Ok(reactive_entity_instance),
                    Err(_) => {
                        return Err(EntityInstanceCreationError::Failed);
                    }
                }
            }
            None => {
                // error!("Unknown entity type ");
                return Err(EntityInstanceCreationError::Failed);
            }
        }
    }

    fn delete(&self, id: Uuid) {
        self.reactive_entity_instance_manager.delete(id);
    }
}
