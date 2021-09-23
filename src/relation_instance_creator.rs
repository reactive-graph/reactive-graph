use inexor_rgf_core_model::{ReactiveRelationInstance, RelationInstance};

pub enum RelationInstanceCreationError {
    FAILED,
}

pub trait RelationInstanceCreator: Send + Sync {
    fn create_relation_instance(
        &self,
        relation_instance: RelationInstance,
    ) -> Result<ReactiveRelationInstance, RelationInstanceCreationError>;
}
