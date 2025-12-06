use std::ops::Deref;
use thiserror::Error;

use crate::NamespaceError;
use crate::NamespaceSegment;
use crate::NamespaceSegmentError;
use crate::NamespacedType;
use crate::NamespacedTypeError;
use crate::NamespacedTypeIdContainer;
use crate::namespace::Namespace;

#[derive(Debug, Error)]
pub enum NamespacedTypeIdsError {
    #[error("The parent namespace {0} must not be a type")]
    ParentNamespaceMustNotBeAType(Namespace),
    #[error("The type name {0} is invalid")]
    InvalidTypeName(NamespaceSegment),
    #[error("Failed to construct a new namespace for type name {0} because: {1}")]
    NamespaceError(NamespaceSegment, NamespaceError),
    #[error("Failed to construct a child namespace of parent namespace {0} because: {1}")]
    ChildNamespaceError(Namespace, NamespacedTypeError),
    #[error("Failed to construct a namespaced type because: {0}")]
    NamespacedTypeError(#[from] NamespacedTypeError),
    #[error("The type name is not a valid type: {0}")]
    NamespaceSegmentError(#[from] NamespaceSegmentError),
}

/// Bind T to a specific namespace.
pub struct NamespacedTypeIds<T: NamespacedTypeIdContainer> {
    /// The namespace to use
    parent_namespace: Namespace,
    tys: T::TypeIds,
}

impl<T: NamespacedTypeIdContainer<TypeIds = T>> NamespacedTypeIds<T> {
    pub fn new<N: Into<Namespace>>(parent_namespace: N) -> Result<Self, NamespacedTypeIdsError> {
        let parent_namespace = parent_namespace.into();
        if parent_namespace.is_type() {
            return Err(NamespacedTypeIdsError::ParentNamespaceMustNotBeAType(parent_namespace));
        }
        Ok(Self {
            parent_namespace,
            tys: T::new(),
        })
    }

    pub fn ty<N: Into<NamespaceSegment>>(self, type_name: N) -> Result<Self, NamespacedTypeIdsError>
    where
        <T as NamespacedTypeIdContainer>::TypeId: From<NamespacedType>,
    {
        let type_name = type_name.into();
        if !type_name.is_type() {
            return Err(NamespacedTypeIdsError::InvalidTypeName(type_name));
        }
        let parent_namespace = self.parent_namespace.clone();

        let type_namespace = parent_namespace
            .try_append_segment(type_name.clone())
            .map_err(|e| NamespacedTypeIdsError::NamespaceError(type_name, e))?;

        let namespaced_type = NamespacedType::new(type_namespace.clone()).map_err(|e| NamespacedTypeIdsError::ChildNamespaceError(type_namespace, e))?;

        self.tys.insert(namespaced_type.into());
        Ok(self)
    }
}

impl<T: NamespacedTypeIdContainer<TypeIds = T>> Deref for NamespacedTypeIds<T> {
    type Target = T::TypeIds;

    fn deref(&self) -> &Self::Target {
        &self.tys
    }
}
