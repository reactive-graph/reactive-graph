#[macro_export]
macro_rules! relation_behaviour_transitions {
    ($transitions: ident $(, $fn_name:ident, $fn_ident: ident)*) => {
        pub struct $transitions {
            pub reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveRelationInstance>,
            pub outbound_property_observers: $crate::EntityPropertyObserverContainerImpl,
            pub property_observers: $crate::RelationPropertyObserverContainerImpl,
            pub inbound_property_observers: $crate::EntityPropertyObserverContainerImpl,
            pub ty: inexor_rgf_core_model::BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $transitions {
            pub fn new(reactive_instance: std::sync::Arc<inexor_rgf_core_model::ReactiveRelationInstance>, ty: inexor_rgf_core_model::BehaviourTypeId $(, $fn_name: $fn_ident)*) -> Self {
                let outbound_property_observers = $crate::EntityPropertyObserverContainerImpl::new(reactive_instance.outbound.clone());
                let property_observers = $crate::RelationPropertyObserverContainerImpl::new(reactive_instance.clone());
                let inbound_property_observers = $crate::EntityPropertyObserverContainerImpl::new(reactive_instance.inbound.clone());
                $transitions {
                    reactive_instance,
                    outbound_property_observers,
                    property_observers,
                    inbound_property_observers,
                    ty,
                    $($fn_name,)*
                }
            }
        }

        impl $crate::BehaviourDisconnect<inexor_rgf_core_model::ReactiveRelationInstance> for $transitions {
            fn disconnect(&self) -> Result<(), $crate::BehaviourDisconnectFailed> {
                self.inbound_property_observers.remove_all_observers();
                self.property_observers.remove_all_observers();
                self.outbound_property_observers.remove_all_observers();
                Ok(())
            }
        }

        impl $crate::BehaviourReactiveInstanceContainer<inexor_rgf_core_model::ReactiveRelationInstance> for $transitions {
            fn get_reactive_instance(&self) -> &std::sync::Arc<inexor_rgf_core_model::ReactiveRelationInstance> {
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
