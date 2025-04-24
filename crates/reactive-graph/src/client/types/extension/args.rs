use clap::Args;
use reactive_graph_client::types::common::variables::type_id::variables::TypeIdVariables;
use reactive_graph_graph::ExtensionTypeId;
use serde_json::Value;

/// The property type.
#[derive(Args, Debug, Clone)]
pub(crate) struct ExtensionDefinitionArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ExtensionTypeIdArgs,

    /// Textual description of the extension.
    pub description: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

/// The extension type.
#[derive(Args, Debug, Clone)]
pub(crate) struct ExtensionTypeIdArgs {
    /// The extension namespace.
    pub extension_namespace: String,

    /// The extension name.
    pub extension_name: String,
}

impl From<ExtensionTypeIdArgs> for ExtensionTypeId {
    fn from(ty: ExtensionTypeIdArgs) -> Self {
        ExtensionTypeId::new_from_type(ty.extension_namespace, ty.extension_name)
    }
}

impl From<&ExtensionTypeIdArgs> for TypeIdVariables {
    fn from(ty: &ExtensionTypeIdArgs) -> Self {
        let ty: ExtensionTypeId = ty.clone().into();
        ty.into()
    }
}
