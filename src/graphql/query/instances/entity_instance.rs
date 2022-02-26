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
                .map(|entity_type| entity_type.into());
        }
        None
    }

    /// The unique identifier of the entity instance.
    async fn id(&self) -> Uuid {
        self.entity_instance.id
    }

    /// The label of the entity instance if available.
    async fn label(&self) -> Option<String> {
        self.entity_instance
            .properties
            .get("label")
            .and_then(|property_instance| property_instance.as_string())
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
            .filter(|property_instance| name.is_none() || name.clone().unwrap().as_str() == property_instance.key().as_str())
            .filter(|property_instance| names.is_none() || names.clone().unwrap().contains(property_instance.key()))
            .map(|property_instance| {
                GraphQLPropertyInstance::new_entity_property(self.entity_instance.type_name.clone(), property_instance.key().clone(), property_instance.get())
            })
            .collect()
    }

    /// List of components which have been actually applied on the entity instance including
    /// components which have been added after creation.
    async fn components(&self) -> Vec<String> {
        self.entity_instance.components.iter().map(|p| p.key().clone()).collect()
    }

    /// List of behaviours which have been actually applied on the entity instance including
    /// behaviours which have been applied after creation.
    async fn behaviours(&self) -> Vec<String> {
        self.entity_instance.behaviours.iter().map(|p| p.key().clone()).collect()
    }

    /// List of relation instances which starts at this entity instance.
    async fn outbound(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The outbound relation type")] type_name: Option<String>,
    ) -> Vec<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>();
        if relation_instance_manager.is_ok() {
            let relation_instance_manager = relation_instance_manager.unwrap();
            return relation_instance_manager
                .get_by_outbound_entity(self.entity_instance.id)
                .iter()
                .filter(|relation_instance| type_name.is_none() || type_name.clone().unwrap() == relation_instance.type_name.clone())
                .map(|relation_instance| relation_instance.clone().into())
                .collect();
        }
        Vec::new()
    }

    /// List of relation instances which ends at this entity instance.
    async fn inbound(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The inbound relation type")] type_name: Option<String>,
    ) -> Vec<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>();
        if relation_instance_manager.is_ok() {
            let relation_instance_manager = relation_instance_manager.unwrap();
            return relation_instance_manager
                .get_by_inbound_entity(self.entity_instance.id)
                .iter()
                .filter(|relation_instance| type_name.is_none() || type_name.clone().unwrap() == relation_instance.type_name.clone())
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
