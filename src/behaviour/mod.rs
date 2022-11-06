use std::sync::Arc;

use crate::model::BehaviourTypeId;
use crate::model::DataType;
use crate::model::ReactivePropertyContainer;

#[derive(Debug)]
pub enum BehaviourCreationError {
    /// Creating the behaviour failed because connecting the behaviour failed.
    BehaviourConnectFailed(BehaviourConnectFailed),
}

#[derive(Debug)]
pub enum BehaviourConnectFailed {
    /// Connecting the behaviour failed because the behaviour is invalid.
    BehaviourInvalid(BehaviourInvalid),
    BehaviourInitializationFailed,
}

#[derive(Debug)]
pub struct BehaviourInitializationFailed {}

#[derive(Debug)]
pub enum BehaviourInvalid {
    /// The behaviour is invalid because one or multiple properties are invalid.
    BehaviourPropertyInvalid(BehaviourPropertyInvalid),
}

#[derive(Debug)]
pub enum BehaviourPropertyInvalid {
    /// The property with the given name is missing.
    PropertyMissing(String),

    /// The property with the given name has a data type which is not the expected data type.
    InvalidDataType(String, DataType, DataType),
}

#[allow(drop_bounds)]
pub trait Behaviour<T: ReactivePropertyContainer>: BehaviourReactiveInstanceContainer<T> + BehaviourValidator<T> + BehaviourInitializer + Drop {
    /// Connects the reactive streams.
    fn connect(&self) -> Result<(), BehaviourConnectFailed> {
        // Validation Guard
        self.validate().map_err(|e| BehaviourConnectFailed::BehaviourInvalid(e))?;
        // Initialize the behaviour
        self.init().map_err(|_| BehaviourConnectFailed::BehaviourInitializationFailed)?;
        Ok(())
    }

    /// Disconnects the reactive streams.
    fn disconnect(&self) {}

    /// Returns the behaviour type.
    fn ty(&self) -> BehaviourTypeId;
}

pub trait BehaviourInitializer {
    /// Initializes the behaviour. For example, calculates and propagates the initial value.
    fn init(&self) -> Result<(), BehaviourInitializationFailed> {
        Ok(())
    }
}

pub trait BehaviourValidator<T: ReactivePropertyContainer>: BehaviourPropertyValidator<T> {
    /// Validates the behaviour.
    fn validate(&self) -> Result<(), BehaviourInvalid> {
        self.validate_properties().map_err(|e| BehaviourInvalid::BehaviourPropertyInvalid(e))
    }
}

pub trait BehaviourPropertyValidator<T: ReactivePropertyContainer>: BehaviourReactiveInstanceContainer<T> {
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

pub trait BehaviourReactiveInstanceContainer<T> {
    /// Returns the reactive instance of the behaviour.
    fn get_reactive_instance(&self) -> &Arc<T>;
}
