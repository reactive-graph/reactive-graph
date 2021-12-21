use async_graphql::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::query::GraphQLPropertyInstance;
use crate::model::EntityInstance;

/// Entity instances represents an typed object which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in it's
/// properties.
#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
pub struct GraphQLEntityInstanceDefinition {
    /// The name of the entity type.
    #[graphql(name = "type")]
    pub type_name: String,

    /// The unique identifier of the entity instance.
    pub id: Uuid,

    /// The description of the entity instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties of then entity instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    pub properties: Vec<GraphQLPropertyInstance>,
}

impl From<GraphQLEntityInstanceDefinition> for EntityInstance {
    fn from(entity_instance: GraphQLEntityInstanceDefinition) -> Self {
        EntityInstance {
            type_name: entity_instance.type_name.clone(),
            id: entity_instance.id,
            description: entity_instance.description.clone(),
            properties: entity_instance
                .properties
                .iter()
                .map(|property_instance| (property_instance.name.clone(), property_instance.value.clone()))
                .collect(),
        }
    }
}
