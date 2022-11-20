pub use container::*;
pub use error::*;
pub use factory::*;
pub use fsm::*;
pub use observer::*;
pub use state::*;
pub use storage::*;
pub use transition::*;
pub use validation::*;

pub mod container;
pub mod error;
pub mod factory;
pub mod fsm;
pub mod observer;
pub mod state;
pub mod storage;
pub mod transition;
pub mod validation;

#[macro_export]
macro_rules! behaviour {
    (
        /// The reactive instance type.
        $reactive_instance: ty,
        /// The property observer container type.
        $property_observer: ty,
        /// The ident of the behaviour.
        $behaviour: ident,
        /// The ident of the factory to create instances of the behaviour.
        $factory: ident,
        /// The ident of the finite state machine of the behaviour.
        $fsm: ident,
        /// The ident of the transitions of the finite state machine.
        $transitions: ident,
        /// The ident of the property validator of the behaviour.
        $validator: ty
        $(,
            /// Function name.
            $fn_name: ident,
            /// Function.
            $fn_ident: ident
        )*
    ) => {
        pub struct $behaviour {
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: Arc<$reactive_instance>, ty: BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<Arc<$behaviour>, BehaviourCreationError> {
                let property_observers = <$property_observer>::new(reactive_instance.clone());
                let transitions = <$transitions>::new(property_observers, ty.clone() $(, $fn_name)*);
                let validator = <$validator>::new(reactive_instance);
                let fsm = <$fsm>::new(ty, validator, transitions);
                let mut behaviour = $behaviour { fsm };
                behaviour
                    .fsm
                    .transition(BehaviourState::Connected)
                    .map_err(BehaviourCreationError::BehaviourTransitionError)?;
                Ok(Arc::new(behaviour))
            }
        }

        impl BehaviourFsm<$reactive_instance> for $behaviour {
            fn ty(&self) -> &BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> BehaviourState {
                self.fsm.state
            }

            fn set_state(&mut self, state: BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn BehaviourValidator<$reactive_instance> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn BehaviourTransitions<$reactive_instance> {
                &self.fsm.transitions
            }

            fn get_property_observers(&self) -> &PropertyObserverContainerImpl<$reactive_instance> {
                self.fsm.get_property_observers()
            }

            fn get_reactive_instance(&self) -> &Arc<$reactive_instance> {
                self.fsm.get_reactive_instance()
            }
        }

        impl Drop for $behaviour {
            fn drop(&mut self) {
                trace!("Drop behaviour {}", &self.fsm.ty);
            }
        }

        behaviour_factory!($factory, $behaviour, $reactive_instance $(, $fn_name, $fn_ident)*);

        behaviour_fsm!($fsm, $validator, $transitions, $reactive_instance);

        behaviour_transitions!($transitions, $reactive_instance $(, $fn_name, $fn_ident)*);
    };
}

#[macro_export]
macro_rules! behaviour_types {
    ($behaviour_types: ident, $namespace: expr $(, $behaviour_type_names:expr)*) => {
        lazy_static! {
            pub static ref $behaviour_types: Vec<BehaviourTypeId> = vec![
                $(
                BehaviourTypeId::new_from_type($namespace, $behaviour_type_names),
                )*
            ]
            .into_iter()
            .collect();
        }
    };
}
