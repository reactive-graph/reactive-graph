pub trait TypeDescriptionGetter {
    /// Returns the description of the type.
    fn description(&self) -> String;
}
