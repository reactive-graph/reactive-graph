use crate::Extension;

pub trait ExtensionContainer {
    /// Returns true, if the container has an extension with the given name.
    fn has_own_extension<S: Into<String>>(&self, extension_name: S) -> bool;

    /// Returns the own extension with the given name.
    /// Doesn't respect extensions from potential components.
    fn get_own_extension<S: Into<String>>(&self, extension_name: S) -> Option<Extension>;
}
