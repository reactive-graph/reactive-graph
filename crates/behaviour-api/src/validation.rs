use inexor_rgf_reactive_api::prelude::*;

use crate::BehaviourInvalid;
use crate::BehaviourPropertyInvalid;
use crate::BehaviourTypesContainer;

pub trait BehaviourValidator<ID: Clone, T: ReactiveInstance<ID> + BehaviourTypesContainer>: BehaviourPropertyValidator<ID, T> {
    /// Validates the behaviour.
    fn validate(&self) -> Result<(), BehaviourInvalid> {
        self.validate_properties().map_err(BehaviourInvalid::BehaviourPropertyInvalid)
    }
}

pub trait BehaviourPropertyValidator<ID: Clone, T: ReactiveInstance<ID> + BehaviourTypesContainer>: ReactiveInstanceContainer<ID, T> {
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
    ($validator: ident, $id: ty, $reactive_instance: ty) => {
        pub struct $validator {
            reactive_instance: $reactive_instance,
        }

        impl $validator {
            pub fn new(reactive_instance: $reactive_instance) -> Self {
                $validator { reactive_instance }
            }
        }

        impl BehaviourValidator<$id, $reactive_instance> for $validator {}

        impl inexor_rgf_reactive_api::ReactiveInstanceContainer<$id, $reactive_instance> for $validator {
            fn get_reactive_instance(&self) -> &$reactive_instance {
                &self.reactive_instance
            }
        }
    };

    ($validator: ident, $id: ty, $reactive_instance: ty $(, $property_names:expr)+) => {
        pub struct $validator {
            reactive_instance: $reactive_instance,
        }

        impl $validator {
            pub fn new(reactive_instance: $reactive_instance) -> Self {
                $validator { reactive_instance }
            }
        }

        impl $crate::BehaviourValidator<$id, $reactive_instance> for $validator {}

        impl inexor_rgf_reactive_api::ReactiveInstanceContainer<$id, $reactive_instance> for $validator {
            fn get_reactive_instance(&self) -> &$reactive_instance {
                &self.reactive_instance
            }
        }

        impl $crate::BehaviourPropertyValidator<$id, $reactive_instance> for $validator {
            fn validate_properties(&self) -> Result<(), $crate::BehaviourPropertyInvalid> {
                $(
                self.validate_property($property_names)?;
                )*
                Ok(())
            }
        }
    };
}
