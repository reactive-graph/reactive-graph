pub trait NamedInstanceContainer {
    /// Returns the name of the instance.
    fn name(&self) -> String;

    /// Returns the description of the instance.
    fn description(&self) -> String;
}
