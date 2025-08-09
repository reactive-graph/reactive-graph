use async_graphql::OneofObject;
use serde::Deserialize;
use serde::Serialize;

use reactive_graph_graph::InboundOutboundType;

#[derive(Serialize, Deserialize, Clone, Debug, OneofObject)]
#[graphql(name = "ComponentOrEntityTypeIdDefinition")]
pub enum ComponentOrEntityTypeIdDefinition {
    #[serde(rename = "component")]
    Component(ComponentTypeIdDefinition),
    #[serde(rename = "entity_type")]
    EntityType(EntityTypeIdDefinition),
}
impl From<ComponentOrEntityTypeIdDefinition> for InboundOutboundType {
    fn from(ty: ComponentOrEntityTypeIdDefinition) -> Self {
        match ty {
            ComponentOrEntityTypeIdDefinition::Component(ty) => InboundOutboundType::Component(ty.into()),
            ComponentOrEntityTypeIdDefinition::EntityType(ty) => InboundOutboundType::EntityType(ty.into()),
        }
    }
}

impl From<InboundOutboundType> for ComponentOrEntityTypeIdDefinition {
    fn from(ty: InboundOutboundType) -> Self {
        match ty {
            InboundOutboundType::Component(ty) => ComponentOrEntityTypeIdDefinition::Component(ty.into()),
            InboundOutboundType::EntityType(ty) => ComponentOrEntityTypeIdDefinition::EntityType(ty.into()),
        }
    }
}
