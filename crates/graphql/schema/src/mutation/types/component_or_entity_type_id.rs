use async_graphql::OneofObject;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeParseError;

#[derive(Serialize, Deserialize, Clone, Debug, OneofObject)]
#[graphql(name = "InboundOutboundType")]
pub enum GraphQLInboundOutboundType {
    #[serde(rename = "component")]
    Component(String),
    #[serde(rename = "entity_type")]
    EntityType(String),
}

impl TryFrom<GraphQLInboundOutboundType> for InboundOutboundType {
    type Error = NamespacedTypeParseError;

    fn try_from(ty: GraphQLInboundOutboundType) -> Result<Self, Self::Error> {
        Ok(match ty {
            GraphQLInboundOutboundType::Component(_type) => match _type.as_str() {
                "*" => InboundOutboundType::Component(MatchingInboundOutboundType::Any),
                _ => InboundOutboundType::Component(MatchingInboundOutboundType::NamespacedType(ComponentTypeId::from_str(&_type)?)),
            },
            GraphQLInboundOutboundType::EntityType(_type) => match _type.as_str() {
                "*" => InboundOutboundType::EntityType(MatchingInboundOutboundType::Any),
                _ => InboundOutboundType::EntityType(MatchingInboundOutboundType::NamespacedType(EntityTypeId::from_str(&_type)?)),
            },
        })
    }
}

impl From<InboundOutboundType> for GraphQLInboundOutboundType {
    fn from(ty: InboundOutboundType) -> Self {
        match ty {
            InboundOutboundType::Component(ty) => GraphQLInboundOutboundType::Component(ty.to_string()),
            InboundOutboundType::EntityType(ty) => GraphQLInboundOutboundType::EntityType(ty.to_string()),
        }
    }
}
