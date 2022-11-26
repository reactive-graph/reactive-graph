#[macro_export]
macro_rules! entity_behaviour_transitions {
    ($transitions: ident $(, $fn_name:ident, $fn_ident: ident)*) => {
        pub struct $transitions {
            pub reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveEntityInstance>,
            pub property_observers: EntityPropertyObserverContainerImpl,
            pub ty: inexor_rgf_core_model::BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $transitions {
            pub fn new(reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveEntityInstance>, ty: inexor_rgf_core_model::BehaviourTypeId $(, $fn_name: $fn_ident)*) -> Self {
                let property_observers = EntityPropertyObserverContainerImpl::new(reactive_instance.clone());
                $transitions {
                    reactive_instance,
                    property_observers,
                    ty,
                    $($fn_name,)*
                }
            }
        }

        impl BehaviourDisconnect<inexor_rgf_core_model::ReactiveEntityInstance> for $transitions {
            fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
                self.property_observers.remove_all_observers();
                Ok(())
            }
        }

        impl BehaviourReactiveInstanceContainer<inexor_rgf_core_model::ReactiveEntityInstance> for $transitions {
            fn get_reactive_instance(&self) -> &std::sync::Arc<inexor_rgf_core_model::ReactiveEntityInstance> {
                &self.reactive_instance
            }
        }

        impl Drop for $transitions {
            fn drop(&mut self) {
                let _ = self.disconnect();
                self.reactive_instance.remove_behaviour(&self.ty);
                let _ = self.shutdown();
            }
        }
    };
}
