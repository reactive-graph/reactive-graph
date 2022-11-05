use crate::model::BehaviourTypeId;

#[derive(Debug)]
pub struct BehaviourCreationError;

pub trait Behaviour {
    /// Wires the reactive streams.
    fn connect(&self) {}

    /// Disconnects the reactive streams.
    fn disconnect(&self) {}

    /// Returns the behaviour type.
    fn ty(&self) -> BehaviourTypeId;
}
