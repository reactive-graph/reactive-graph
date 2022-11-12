use std::sync::Arc;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveInstance;
use crate::BehaviourCreationError;
use crate::BehaviourFsm;

pub trait BehaviourFactory<T: ReactiveInstance> {
    /// Creates a new behaviour in the given reactive instance and returns the created state machine.
    fn create(&self, reactive_instance: Arc<T>) -> Result<Arc<dyn BehaviourFsm<T> + Send + Sync>, BehaviourCreationError>;

    /// Returns the behaviour type of the behaviour factory.
    fn behaviour_ty(&self) -> &BehaviourTypeId;
}
