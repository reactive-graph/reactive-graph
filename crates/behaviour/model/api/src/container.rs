use crate::BehaviourTypeId;
use crate::BehaviourTypeIds;

pub trait BehaviourTypeContainer {
    fn ty(&self) -> BehaviourTypeId;
}

pub trait BehaviourTypesContainer {
    /// Returns the behaviour types of the container.
    fn get_behaviours(&self) -> Vec<BehaviourTypeId>;

    /// Adds a behaviour to the container.
    fn add_behaviour(&self, ty: BehaviourTypeId);

    /// Removes a behaviour from the container.
    fn remove_behaviour(&self, ty: &BehaviourTypeId);

    /// Returns true, if the reactive instance behaves as the given behaviour.
    fn behaves_as(&self, ty: &BehaviourTypeId) -> bool;

    /// Returns true, if the reactive instance behaves as all the given behaviours.
    fn behaves_as_all(&self, tys: &BehaviourTypeIds) -> bool;
}
