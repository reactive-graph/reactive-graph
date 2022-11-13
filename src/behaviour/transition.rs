use crate::model::ReactiveInstance;
use crate::BehaviourConnectFailed;
use crate::BehaviourDisconnectFailed;
use crate::BehaviourInitializationFailed;
use crate::BehaviourShutdownFailed;
use crate::PropertyObserverContainer;
use crate::PropertyObserverContainerImpl;

#[allow(drop_bounds)]
pub trait BehaviourTransitions<T: ReactiveInstance>: Drop {
    /// Initializes the behaviour. For example, calculates and propagates the initial value.
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        Ok(())
    }

    /// Connects the reactive streams.
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        Ok(())
    }

    /// Disconnects the reactive streams.
    fn disconnect(&self) -> Result<(), BehaviourDisconnectFailed> {
        self.get_property_observers().remove_all_observers();
        Ok(())
    }

    /// Destructs the behaviour.
    fn shutdown(&self) -> Result<(), BehaviourShutdownFailed> {
        Ok(())
    }

    fn get_property_observers(&self) -> &PropertyObserverContainerImpl<T>;
}

#[macro_export]
macro_rules! behaviour_transitions {
    ($transitions: ident, $reactive_instance: ty) => {
        pub struct $transitions {
            pub property_observers: PropertyObserverContainerImpl<$reactive_instance>,
        }

        impl $transitions {
            pub fn new(property_observers: PropertyObserverContainerImpl<$reactive_instance>) -> Self {
                $transitions { property_observers }
            }
        }

        impl Drop for $transitions {
            fn drop(&mut self) {
                let _ = self.disconnect();
                let _ = self.shutdown();
            }
        }
    };
}
