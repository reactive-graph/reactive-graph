use crate::api::ReactiveRelationInstanceManager;
use indradb::EdgeKey;
use inexor_rgf_core_model::{ReactiveRelationInstance, RelationInstance};
use inexor_rgf_core_plugins::relation_instance_manager::{
    RelationInstanceCreationError, RelationInstanceManager,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct RelationInstanceManagerImpl {
    reactive_relation_instance_manager: Arc<dyn ReactiveRelationInstanceManager>,
}

impl RelationInstanceManagerImpl {
    pub fn new(
        reactive_relation_instance_manager: Arc<dyn ReactiveRelationInstanceManager>,
    ) -> Self {
        Self {
            reactive_relation_instance_manager,
        }
    }
}
impl RelationInstanceManager for RelationInstanceManagerImpl {
    fn has(&self, edge_key: EdgeKey) -> bool {
        self.reactive_relation_instance_manager.has(edge_key)
    }

    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager.get(edge_key)
    }

    fn get_by_outbound_entity(
        &self,
        outbound_entity_id: Uuid,
    ) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager
            .get_by_outbound_entity(outbound_entity_id)
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instance_manager
            .get_by_inbound_entity(inbound_entity_id)
    }

    fn create(
        &self,
        relation_instance: RelationInstance,
    ) -> Result<Arc<ReactiveRelationInstance>, RelationInstanceCreationError> {
        let reactive_relation_instance = self
            .reactive_relation_instance_manager
            .create_reactive_instance(relation_instance);
        match reactive_relation_instance {
            Ok(reactive_relation_instance) => Ok(reactive_relation_instance),
            Err(_) => {
                return Err(RelationInstanceCreationError::Failed);
            }
        }
    }

    fn delete(&self, edge_key: EdgeKey) -> bool {
        self.reactive_relation_instance_manager.delete(edge_key)
    }
}
