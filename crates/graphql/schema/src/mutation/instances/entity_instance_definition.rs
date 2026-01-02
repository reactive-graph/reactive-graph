use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

use crate::mutation::GraphQLExtensionDefinition;
use crate::query::GraphQLPropertyInstance;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityInstances;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Extension;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::NamespacedTypeParseError;
use reactive_graph_graph::PropertyInstances;

/// Entity instances represents a typed object which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in its
/// properties.
#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "EntityInstanceDefinition")]
pub struct GraphQLEntityInstanceDefinition {
    /// The fully qualified name of the entity type.
    /// TODO: REGEX
    #[graphql(name = "type")]
    pub _type: String,

    /// The unique identifier of the entity instance.
    pub id: Uuid,

    /// The description of the entity instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties of then entity instance.
    ///
    /// Each property is represented by its name (String) and it's value. The value is
    /// a representation of a JSON. Therefore, the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    pub properties: Vec<GraphQLPropertyInstance>,

    // TODO: components
    /// Entity instance specific extensions.
    pub extensions: Vec<GraphQLExtensionDefinition>,
}

impl TryFrom<GraphQLEntityInstanceDefinition> for EntityInstance {
    type Error = NamespacedTypeParseError;

    fn try_from(entity_instance: GraphQLEntityInstanceDefinition) -> Result<Self, Self::Error> {
        let entity_ty = EntityTypeId::from_str(&entity_instance._type)?;
        let properties: PropertyInstances = entity_instance
            .properties
            .iter()
            .map(|property_instance| (property_instance.name.clone(), property_instance.value.clone()))
            .collect();
        let extensions = Extensions::new();
        for extension in entity_instance.extensions {
            extensions.push(Extension::try_from(extension.clone())?);
        }
        // let components; relation_instance.components.iter().map(|e| ComponentTypeId::from(e.clone())).collect();
        Ok(EntityInstance::builder()
            .ty(entity_ty)
            .id(entity_instance.id)
            .description(&entity_instance.description)
            .properties(properties)
            // .components(components) ???
            .extensions(extensions)
            .build())
    }
}

#[derive(Default)]
pub struct GraphQLEntityInstanceDefinitions(pub Vec<GraphQLEntityInstanceDefinition>);

impl GraphQLEntityInstanceDefinitions {
    pub fn new(entity_instances: Vec<GraphQLEntityInstanceDefinition>) -> Self {
        Self(entity_instances)
    }
}

impl TryFrom<GraphQLEntityInstanceDefinitions> for EntityInstances {
    type Error = NamespacedTypeParseError;

    fn try_from(entity_instance_definitions: GraphQLEntityInstanceDefinitions) -> Result<Self, Self::Error> {
        let entity_instances = EntityInstances::new();
        for entity_instance_definition in entity_instance_definitions.0 {
            entity_instances.push(EntityInstance::try_from(entity_instance_definition)?);
        }
        Ok(entity_instances)
    }
}
