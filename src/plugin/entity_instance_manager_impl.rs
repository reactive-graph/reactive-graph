use crate::api::ReactiveEntityInstanceManager;
use crate::model::{EntityInstance, ReactiveEntityInstance};
use crate::plugins::entity_instance_manager::EntityInstanceCreationError;
use crate::plugins::EntityInstanceManager;
use std::sync::Arc;
use uuid::Uuid;

pub struct EntityInstanceManagerImpl {
    reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>,
}

impl EntityInstanceManagerImpl {
    pub fn new(reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>) -> Self {
        Self {
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

    fn delete(&self, id: Uuid) {
        self.reactive_entity_instance_manager.delete(id);
    }
}
