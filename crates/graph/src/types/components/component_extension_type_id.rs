use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::ComponentTypeId;
use crate::ExtensionContainerGetter;
use crate::ExtensionTypeId;
use crate::NamespacedType;

/// Addresses the extension of a component.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct ComponentExtensionTypeId {
    /// The component type.
    #[builder(setter(into))]
    pub component_ty: ComponentTypeId,

    /// The extension type.
    #[builder(setter(into))]
    pub extension_ty: ExtensionTypeId,
}

impl ComponentExtensionTypeId {
    pub fn new<C: Into<ComponentTypeId>, E: Into<ExtensionTypeId>>(component_ty: C, extension_ty: E) -> Self {
        Self {
            component_ty: component_ty.into(),
            extension_ty: extension_ty.into(),
        }
    }
}

impl ExtensionContainerGetter for ComponentExtensionTypeId {
    fn container_ty(&self) -> NamespacedType {
        NamespacedType::from(&self.component_ty)
    }

    fn extension_ty(&self) -> ExtensionTypeId {
        self.extension_ty.clone()
    }
}
