use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use reactive_graph_graph::FlowTypeId;

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "FlowTypeId")]
pub struct FlowTypeIdDefinition {
    /// The namespace of the flow type.
    pub namespace: String,

    /// The name of the flow type.
    #[graphql(name = "name")]
    pub type_name: String,
}

impl From<FlowTypeIdDefinition> for FlowTypeId {
    fn from(ty: FlowTypeIdDefinition) -> Self {
        FlowTypeId::new_from_type(ty.namespace, ty.type_name)
    }
}
