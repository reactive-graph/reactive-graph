use inexor_rgf_reactive_api::prelude::*;

use crate::BehaviourState;
use crate::BehaviourTransitionError;
use crate::BehaviourTransitions;
use crate::BehaviourTypeId;
use crate::BehaviourTypesContainer;
use crate::BehaviourValidator;

pub trait BehaviourFsm<ID: Clone, T: ReactiveInstance<ID> + BehaviourTypesContainer>: ReactiveInstanceContainer<ID, T> + Send + Sync {
    /// Returns the current state of the behaviour.
    fn ty(&self) -> &BehaviourTypeId;

    /// Returns the current state of the behaviour.
    fn get_state(&self) -> BehaviourState;

    /// Returns the current state of the behaviour.
    fn set_state(&self, state: BehaviourState);

    /// Returns the validator.
    fn get_validator(&self) -> &dyn BehaviourValidator<ID, T>;

    /// Returns the validator.
    fn get_transitions(&self) -> &dyn BehaviourTransitions<ID, T>;

    /// Executes a behaviour transition.
    fn transition(&self, target_state: BehaviourState) -> Result<(), BehaviourTransitionError> {
        match self.get_state() {
            BehaviourState::Created => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => self
                    .get_validator()
                    .validate()
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourInvalid),
                BehaviourState::Ready => self.transition(BehaviourState::Valid).and_then(|_| {
                    self.get_transitions()
                        .init()
                        .map(|_| self.set_state(target_state))
                        .map_err(BehaviourTransitionError::BehaviourInitializationFailed)
                }),
                BehaviourState::Connected => self.transition(BehaviourState::Ready).and_then(|_| {
                    self.get_transitions()
                        .connect()
                        .map(|_| self.get_reactive_instance().add_behaviour(self.ty().clone()))
                        .map(|_| self.set_state(target_state))
                        .map_err(BehaviourTransitionError::BehaviourConnectFailed)
                }),
            },
            BehaviourState::Valid => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => self
                    .get_transitions()
                    .init()
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourInitializationFailed),
                BehaviourState::Connected => self.transition(BehaviourState::Ready).and_then(|_| {
                    self.get_transitions()
                        .connect()
                        .map(|_| self.get_reactive_instance().add_behaviour(self.ty().clone()))
                        .map(|_| self.set_state(target_state))
                        .map_err(BehaviourTransitionError::BehaviourConnectFailed)
                }),
            },
            BehaviourState::Ready => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Connected => self
                    .get_transitions()
                    .connect()
                    .map(|_| self.get_reactive_instance().add_behaviour(self.ty().clone()))
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourConnectFailed),
            },
            BehaviourState::Connected => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => self
                    .get_transitions()
                    .disconnect()
                    .map(|_| self.get_reactive_instance().remove_behaviour(self.ty()))
                    .map(|_| self.set_state(target_state))
                    .map_err(BehaviourTransitionError::BehaviourDisconnectFailed),
                BehaviourState::Connected => Err(BehaviourTransitionError::InvalidTransition),
            },
        }
    }
}

#[macro_export]
macro_rules! behaviour_fsm {
    ($fsm: ident, $validator: ty, $transitions: ty, $id: ty, $reactive_instance: ty) => {
        use inexor_rgf_graph::PropertyInstanceGetter as BehaviourFsmPropertyInstanceGetter;

        pub struct $fsm {
            pub reactive_instance: $reactive_instance,
            pub ty: $crate::BehaviourTypeId,
            pub state: std::sync::RwLock<$crate::BehaviourState>,
            pub validator: $validator,
            pub transitions: $transitions,
        }

        impl $fsm {
            pub fn new(reactive_instance: $reactive_instance, ty: $crate::BehaviourTypeId, validator: $validator, transitions: $transitions) -> Self {
                $fsm {
                    reactive_instance,
                    ty,
                    state: std::sync::RwLock::new($crate::BehaviourState::Created),
                    validator,
                    transitions,
                }
            }
        }

        impl $crate::BehaviourFsm<$id, $reactive_instance> for $fsm {
            fn ty(&self) -> &$crate::BehaviourTypeId {
                &self.ty
            }

            fn get_state(&self) -> $crate::BehaviourState {
                let reader = self.state.read().unwrap();
                (*reader).clone()
            }

            fn set_state(&self, state: $crate::BehaviourState) {
                let mut writer = self.state.write().unwrap();
                *writer = state;
            }

            fn get_validator(&self) -> &dyn $crate::BehaviourValidator<$id, $reactive_instance> {
                &self.validator
            }

            fn get_transitions(&self) -> &dyn $crate::BehaviourTransitions<$id, $reactive_instance> {
                &self.transitions
            }
        }

        impl inexor_rgf_reactive_api::ReactiveInstanceContainer<$id, $reactive_instance> for $fsm {
            fn get_reactive_instance(&self) -> &$reactive_instance {
                &self.reactive_instance
            }

            fn get(&self, property_name: &str) -> Option<serde_json::Value> {
                self.reactive_instance.get(property_name)
            }

            fn set(&self, property_name: &str, value: serde_json::Value) {
                self.reactive_instance.set(property_name, value);
            }
        }
    };
}
