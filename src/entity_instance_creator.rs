use inexor_rgf_core_model::{
    EntityInstance, ReactiveEntityInstance, ReactiveRelationInstance, RelationInstance,
};
use std::sync::Arc;

pub enum EntityInstanceCreationError {
    FAILED,
}

pub trait EntityInstanceCreator: Send + Sync {
    fn create_entity_instance(
        &self,
        entity_instance: EntityInstance,
    ) -> Result<ReactiveEntityInstance, EntityInstanceCreationError>;
}
