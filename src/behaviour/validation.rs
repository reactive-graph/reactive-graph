use crate::model::ReactiveInstance;
use crate::BehaviourInvalid;
use crate::BehaviourPropertyInvalid;
use crate::BehaviourReactiveInstanceContainer;

pub trait BehaviourValidator<T: ReactiveInstance>: BehaviourPropertyValidator<T> {
    /// Validates the behaviour.
    fn validate(&self) -> Result<(), BehaviourInvalid> {
        self.validate_properties().map_err(|e| BehaviourInvalid::BehaviourPropertyInvalid(e))
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
