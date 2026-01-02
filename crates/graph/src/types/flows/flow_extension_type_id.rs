use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::ExtensionContainerGetter;
use crate::ExtensionTypeId;
use crate::FlowTypeId;
use crate::NamespacedType;

/// Addresses the extension of a flow type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct FlowExtensionTypeId {
    /// The flow type.
    #[builder(setter(into))]
    pub flow_ty: FlowTypeId,

    /// The extension type.
    #[builder(setter(into))]
    pub extension_ty: ExtensionTypeId,
}

impl FlowExtensionTypeId {
    pub fn new<F: Into<FlowTypeId>, E: Into<ExtensionTypeId>>(flow_ty: F, extension_ty: E) -> Self {
        Self {
            flow_ty: flow_ty.into(),
            extension_ty: extension_ty.into(),
        }
    }
}

impl ExtensionContainerGetter for FlowExtensionTypeId {
    fn container_ty(&self) -> NamespacedType {
        NamespacedType::from(&self.flow_ty)
    }

    fn extension_ty(&self) -> ExtensionTypeId {
        self.extension_ty.clone()
    }
}
