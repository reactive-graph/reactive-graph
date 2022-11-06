use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::graphql::query::GraphQLExtension;
use crate::graphql::query::GraphQLPropertyInstance;
use crate::model::EntityInstance;
use crate::model::EntityTypeId;
use crate::model::Extension;

/// Entity instances represents an typed object which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in it's
/// properties.
#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "EntityInstanceDefinition")]
pub struct GraphQLEntityInstanceDefinition {
    /// The namespace the entity type belongs to.
    pub namespace: String,

    /// The name of the entity type.
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

    /// Entity instance specific extensions.
    pub extensions: Vec<GraphQLExtension>,
}

impl From<GraphQLEntityInstanceDefinition> for EntityInstance {
    fn from(entity_instance: GraphQLEntityInstanceDefinition) -> Self {
        EntityInstance {
            ty: EntityTypeId::new_from_type(entity_instance.namespace, entity_instance.type_name),
            id: entity_instance.id,
            description: entity_instance.description.clone(),
            properties: entity_instance
                .properties
                .iter()
                .map(|property_instance| (property_instance.name.clone(), property_instance.value.clone()))
                .collect(),
            extensions: entity_instance.extensions.iter().map(|e| Extension::from(e.clone())).collect(),
        }
    }
}
