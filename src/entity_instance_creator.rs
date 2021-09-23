use std::sync::Arc;

use inexor_rgf_core_model::{EntityInstance, ReactiveEntityInstance};

#[derive(Debug)]
pub enum EntityInstanceCreationError {
    Failed,
}

pub trait EntityInstanceCreator: Send + Sync {
    fn create_entity_instance(
        &self,
        entity_instance: EntityInstance,
    ) -> Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError>;
}
