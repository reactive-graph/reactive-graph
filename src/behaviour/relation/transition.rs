#[macro_export]
macro_rules! relation_behaviour_transitions {
    ($transitions: ident $(, $fn_name:ident, $fn_ident: ident)*) => {
        pub struct $transitions {
            pub reactive_instance: Arc<ReactiveRelationInstance>,
            pub outbound_property_observers: EntityPropertyObserverContainerImpl,
            pub property_observers: RelationPropertyObserverContainerImpl,
            pub inbound_property_observers: EntityPropertyObserverContainerImpl,
            pub ty: BehaviourTypeId,
            $(pub $fn_name: $fn_ident,)*
        }

        impl $transitions {
            pub fn new(reactive_instance: Arc<ReactiveRelationInstance>, ty: BehaviourTypeId $(, $fn_name: $fn_ident)*) -> Self {
                let outbound_property_observers = EntityPropertyObserverContainerImpl::new(reactive_instance.outbound.clone());
                let property_observers = RelationPropertyObserverContainerImpl::new(reactive_instance.clone());
                let inbound_property_observers = EntityPropertyObserverContainerImpl::new(reactive_instance.inbound.clone());
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

        impl BehaviourDisconnect<ReactiveRelationInstance> for $transitions {
            fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
                trace!("remove all observers");
                self.inbound_property_observers.remove_all_observers();
                self.property_observers.remove_all_observers();
                self.outbound_property_observers.remove_all_observers();
                Ok(())
            }
        }

        impl BehaviourReactiveInstanceContainer<ReactiveRelationInstance> for $transitions {
            fn get_reactive_instance(&self) -> &Arc<ReactiveRelationInstance> {
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
