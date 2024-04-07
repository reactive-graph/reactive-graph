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
            pub reactive_instance: reactive_graph_reactive_model_impl::ReactiveRelation,
            pub fsm: $fsm,
        }

        impl $behaviour {
            pub fn new(reactive_instance: reactive_graph_reactive_model_impl::ReactiveRelation, ty: reactive_graph_behaviour_model_api::BehaviourTypeId, $($fn_name: $fn_ident)*) -> Result<std::sync::Arc<$behaviour>, $crate::BehaviourCreationError> {
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

        impl reactive_graph_behaviour_model_api::BehaviourFsm<reactive_graph_graph::RelationInstanceId, reactive_graph_reactive_model_impl::ReactiveRelation> for $behaviour {
            fn ty(&self) -> &reactive_graph_behaviour_model_api::BehaviourTypeId {
                &self.fsm.ty
            }

            fn get_state(&self) -> reactive_graph_behaviour_model_api::BehaviourState {
                self.fsm.get_state()
            }

            fn set_state(&self, state: reactive_graph_behaviour_model_api::BehaviourState) {
                self.fsm.set_state(state);
            }

            fn get_validator(&self) -> &dyn reactive_graph_behaviour_model_api::BehaviourValidator<reactive_graph_graph::RelationInstanceId, reactive_graph_reactive_model_impl::ReactiveRelation> {
                &self.fsm.validator
            }

            fn get_transitions(&self) -> &dyn reactive_graph_behaviour_model_api::BehaviourTransitions<reactive_graph_graph::RelationInstanceId, reactive_graph_reactive_model_impl::ReactiveRelation> {
                &self.fsm.transitions
            }
        }

        impl reactive_graph_reactive_model_api::ReactiveInstanceContainer<reactive_graph_graph::RelationInstanceId, reactive_graph_reactive_model_impl::ReactiveRelation> for $behaviour {
            fn get_reactive_instance(&self) -> &reactive_graph_reactive_model_impl::ReactiveRelation {
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

        reactive_graph_behaviour_model_api::behaviour_factory!($factory, $behaviour, reactive_graph_graph::RelationInstanceId, reactive_graph_reactive_model_impl::ReactiveRelation $(, $fn_name, $fn_ident)*);

        reactive_graph_behaviour_model_api::behaviour_fsm!($fsm, $validator, $transitions, reactive_graph_graph::RelationInstanceId, reactive_graph_reactive_model_impl::ReactiveRelation);

        $crate::relation_behaviour_transitions!($transitions $(, $fn_name, $fn_ident)*);
    };
}
