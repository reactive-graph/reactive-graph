use crate::model::ReactiveInstance;
use crate::BehaviourInvalid;
use crate::BehaviourPropertyInvalid;
use crate::BehaviourReactiveInstanceContainer;

pub trait BehaviourValidator<T: ReactiveInstance>: BehaviourPropertyValidator<T> {
    /// Validates the behaviour.
    fn validate(&self) -> Result<(), BehaviourInvalid> {
        self.validate_properties().map_err(BehaviourInvalid::BehaviourPropertyInvalid)
    }
}

pub trait BehaviourPropertyValidator<T: ReactiveInstance>: BehaviourReactiveInstanceContainer<T> {
    /// Validates the properties of the reactive instance.
    fn validate_properties(&self) -> Result<(), BehaviourPropertyInvalid> {
        Ok(())
    }

    /// Validates the property with the given name.
    fn validate_property(&self, property_name: &str) -> Result<(), BehaviourPropertyInvalid> {
        let reactive_property_container = self.get_reactive_instance();
        if !reactive_property_container.has_property(property_name) {
            return Err(BehaviourPropertyInvalid::PropertyMissing(property_name.to_owned()));
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! behaviour_validator {
    ($validator: ident, $reactive_instance: ty) => {
        pub struct $validator {
            reactive_instance: std::sync::Arc<$reactive_instance>,
        }

        impl $validator {
            pub fn new(reactive_instance: std::sync::Arc<$reactive_instance>) -> Self {
                $validator { reactive_instance }
            }
        }

        impl BehaviourValidator<$reactive_instance> for $validator {}

        impl BehaviourReactiveInstanceContainer<$reactive_instance> for $validator {
            fn get_reactive_instance(&self) -> &std::sync::Arc<$reactive_instance> {
                &self.reactive_instance
            }
        }
    };

    ($validator: ident, $reactive_instance: ty $(, $property_names:expr)+) => {
        pub struct $validator {
            reactive_instance: std::sync::Arc<$reactive_instance>,
        }

        impl $validator {
            pub fn new(reactive_instance: std::sync::Arc<$reactive_instance>) -> Self {
                $validator { reactive_instance }
            }
        }

        impl BehaviourValidator<$reactive_instance> for $validator {}

        impl BehaviourReactiveInstanceContainer<$reactive_instance> for $validator {
            fn get_reactive_instance(&self) -> &std::sync::Arc<$reactive_instance> {
                &self.reactive_instance
            }
        }

        impl BehaviourPropertyValidator<$reactive_instance> for $validator {
            fn validate_properties(&self) -> Result<(), BehaviourPropertyInvalid> {
                $(
                self.validate_property($property_names)?;
                )*
                Ok(())
            }
        }
    };
}
