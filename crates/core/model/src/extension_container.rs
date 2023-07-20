use crate::Extension;
use crate::ExtensionTypeId;

pub trait ExtensionContainer {
    /// Returns true, if the container has an extension with the given type.
    fn has_own_extension(&self, extension_ty: &ExtensionTypeId) -> bool;

    /// Returns true, if the type contains any of the given extensions.
    fn has_any_own_extensions(&self, extension_tys: &Vec<ExtensionTypeId>) -> bool {
        extension_tys.iter().any(|ty| self.has_own_extension(ty))
    }

    /// Returns true, if the type contains all given extensions.
    fn has_all_own_extensions(&self, extension_tys: &Vec<ExtensionTypeId>) -> bool {
        extension_tys.iter().all(|ty| self.has_own_extension(ty))
    }

    /// Returns the own extension with the given type.
    /// Doesn't respect extensions from potential components.
    fn get_own_extension(&self, extension_ty: &ExtensionTypeId) -> Option<Extension>;

    /// Merge the given extensions into the own extensions.
    fn merge_extensions(&mut self, extensions_to_merge: Vec<Extension>);
}
