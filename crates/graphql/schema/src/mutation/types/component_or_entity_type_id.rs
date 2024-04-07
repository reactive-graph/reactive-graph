use async_graphql::OneofObject;
use serde::Deserialize;
use serde::Serialize;

use reactive_graph_graph::ComponentOrEntityTypeId;

use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::EntityTypeIdDefinition;

#[derive(Serialize, Deserialize, Clone, Debug, OneofObject)]
#[graphql(name = "ComponentOrEntityTypeIdDefinition")]
pub enum ComponentOrEntityTypeIdDefinition {
    #[serde(rename = "component")]
    Component(ComponentTypeIdDefinition),
    #[serde(rename = "entity_type")]
    EntityType(EntityTypeIdDefinition),
}
impl From<ComponentOrEntityTypeIdDefinition> for ComponentOrEntityTypeId {
    fn from(ty: ComponentOrEntityTypeIdDefinition) -> Self {
        match ty {
            ComponentOrEntityTypeIdDefinition::Component(ty) => ComponentOrEntityTypeId::Component(ty.into()),
            ComponentOrEntityTypeIdDefinition::EntityType(ty) => ComponentOrEntityTypeId::EntityType(ty.into()),
        }
    }
}

impl From<ComponentOrEntityTypeId> for ComponentOrEntityTypeIdDefinition {
    fn from(ty: ComponentOrEntityTypeId) -> Self {
        match ty {
            ComponentOrEntityTypeId::Component(ty) => ComponentOrEntityTypeIdDefinition::Component(ty.into()),
            ComponentOrEntityTypeId::EntityType(ty) => ComponentOrEntityTypeIdDefinition::EntityType(ty.into()),
        }
    }
}
