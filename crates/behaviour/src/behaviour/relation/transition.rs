#[macro_export]
macro_rules! relation_behaviour_transitions {
    ($transitions: ident $(, $fn_name:ident, $fn_ident: ident)*) => {
        pub struct $transitions {
            pub reactive_instance: inexor_rgf_reactive::ReactiveRelation,
            pub outbound_property_observers: $crate::EntityPropertyObserverContainerImpl,
            pub property_observers: $crate::RelationPropertyObserverContainerImpl,
            pub inbound_property_observers: $crate::EntityPropertyObserverContainerImpl,
            pub ty: inexor_rgf_behaviour_api::BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $transitions {
            pub fn new(reactive_instance: inexor_rgf_reactive::ReactiveRelation, ty: inexor_rgf_behaviour_api::BehaviourTypeId $(, $fn_name: $fn_ident)*) -> Self {
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

        impl $crate::BehaviourDisconnect<inexor_rgf_graph::RelationInstanceId, inexor_rgf_reactive::ReactiveRelation> for $transitions {
            fn disconnect(&self) -> Result<(), $crate::BehaviourDisconnectFailed> {
                self.inbound_property_observers.remove_all_observers();
                self.property_observers.remove_all_observers();
                self.outbound_property_observers.remove_all_observers();
                Ok(())
            }
        }

        impl inexor_rgf_reactive_api::ReactiveInstanceContainer<inexor_rgf_reactive::ReactiveRelation> for $transitions {
            fn get_reactive_instance(&self) -> &inexor_rgf_reactive::ReactiveRelation {
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
