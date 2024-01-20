use std::fmt::Display;
use std::fmt::Formatter;

use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use inexor_rgf_graph::ExtensionTypeId;
use inexor_rgf_graph::TYPE_ID_TYPE_SEPARATOR;

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

impl Display for ExtensionTypeIdDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "c{}{}{}{}", &TYPE_ID_TYPE_SEPARATOR, &self.namespace, &TYPE_ID_TYPE_SEPARATOR, &self.type_name)
    }
}
