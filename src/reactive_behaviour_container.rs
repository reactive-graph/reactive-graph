pub trait ReactiveBehaviourContainer {
    fn add_behaviour<S: Into<String>>(&self, behaviour: S);

    fn remove_behaviour<S: Into<String>>(&self, behaviour: S);

    /// Returns true, if the reactive instance behaves as the given behaviour.
    fn behaves_as<S: Into<String>>(&self, behaviour: S) -> bool;
}
