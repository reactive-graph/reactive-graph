#[macro_export]
macro_rules! entity_behaviour_transitions {
    ($transitions: ident $(, $fn_name:ident, $fn_ident: ident)*) => {
        pub struct $transitions {
            pub reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveEntity>,
            pub property_observers: $crate::EntityPropertyObserverContainerImpl,
            pub ty: inexor_rgf_core_model::BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $transitions {
            pub fn new(reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveEntity>, ty: inexor_rgf_core_model::BehaviourTypeId $(, $fn_name: $fn_ident)*) -> Self {
                let property_observers = $crate::EntityPropertyObserverContainerImpl::new(reactive_instance.clone());
                $transitions {
                    reactive_instance,
                    property_observers,
                    ty,
                    $($fn_name,)*
                }
            }
        }

        impl $crate::BehaviourDisconnect<inexor_rgf_core_model::ReactiveEntity> for $transitions {
            fn disconnect(&self) -> Result<(), $crate::BehaviourDisconnectFailed> {
                self.property_observers.remove_all_observers();
                Ok(())
            }
        }

        impl $crate::BehaviourReactiveInstanceContainer<inexor_rgf_core_model::ReactiveEntity> for $transitions {
            fn get_reactive_instance(&self) -> &std::sync::Arc<inexor_rgf_core_model::ReactiveEntity> {
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
