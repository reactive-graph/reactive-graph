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
            pub reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveEntityInstance>,
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveEntityInstance>, ty: inexor_rgf_core_model::BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<std::sync::Arc<$behaviour>, BehaviourCreationError> {
                let transitions = <$transitions>::new(reactive_instance.clone(), ty.clone() $(, $fn_name)*);
                let validator = <$validator>::new(reactive_instance.clone());
                let fsm = <$fsm>::new(reactive_instance.clone(), ty, validator, transitions);
                let mut behaviour = $behaviour { reactive_instance, fsm };
                behaviour
                    .fsm
                    .transition(BehaviourState::Connected)
                    .map_err(BehaviourCreationError::BehaviourTransitionError)?;
                Ok(std::sync::Arc::new(behaviour))
            }
        }

        impl BehaviourFsm<inexor_rgf_core_model::ReactiveEntityInstance> for $behaviour {
            fn ty(&self) -> &inexor_rgf_core_model::BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> BehaviourState {
                self.fsm.get_state()
            }

            fn set_state(&self, state: BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn BehaviourValidator<inexor_rgf_core_model::ReactiveEntityInstance> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn BehaviourTransitions<inexor_rgf_core_model::ReactiveEntityInstance> {
                &self.fsm.transitions
            }
        }

        impl BehaviourReactiveInstanceContainer<inexor_rgf_core_model::ReactiveEntityInstance> for $behaviour {
            fn get_reactive_instance(&self) -> &std::sync::Arc<inexor_rgf_core_model::ReactiveEntityInstance> {
                &self.reactive_instance
            }

            fn get(&self, property_name: &str) -> Option<serde_json::Value> {
                self.reactive_instance.get(property_name)
            }

            fn set(&self, property_name: &str, value: serde_json::Value) {
                self.reactive_instance.set(property_name, value);
            }
        }

        impl Drop for $behaviour {
            fn drop(&mut self) {
                log::trace!("Drop entity behaviour {}", &self.fsm.ty);
            }
        }

        behaviour_factory!($factory, $behaviour, ReactiveEntityInstance $(, $fn_name, $fn_ident)*);

        behaviour_fsm!($fsm, $validator, $transitions, ReactiveEntityInstance);

        entity_behaviour_transitions!($transitions $(, $fn_name, $fn_ident)*);
    };
}
