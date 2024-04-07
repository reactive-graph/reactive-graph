use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::InputObject;
use async_graphql::Object;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::query::GraphQLPropertyType;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum GraphQLPropertyTypeContainer {
    #[default]
    None,
    Entity(EntityTypeId),
    Relation(RelationTypeId),
}

#[derive(Serialize, Deserialize, Clone, Debug, InputObject)]
#[graphql(name = "PropertyInstanceDefinition")]
pub struct GraphQLPropertyInstance {
    #[graphql(skip)]
    pub property_type_container: GraphQLPropertyTypeContainer,

    /// The name of the property.
    pub name: String,

    /// The value of the property as JSON representation.
    pub value: Value,
}

/// The named property stores a value/document as JSON representation.
///
/// Each property is represented by it's name (String) and it's value. The value is
/// a representation of a JSON value/document. Therefore the value can be boolean,
/// number, string, array or an object. For more information about the data types
/// please look at https://docs.serde.rs/serde_json/value/enum.Value.html
#[Object(name = "PropertyInstance")]
impl GraphQLPropertyInstance {
    /// The name of the property.
    async fn name(&self) -> String {
        self.name.clone()
    }

    /// The value of the property as JSON representation.
    async fn value(&self) -> Value {
        self.value.clone()
    }

    /// The type of the property.
    #[graphql(name = "type")]
    async fn property_type(&self, context: &Context<'_>) -> Option<GraphQLPropertyType> {
        let property_name = self.name.clone();
        match &self.property_type_container {
            GraphQLPropertyTypeContainer::None => None,
            GraphQLPropertyTypeContainer::Entity(ty) => match context.data::<Arc<dyn EntityTypeManager + Send + Sync>>() {
                Ok(entity_type_manager) => match entity_type_manager.get(ty) {
                    Some(entity_type) => {
                        let property_type = entity_type
                            .properties
                            .iter()
                            .find(|property_type| property_type.name == property_name)
                            .map(|property_type| property_type.value().clone().into());
                        property_type
                    }
                    None => None,
                },
                Err(_) => None,
            },
            GraphQLPropertyTypeContainer::Relation(ty) => match context.data::<Arc<dyn RelationTypeManager + Send + Sync>>() {
                Ok(relation_type_manager) => match relation_type_manager.get(ty) {
                    Some(relation_type) => {
                        let property_type = relation_type
                            .properties
                            .iter()
                            .find(|property_type| property_type.name == property_name)
                            .map(|property_type| property_type.value().clone().into());
                        property_type
                    }
                    None => None,
                },
                Err(_) => None,
            },
        }
    }
}

impl GraphQLPropertyInstance {
    pub fn new_entity_property(ty: EntityTypeId, name: String, value: Value) -> Self {
        GraphQLPropertyInstance {
            property_type_container: GraphQLPropertyTypeContainer::Entity(ty),
            name,
            value,
        }
    }

    // TODO: Change to ty: RelationInstanceTypeId ???
    pub fn new_relation_property(ty: RelationTypeId, name: String, value: Value) -> Self {
        GraphQLPropertyInstance {
            property_type_container: GraphQLPropertyTypeContainer::Relation(ty),
            name,
            value,
        }
    }

    pub fn to_map(properties: Option<Vec<GraphQLPropertyInstance>>) -> HashMap<String, Value> {
        match properties {
            Some(properties) => {
                let mut props = HashMap::new();
                for property in properties {
                    props.insert(property.name.clone(), property.value.clone());
                }
                props
            }
            None => HashMap::new(),
        }
    }

    pub fn to_map_with_defaults(properties: Option<Vec<GraphQLPropertyInstance>>, property_types: PropertyTypes) -> HashMap<String, Value> {
        let mut props = HashMap::new();
        for property_type in property_types.iter() {
            props.insert(property_type.name.clone(), property_type.data_type.default_value());
        }
        if let Some(properties) = properties {
            for property in properties {
                props.insert(property.name.clone(), property.value.clone());
            }
        }
        props
    }

    pub fn to_property_instances_with_defaults(properties: Option<Vec<GraphQLPropertyInstance>>, property_types: PropertyTypes) -> PropertyInstances {
        let property_instances = PropertyInstances::new();
        for property_type in property_types.iter() {
            property_instances.insert(property_type.name.clone(), property_type.data_type.default_value());
        }
        if let Some(properties) = properties {
            for property in properties {
                property_instances.insert(property.name.clone(), property.value.clone());
            }
        }
        property_instances
    }
}
