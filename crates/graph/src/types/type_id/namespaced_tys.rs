use std::ops::Deref;

use crate::NamespacedType;
use crate::NamespacedTypeIdContainer;

/// Bind T to a specific namespace.
pub struct NamespacedTypeIds<T: NamespacedTypeIdContainer> {
    /// The namespace to use
    namespace: String,
    tys: T::TypeIds,
}

impl<T: NamespacedTypeIdContainer<TypeIds = T>> NamespacedTypeIds<T> {
    pub fn new<N: Into<String>>(namespace: N) -> Self {
        Self {
            namespace: namespace.into(),
            tys: T::new(),
        }
    }

    pub fn ty<S: Into<String>>(self, type_name: S) -> Self
    where
        <T as NamespacedTypeIdContainer>::TypeId: From<NamespacedType>,
    {
        let nt = NamespacedType::new(self.namespace.clone(), type_name.into());
        self.tys.insert(nt.into());
        self
    }
}

impl<T: NamespacedTypeIdContainer<TypeIds = T>> Deref for NamespacedTypeIds<T> {
    type Target = T::TypeIds;

    fn deref(&self) -> &Self::Target {
        &self.tys
    }
}
