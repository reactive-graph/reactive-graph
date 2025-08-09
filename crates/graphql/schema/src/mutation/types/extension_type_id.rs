use std::fmt::Display;
use std::fmt::Formatter;

use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::NAMESPACE_SEPARATOR;
use reactive_graph_graph::NamespacedTypeGetter;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, InputObject)]
#[graphql(name = "ExtensionTypeId")]
pub struct ExtensionTypeIdDefinition {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    #[graphql(name = "name")]
    pub type_name: String,
}

impl From<ExtensionTypeIdDefinition> for ExtensionTypeId {
    fn from(ty: ExtensionTypeIdDefinition) -> Self {
        ExtensionTypeId::new_from_type(ty.namespace, ty.type_name)
    }
}

impl From<ExtensionTypeId> for ExtensionTypeIdDefinition {
    fn from(ty: ExtensionTypeId) -> Self {
        ExtensionTypeIdDefinition {
            namespace: ty.namespace(),
            type_name: ty.type_name(),
        }
    }
}

impl Display for ExtensionTypeIdDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "c{}{}{}{}", &NAMESPACE_SEPARATOR, &self.namespace, &NAMESPACE_SEPARATOR, &self.type_name)
    }
}
