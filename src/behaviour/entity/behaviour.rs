#[macro_export]
macro_rules! entity_behaviour {
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
            pub reactive_instance: Arc<ReactiveEntityInstance>,
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: Arc<ReactiveEntityInstance>, ty: BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<Arc<$behaviour>, BehaviourCreationError> {
                let transitions = <$transitions>::new(reactive_instance.clone(), ty.clone() $(, $fn_name)*);
                let validator = <$validator>::new(reactive_instance.clone());
                let fsm = <$fsm>::new(reactive_instance.clone(), ty, validator, transitions);
                let mut behaviour = $behaviour { reactive_instance, fsm };
                behaviour
                    .fsm
                    .transition(BehaviourState::Connected)
                    .map_err(BehaviourCreationError::BehaviourTransitionError)?;
                Ok(Arc::new(behaviour))
            }
        }

        impl BehaviourFsm<ReactiveEntityInstance> for $behaviour {
            fn ty(&self) -> &BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> BehaviourState {
                self.fsm.get_state()
            }

            fn set_state(&self, state: BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn BehaviourValidator<ReactiveEntityInstance> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn BehaviourTransitions<ReactiveEntityInstance> {
                &self.fsm.transitions
            }
        }

        impl BehaviourReactiveInstanceContainer<ReactiveEntityInstance> for $behaviour {
            fn get_reactive_instance(&self) -> &Arc<ReactiveEntityInstance> {
                &self.reactive_instance
            }

            fn get(&self, property_name: &str) -> Option<Value> {
                self.reactive_instance.get(property_name)
            }

            fn set(&self, property_name: &str, value: Value) {
                self.reactive_instance.set(property_name, value);
            }
        }

        impl Drop for $behaviour {
            fn drop(&mut self) {
                trace!("Drop entity behaviour {}", &self.fsm.ty);
            }
        }

        behaviour_factory!($factory, $behaviour, ReactiveEntityInstance $(, $fn_name, $fn_ident)*);

        behaviour_fsm!($fsm, $validator, $transitions, ReactiveEntityInstance);

        entity_behaviour_transitions!($transitions $(, $fn_name, $fn_ident)*);
    };
}
