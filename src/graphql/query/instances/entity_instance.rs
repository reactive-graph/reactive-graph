use std::ops::Deref;
use std::sync::Arc;

use async_graphql::*;
use uuid::Uuid;

use crate::api::{EntityTypeManager, ReactiveRelationInstanceManager};
use crate::graphql::query::{GraphQLEntityType, GraphQLPropertyInstance, GraphQLRelationInstance};
use crate::model::ReactiveEntityInstance;

pub struct GraphQLEntityInstance {
    entity_instance: Arc<ReactiveEntityInstance>,
}

/// Entity instances represents an typed objects which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in it's
/// properties.
#[Object(name = "EntityInstance")]
impl GraphQLEntityInstance {
    /// The entity type of the entity instance.
    #[graphql(name = "type")]
    async fn entity_type(&self, context: &Context<'_>) -> Option<GraphQLEntityType> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>();
        if entity_type_manager.is_ok() {
            let entity_type_manager = entity_type_manager.unwrap();
            return entity_type_manager
                .get(self.entity_instance.type_name.clone())
                .and_then(|entity_type| {
                    let entity_type: GraphQLEntityType = entity_type.clone().into();
                    Some(entity_type)
                });
        }
        None
    }

    /// The unique identifier of the entity instance.
    async fn id(&self) -> Uuid {
        self.entity_instance.id
    }

    /// The description of the entity instance.
    async fn description(&self) -> String {
        self.entity_instance.description.clone()
    }

    /// The properties of then entity instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    async fn properties(
        &self,
        #[graphql(desc = "Filters by property name")] name: Option<String>,
        #[graphql(desc = "Filters by property names")] names: Option<Vec<String>>,
    ) -> Vec<GraphQLPropertyInstance> {
        self.entity_instance
            .properties
            .iter()
            .filter(|(property_name, _property_instance)| {
                name.is_none() || name.clone().unwrap() == property_name.deref().clone()
            })
            .filter(|(property_name, _property_instance)| {
                names.is_none() || names.clone().unwrap().contains(&property_name)
            })
            .map(|(property_name, property_instance)| {
                let value = property_instance.value.read().unwrap().deref().clone();
                GraphQLPropertyInstance {
                    name: property_name.clone(),
                    value: value.clone(),
                }
            })
            .collect()
    }

    /// List of relation instances which starts at this entity instance.
    async fn outbound(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Unimplemented")] type_name: Option<String>,
    ) -> Vec<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>();
        if relation_instance_manager.is_ok() {
            let relation_instance_manager = relation_instance_manager.unwrap();
            return relation_instance_manager
                .get_by_outbound_entity(self.entity_instance.id)
                .iter()
                .filter(|relation_instance| {
                    type_name.is_none()
                        || type_name.clone().unwrap() == relation_instance.type_name.clone()
                })
                .map(|relation_instance| relation_instance.clone().into())
                .collect();
        }
        Vec::new()
    }

    /// List of relation instances which ends at this entity instance.
    async fn inbound(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Unimplemented")] type_name: Option<String>,
    ) -> Vec<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>();
        if relation_instance_manager.is_ok() {
            let relation_instance_manager = relation_instance_manager.unwrap();
            return relation_instance_manager
                .get_by_inbound_entity(self.entity_instance.id)
                .iter()
                .filter(|relation_instance| {
                    type_name.is_none()
                        || type_name.clone().unwrap() == relation_instance.type_name.clone()
                })
                .map(|relation_instance| relation_instance.clone().into())
                .collect();
        }
        Vec::new()
    }
}

impl From<Arc<ReactiveEntityInstance>> for GraphQLEntityInstance {
    fn from(entity_instance: Arc<ReactiveEntityInstance>) -> Self {
        GraphQLEntityInstance { entity_instance }
    }
}
