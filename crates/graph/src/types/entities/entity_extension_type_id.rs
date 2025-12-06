use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::EntityTypeId;
use crate::ExtensionContainerGetter;
use crate::ExtensionTypeId;
use crate::NamespacedType;

/// Addresses the extension of an entity type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct EntityExtensionTypeId {
    /// The entity type.
    #[builder(setter(into))]
    pub entity_ty: EntityTypeId,

    /// The extension type.
    #[builder(setter(into))]
    pub extension_ty: ExtensionTypeId,
}

impl EntityExtensionTypeId {
    pub fn new<E: Into<EntityTypeId>, EX: Into<ExtensionTypeId>>(entity_ty: E, extension_ty: EX) -> Self {
        Self {
            entity_ty: entity_ty.into(),
            extension_ty: extension_ty.into(),
        }
    }
}

impl ExtensionContainerGetter for EntityExtensionTypeId {
    fn container_ty(&self) -> NamespacedType {
        NamespacedType::from(&self.entity_ty)
    }

    fn extension_ty(&self) -> ExtensionTypeId {
        self.extension_ty.clone()
    }
}
