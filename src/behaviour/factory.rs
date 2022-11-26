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
    (
        $factory: ident,
        $behaviour: ty,
        $reactive_instance: ty
        $(, $fn_name:ident, $fn_ident: ident)*
    ) => {
        pub struct $factory {
            pub ty: inexor_rgf_core_model::BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $factory {
            pub fn new(ty: inexor_rgf_core_model::BehaviourTypeId, $($fn_name: $fn_ident)*) -> Self {
                $factory {
                    ty,
                    $($fn_name,)*
                }
            }
        }

        impl BehaviourFactory<$reactive_instance> for $factory {
            fn create(
                &self,
                reactive_instance: std::sync::Arc<$reactive_instance>,
            ) -> Result<std::sync::Arc<dyn BehaviourFsm<$reactive_instance> + Send + Sync>, BehaviourCreationError> {
                // Prevent that the same behaviour can be applied twice / multiple times.
                if reactive_instance.behaves_as(&self.ty) {
                    return Err(BehaviourCreationError::BehaviourAlreadyApplied(self.ty.clone()));
                }
                match <$behaviour>::new(reactive_instance, self.ty.clone() $(, self.$fn_name)*) {
                    Ok(state) => {
                        let state = state as std::sync::Arc<dyn BehaviourFsm<$reactive_instance> + Send + Sync>;
                        Ok(state)
                    }
                    Err(e) => Err(e),
                }
            }

            fn behaviour_ty(&self) -> &inexor_rgf_core_model::BehaviourTypeId {
                &self.ty
            }
        }
    };
}
