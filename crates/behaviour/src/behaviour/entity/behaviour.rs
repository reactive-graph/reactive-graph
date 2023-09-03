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
            // Function name.
            $fn_name: ident,
            // Function.
            $fn_ident: ident
        )*
    ) => {
        use inexor_rgf_core_model::ReactiveEntity as ModelReactiveEntity;
        use inexor_rgf_core_model::PropertyInstanceGetter as EntityBehaviourPropertyInstanceGetter;

        pub struct $behaviour {
            pub reactive_instance: std::sync::Arc<ModelReactiveEntity>,
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: std::sync::Arc<ModelReactiveEntity>, ty: inexor_rgf_core_model::BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<std::sync::Arc<$behaviour>, $crate::BehaviourCreationError> {
                let transitions = <$transitions>::new(reactive_instance.clone(), ty.clone() $(, $fn_name)*);
                let validator = <$validator>::new(reactive_instance.clone());
                let fsm = <$fsm>::new(reactive_instance.clone(), ty, validator, transitions);
                let mut behaviour = $behaviour { reactive_instance, fsm };
                behaviour
                    .fsm
                    .transition($crate::BehaviourState::Connected)
                    .map_err($crate::BehaviourCreationError::BehaviourTransitionError)?;
                Ok(std::sync::Arc::new(behaviour))
            }
        }

        impl $crate::BehaviourFsm<ModelReactiveEntity> for $behaviour {
            fn ty(&self) -> &inexor_rgf_core_model::BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> $crate::BehaviourState {
                self.fsm.get_state()
            }

            fn set_state(&self, state: $crate::BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn $crate::BehaviourValidator<ModelReactiveEntity> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn $crate::BehaviourTransitions<ModelReactiveEntity> {
                &self.fsm.transitions
            }
        }

        impl $crate::BehaviourReactiveInstanceContainer<ModelReactiveEntity> for $behaviour {
            fn get_reactive_instance(&self) -> &std::sync::Arc<ModelReactiveEntity> {
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

        $crate::behaviour_factory!($factory, $behaviour, ModelReactiveEntity $(, $fn_name, $fn_ident)*);

        $crate::behaviour_fsm!($fsm, $validator, $transitions, ModelReactiveEntity);

        $crate::entity_behaviour_transitions!($transitions $(, $fn_name, $fn_ident)*);
    };
}
