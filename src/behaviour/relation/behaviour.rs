#[macro_export]
macro_rules! relation_behaviour {
    (
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
            pub reactive_instance: Arc<ReactiveRelationInstance>,
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: Arc<ReactiveRelationInstance>, ty: BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<Arc<$behaviour>, BehaviourCreationError> {
                let transitions = <$transitions>::new(reactive_instance.clone(), ty.clone() $(, $fn_name)*);
                let validator = <$validator>::new(reactive_instance.clone());
                let fsm = <$fsm>::new(ty, validator, transitions);
                let mut behaviour = $behaviour { reactive_instance, fsm };
                // TODO: auto connect
                behaviour
                    .fsm
                    .transition(BehaviourState::Connected)
                    .map_err(BehaviourCreationError::BehaviourTransitionError)?;
                Ok(Arc::new(behaviour))
            }
        }

        impl BehaviourFsm<ReactiveRelationInstance> for $behaviour {
            fn ty(&self) -> &BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> BehaviourState {
                self.fsm.state
            }

            fn set_state(&mut self, state: BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn BehaviourValidator<ReactiveRelationInstance> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn BehaviourTransitions<ReactiveRelationInstance> {
                &self.fsm.transitions
            }

            fn get_reactive_instance(&self) -> &Arc<ReactiveRelationInstance> {
                &self.reactive_instance
            }
        }

        impl Drop for $behaviour {
            fn drop(&mut self) {
                trace!("Drop relation behaviour {}", &self.fsm.ty);
            }
        }

        behaviour_factory!($factory, $behaviour, ReactiveRelationInstance $(, $fn_name, $fn_ident)*);

        behaviour_fsm!($fsm, $validator, $transitions, ReactiveRelationInstance);

        relation_behaviour_transitions!($transitions $(, $fn_name, $fn_ident)*);
    };
}
