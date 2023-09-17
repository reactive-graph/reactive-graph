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
            // Function name.
            $fn_name: ident,
            // Function.
            $fn_ident: ident
        )*
        $(,)?
    ) => {
        pub struct $behaviour {
            pub reactive_instance: inexor_rgf_reactive::ReactiveRelation,
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: inexor_rgf_reactive::ReactiveRelation, ty: inexor_rgf_behaviour_api::BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<std::sync::Arc<$behaviour>, $crate::BehaviourCreationError> {
                let transitions = <$transitions>::new(reactive_instance.clone(), ty.clone() $(, $fn_name)*);
                let validator = <$validator>::new(reactive_instance.clone());
                let fsm = <$fsm>::new(reactive_instance.clone(), ty, validator, transitions);
                let mut behaviour = $behaviour { reactive_instance, fsm };
                // TODO: auto connect
                behaviour
                    .fsm
                    .transition($crate::BehaviourState::Connected)
                    .map_err($crate::BehaviourCreationError::BehaviourTransitionError)?;
                Ok(std::sync::Arc::new(behaviour))
            }
        }

        impl inexor_rgf_behaviour_api::BehaviourFsm<inexor_rgf_graph::RelationInstanceId, inexor_rgf_reactive::ReactiveRelation> for $behaviour {
            fn ty(&self) -> &inexor_rgf_behaviour_api::BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> inexor_rgf_behaviour_api::BehaviourState {
                self.fsm.get_state()
            }

            fn set_state(&self, state: inexor_rgf_behaviour_api::BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn inexor_rgf_behaviour_api::BehaviourValidator<inexor_rgf_graph::RelationInstanceId, inexor_rgf_reactive::ReactiveRelation> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn inexor_rgf_behaviour_api::BehaviourTransitions<inexor_rgf_graph::RelationInstanceId, inexor_rgf_reactive::ReactiveRelation> {
                &self.fsm.transitions
            }
        }

        impl inexor_rgf_reactive_api::ReactiveInstanceContainer<inexor_rgf_graph::RelationInstanceId, inexor_rgf_reactive::ReactiveRelation> for $behaviour {
            fn get_reactive_instance(&self) -> &inexor_rgf_reactive::ReactiveRelation {
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
                log::trace!("Drop relation behaviour {}", &self.fsm.ty);
            }
        }

        inexor_rgf_behaviour_api::behaviour_factory!($factory, $behaviour, inexor_rgf_graph::RelationInstanceId, inexor_rgf_reactive::ReactiveRelation $(, $fn_name, $fn_ident)*);

        inexor_rgf_behaviour_api::behaviour_fsm!($fsm, $validator, $transitions, inexor_rgf_graph::RelationInstanceId, inexor_rgf_reactive::ReactiveRelation);

        $crate::relation_behaviour_transitions!($transitions $(, $fn_name, $fn_ident)*);
    };
}
