use crate::Extension;
use crate::ExtensionTypeId;

pub trait ExtensionContainer {
    /// Returns true, if the container has an extension with the given type.
    fn has_own_extension(&self, extension_ty: &ExtensionTypeId) -> bool;

    /// Returns the own extension with the given type.
    /// Doesn't respect extensions from potential components.
    fn get_own_extension(&self, extension_ty: &ExtensionTypeId) -> Option<Extension>;

    /// Merge the given extensions into the own extensions.
    fn merge_extensions(&mut self, extensions_to_merge: Vec<Extension>);
}
