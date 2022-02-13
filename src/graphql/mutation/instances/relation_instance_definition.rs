use async_graphql::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::graphql::query::GraphQLPropertyInstance;
use crate::model::RelationInstance;

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In constrast to the relation type, the relation instance stores values/
/// documents in it's properties.
#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "RelationInstanceDefinition")]
pub struct GraphQLRelationInstanceDefinition {
    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The name of the relation type
    #[graphql(name = "type")]
    pub type_name: String,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,

    /// Textual description of the relation instance.
    pub description: String,

    /// The properties of then relation instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    pub properties: Vec<GraphQLPropertyInstance>,
}

impl From<GraphQLRelationInstanceDefinition> for RelationInstance {
    fn from(relation_instance: GraphQLRelationInstanceDefinition) -> Self {
        RelationInstance {
            outbound_id: relation_instance.outbound_id,
            type_name: relation_instance.type_name.clone(),
            inbound_id: relation_instance.inbound_id,
            description: relation_instance.description.clone(),
            properties: relation_instance
                .properties
                .iter()
                .map(|property_instance| (property_instance.name.clone(), property_instance.value.clone()))
                .collect(),
        }
    }
}
