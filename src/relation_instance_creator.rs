use inexor_rgf_core_model::{ReactiveRelationInstance, RelationInstance};
use std::sync::Arc;

#[derive(Debug)]
pub enum RelationInstanceCreationError {
    Failed,
}

pub trait RelationInstanceCreator: Send + Sync {
    fn create_relation_instance(
        &self,
        relation_instance: RelationInstance,
    ) -> Result<Arc<ReactiveRelationInstance>, RelationInstanceCreationError>;
}
