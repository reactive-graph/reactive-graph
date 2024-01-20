use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use inexor_rgf_graph::Extension;
use inexor_rgf_graph::RelationInstance;
use inexor_rgf_graph::RelationInstanceTypeId;
use inexor_rgf_graph::RelationInstances;

use crate::query::GraphQLExtension;
use crate::query::GraphQLPropertyInstance;

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

    /// The namespace the relation type belongs to.
    pub namespace: String,

    /// The name of the relation type.
    pub type_name: String,

    /// The instance id of the relation instance type.
    pub instance_id: String,

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

    /// Relation instance specific extensions.
    pub extensions: Vec<GraphQLExtension>,
}

impl From<GraphQLRelationInstanceDefinition> for RelationInstance {
    fn from(relation_instance: GraphQLRelationInstanceDefinition) -> Self {
        let ty = RelationInstanceTypeId::new_from_type_unique_for_instance_id(
            relation_instance.namespace,
            relation_instance.type_name,
            relation_instance.instance_id,
        );
        RelationInstance {
            outbound_id: relation_instance.outbound_id,
            ty,
            inbound_id: relation_instance.inbound_id,
            description: relation_instance.description.clone(),
            properties: relation_instance
                .properties
                .iter()
                .map(|property_instance| (property_instance.name.clone(), property_instance.value.clone()))
                .collect(),
            extensions: relation_instance.extensions.iter().map(|e| Extension::from(e.clone())).collect(),
        }
    }
}

#[derive(Default)]
pub struct GraphQLRelationInstanceDefinitions(pub Vec<GraphQLRelationInstanceDefinition>);

impl GraphQLRelationInstanceDefinitions {
    pub fn new(relation_instances: Vec<GraphQLRelationInstanceDefinition>) -> Self {
        Self(relation_instances)
    }
}

impl From<GraphQLRelationInstanceDefinitions> for RelationInstances {
    fn from(relation_instances: GraphQLRelationInstanceDefinitions) -> Self {
        relation_instances.0.into_iter().map(|entity_instance| entity_instance.into()).collect()
    }
}
