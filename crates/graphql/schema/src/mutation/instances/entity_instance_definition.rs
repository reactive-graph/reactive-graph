use async_graphql::*;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityInstances;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::PropertyInstances;

use crate::query::GraphQLExtension;
use crate::query::GraphQLPropertyInstance;

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
        let properties: PropertyInstances = entity_instance
            .properties
            .iter()
            .map(|property_instance| (property_instance.name.clone(), property_instance.value.clone()))
            .collect();
        let extensions: Extensions = entity_instance.extensions.iter().map(|e| Extension::from(e.clone())).collect();
        EntityInstance::builder()
            .ty(EntityTypeId::new_from_type(entity_instance.namespace, entity_instance.type_name))
            .id(entity_instance.id)
            .description(&entity_instance.description)
            .properties(properties)
            .extensions(extensions)
            .build()
    }
}

#[derive(Default)]
pub struct GraphQLEntityInstanceDefinitions(pub Vec<GraphQLEntityInstanceDefinition>);

impl GraphQLEntityInstanceDefinitions {
    pub fn new(entity_instances: Vec<GraphQLEntityInstanceDefinition>) -> Self {
        Self(entity_instances)
    }
}

impl From<GraphQLEntityInstanceDefinitions> for EntityInstances {
    fn from(entity_instances: GraphQLEntityInstanceDefinitions) -> Self {
        entity_instances.0.into_iter().map(|entity_instance| entity_instance.into()).collect()
    }
}
