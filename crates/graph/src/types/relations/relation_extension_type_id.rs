use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::ExtensionContainerGetter;
use crate::ExtensionTypeId;
use crate::NamespacedType;
use crate::RelationTypeId;

/// Addresses the extension of a relation type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct RelationExtensionTypeId {
    /// The relation type.
    #[builder(setter(into))]
    pub relation_ty: RelationTypeId,

    /// The extension type.
    #[builder(setter(into))]
    pub extension_ty: ExtensionTypeId,
}

impl RelationExtensionTypeId {
    pub fn new<R: Into<RelationTypeId>, E: Into<ExtensionTypeId>>(relation_ty: R, extension_ty: E) -> Self {
        Self {
            relation_ty: relation_ty.into(),
            extension_ty: extension_ty.into(),
        }
    }
}

impl ExtensionContainerGetter for RelationExtensionTypeId {
    fn container_ty(&self) -> NamespacedType {
        NamespacedType::from(&self.relation_ty)
    }

    fn extension_ty(&self) -> ExtensionTypeId {
        self.extension_ty.clone()
    }
}
