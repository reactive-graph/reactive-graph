use crate::BehaviourTypeId;

pub trait ReactiveBehaviourContainer {
    /// Returns the behaviour types of the container.
    fn get_behaviours(&self) -> Vec<BehaviourTypeId>;

    /// Adds a behaviour to the container.
    fn add_behaviour(&self, ty: BehaviourTypeId);

    /// Removes a behaviour from the container.
    fn remove_behaviour(&self, ty: &BehaviourTypeId);

    /// Returns true, if the reactive instance behaves as the given behaviour.
    fn behaves_as(&self, ty: &BehaviourTypeId) -> bool;
}
