use std::sync::Arc;

use log::trace;

use crate::model::BehaviourTypeId;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveInstance;
use crate::BehaviourTransitionError;
use crate::BehaviourTransitions;
use crate::BehaviourValidator;
use crate::PropertyObserverContainerImpl;

/// The state of a behaviour.
#[derive(Debug, Clone, Copy)]
pub enum BehaviourState {
    Created,
    Valid,
    Ready,
    Connected,
}

pub trait BehaviourFsm<T: ReactiveInstance>: Send + Sync {
    /// Returns the current state of the behaviour.
    fn ty(&self) -> &BehaviourTypeId;

    /// Returns the current state of the behaviour.
    fn get_state(&self) -> BehaviourState;

    /// Returns the current state of the behaviour.
    fn set_state(&mut self, state: BehaviourState);

    /// Returns the validator.
    fn get_validator(&self) -> &dyn BehaviourValidator<T>;

    /// Returns the validator.
    fn get_transitions(&self) -> &dyn BehaviourTransitions<T>;

    fn get_property_observers(&self) -> &PropertyObserverContainerImpl<T>;

    fn get_reactive_instance(&self) -> &Arc<T>;

    fn transition(&mut self, target_state: BehaviourState) -> Result<(), BehaviourTransitionError> {
        trace!("transition {:?} -> {:?}", self.get_state(), target_state);
        match self.get_state() {
            BehaviourState::Created => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => self
                    .get_validator()
                    .validate()
                    .map(|_| self.set_state(target_state))
                    .map_err(|e| BehaviourTransitionError::BehaviourInvalid(e)),
                BehaviourState::Ready => self.transition(BehaviourState::Valid).and_then(|_| {
                    self.get_transitions()
                        .init()
                        .map(|_| self.set_state(target_state))
                        .map_err(|e| BehaviourTransitionError::BehaviourInitializationFailed(e))
                }),
                BehaviourState::Connected => self.transition(BehaviourState::Ready).and_then(|_| {
                    self.get_transitions()
                        .connect()
                        .map(|_| self.get_property_observers().add_behaviour(self.ty().clone()))
                        .map(|_| self.set_state(target_state))
                        .map_err(|e| BehaviourTransitionError::BehaviourConnectFailed(e))
                }),
            },
            BehaviourState::Valid => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => self
                    .get_transitions()
                    .init()
                    .map(|_| self.set_state(target_state))
                    .map_err(|e| BehaviourTransitionError::BehaviourInitializationFailed(e)),
                BehaviourState::Connected => self.transition(BehaviourState::Ready).and_then(|_| {
                    self.get_transitions()
                        .connect()
                        .map(|_| self.get_property_observers().add_behaviour(self.ty().clone()))
                        .map(|_| self.set_state(target_state))
                        .map_err(|e| BehaviourTransitionError::BehaviourConnectFailed(e))
                }),
            },
            BehaviourState::Ready => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Connected => self
                    .get_transitions()
                    .connect()
                    .map(|_| self.get_property_observers().add_behaviour(self.ty().clone()))
                    .map(|_| self.set_state(target_state))
                    .map_err(|e| BehaviourTransitionError::BehaviourConnectFailed(e)),
            },
            BehaviourState::Connected => match target_state {
                BehaviourState::Created => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Valid => Err(BehaviourTransitionError::InvalidTransition),
                BehaviourState::Ready => self
                    .get_transitions()
                    .disconnect()
                    .map(|_| self.get_property_observers().remove_behaviour(&self.ty()))
                    .map(|_| self.set_state(target_state))
                    .map_err(|e| BehaviourTransitionError::BehaviourDisconnectFailed(e)),
                BehaviourState::Connected => Err(BehaviourTransitionError::InvalidTransition),
            },
        }
    }
}
