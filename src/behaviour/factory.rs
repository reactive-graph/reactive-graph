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

#[macro_export]
macro_rules! behaviour_factory {
    ($factory: ident, $behaviour: ty, $reactive_instance: ty) => {
        pub struct $factory {
            pub ty: BehaviourTypeId,
        }

        impl $factory {
            pub fn new(ty: BehaviourTypeId) -> Self {
                $factory { ty }
            }
        }

        impl BehaviourFactory<$reactive_instance> for $factory {
            fn create(
                &self,
                reactive_instance: Arc<$reactive_instance>,
            ) -> Result<Arc<dyn BehaviourFsm<$reactive_instance> + Send + Sync>, BehaviourCreationError> {
                match <$behaviour>::new(reactive_instance, self.ty.clone()) {
                    Ok(state) => {
                        let state = state as Arc<dyn BehaviourFsm<$reactive_instance> + Send + Sync>;
                        Ok(state)
                    }
                    Err(e) => Err(e),
                }
            }

            fn behaviour_ty(&self) -> &BehaviourTypeId {
                &self.ty
            }
        }
    };
}
