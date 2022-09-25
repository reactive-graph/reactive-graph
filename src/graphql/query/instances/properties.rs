use crate::api::{EntityTypeManager, RelationTypeManager};
use crate::graphql::query::GraphQLPropertyType;
use async_graphql::{Context, InputObject, Object};
use inexor_rgf_core_model::PropertyType;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum GraphQLPropertyTypeContainer {
    #[default]
    None,
    Entity(String),
    Relation(String),
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
            GraphQLPropertyTypeContainer::Entity(type_name) => match context.data::<Arc<dyn EntityTypeManager>>() {
                Ok(entity_type_manager) => match entity_type_manager.get(&type_name) {
                    Some(entity_type) => {
                        let property_type = entity_type
                            .properties
                            .iter()
                            .find(|property_type| property_type.name == property_name)
                            .cloned()
                            .map(|property_type| property_type.into());
                        property_type
                    }
                    None => None,
                },
                Err(_) => None,
            },
            GraphQLPropertyTypeContainer::Relation(type_name) => match context.data::<Arc<dyn RelationTypeManager>>() {
                Ok(relation_type_manager) => match relation_type_manager.get_starts_with(&type_name) {
                    Some(relation_type) => {
                        let property_type = relation_type
                            .properties
                            .iter()
                            .find(|property_type| property_type.name == property_name)
                            .cloned()
                            .map(|property_type| property_type.into());
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
    pub fn new_entity_property(type_name: String, name: String, value: Value) -> Self {
        GraphQLPropertyInstance {
            property_type_container: GraphQLPropertyTypeContainer::Entity(type_name),
            name,
            value,
        }
    }

    pub fn new_relation_property(type_name: String, name: String, value: Value) -> Self {
        GraphQLPropertyInstance {
            property_type_container: GraphQLPropertyTypeContainer::Relation(type_name),
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

    pub fn to_map_with_defaults(properties: Option<Vec<GraphQLPropertyInstance>>, property_types: Vec<PropertyType>) -> HashMap<String, Value> {
        let mut props = HashMap::new();
        for property_type in property_types {
            props.insert(property_type.name.clone(), property_type.data_type.default_value());
        }
        if let Some(properties) = properties {
            for property in properties {
                props.insert(property.name.clone(), property.value.clone());
            }
        }
        props
    }
}
