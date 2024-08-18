use crate::AddExtensionError;
use crate::Extension;
use crate::ExtensionTypeId;
use crate::ExtensionTypeIds;
use crate::Extensions;
use crate::RemoveExtensionError;
use crate::UpdateExtensionError;

pub trait ExtensionContainer {
    /// Returns true, if the container has an extension with the given type.
    fn has_own_extension(&self, ty: &ExtensionTypeId) -> bool;

    /// Returns true, if the type contains any of the given extensions.
    fn has_any_own_extensions(&self, tys: &ExtensionTypeIds) -> bool {
        tys.iter().any(|ty| self.has_own_extension(&ty))
    }

    /// Returns true, if the type contains all given extensions.
    fn has_all_own_extensions(&self, tys: &ExtensionTypeIds) -> bool {
        tys.iter().all(|ty| self.has_own_extension(&ty))
    }

    /// Returns the own extension with the given type.
    /// Doesn't respect extensions from potential components.
    fn get_own_extension(&self, ty: &ExtensionTypeId) -> Option<Extension>;

    fn add_extension<E: Into<Extension>>(&self, extension: E) -> Result<ExtensionTypeId, AddExtensionError>;

    fn update_extension<T: Into<ExtensionTypeId>, E: Into<Extension>>(&self, ty: T, extension: E) -> Result<Extension, UpdateExtensionError>;

    fn remove_extension<T: Into<ExtensionTypeId>>(&self, ty: T) -> Result<Extension, RemoveExtensionError>;

    /// Merge the given extensions into the own extensions.
    fn merge_extensions<E: Into<Extensions>>(&mut self, extensions_to_merge: E);
}

pub trait NamespacedTypeExtensionContainer<T, AddExtensionError, UpdateExtensionError, RemoveExtensionError, MergeExtensionsError> {
    fn add_extension<E: Into<Extension>>(&self, ty: &T, extension: E) -> Result<ExtensionTypeId, AddExtensionError>;

    fn update_extension<ET: Into<ExtensionTypeId>, E: Into<Extension>>(
        &self,
        ty: &T,
        extension_ty: ET,
        extension: E,
    ) -> Result<Extension, UpdateExtensionError>;

    fn remove_extension<ET: Into<ExtensionTypeId>>(&self, ty: &T, extension_ty: ET) -> Result<Extension, RemoveExtensionError>;

    /// Merge the given extensions into the own extensions.
    fn merge_extensions<E: Into<Extensions>>(&mut self, ty: &T, extensions_to_merge: E) -> Result<(), MergeExtensionsError>;
}
