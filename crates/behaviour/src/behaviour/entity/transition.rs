#[macro_export]
macro_rules! entity_behaviour_transitions {
    ($transitions: ident $(, $fn_name:ident, $fn_ident: ident)*) => {
        pub struct $transitions {
            pub reactive_instance: inexor_rgf_reactive::ReactiveEntity,
            pub property_observers: $crate::EntityPropertyObserverContainerImpl,
            pub ty: inexor_rgf_behaviour_api::BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $transitions {
            pub fn new(reactive_instance: inexor_rgf_reactive::ReactiveEntity, ty: inexor_rgf_behaviour_api::BehaviourTypeId $(, $fn_name: $fn_ident)*) -> Self {
                let property_observers = $crate::EntityPropertyObserverContainerImpl::new(reactive_instance.clone());
                $transitions {
                    reactive_instance,
                    property_observers,
                    ty,
                    $($fn_name,)*
                }
            }
        }

        impl $crate::BehaviourDisconnect<uuid::Uuid, inexor_rgf_reactive::ReactiveEntity> for $transitions {
            fn disconnect(&self) -> Result<(), $crate::BehaviourDisconnectFailed> {
                self.property_observers.remove_all_observers();
                Ok(())
            }
        }

        impl inexor_rgf_reactive_api::ReactiveInstanceContainer<uuid::Uuid, inexor_rgf_reactive::ReactiveEntity> for $transitions {
            fn get_reactive_instance(&self) -> &inexor_rgf_reactive::ReactiveEntity {
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
