use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use uuid::Uuid;

use reactive_graph_behaviour_service_api::EntityBehaviourRegistry;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourRegistry;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;

use crate::query::GraphQLComponent;
use crate::query::GraphQLComponentBehaviour;
use crate::query::GraphQLEntityBehaviour;
use crate::query::GraphQLEntityType;
use crate::query::GraphQLPropertyInstance;
use crate::query::GraphQLRelationInstance;

pub struct GraphQLEntityInstance {
    entity_instance: ReactiveEntity,
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
        if let Ok(entity_type_manager) = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>() {
            return entity_type_manager.get(&self.entity_instance.ty).map(|entity_type| entity_type.into());
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

    /// The name of the entity instance.
    async fn name(&self) -> String {
        self.entity_instance.name.clone()
    }

    /// Textual description of the entity instance.
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
        #[graphql(desc = "If true, the properties are sorted by name")] sort: Option<bool>,
    ) -> Vec<GraphQLPropertyInstance> {
        let mut properties: Vec<GraphQLPropertyInstance> = self
            .entity_instance
            .properties
            .iter()
            .filter(|property_instance| name.is_none() || name.clone().unwrap().as_str() == property_instance.key().as_str())
            .filter(|property_instance| names.is_none() || names.clone().unwrap().contains(property_instance.key()))
            .map(|property_instance| {
                GraphQLPropertyInstance::new_entity_property(self.entity_instance.ty.clone(), property_instance.key().clone(), property_instance.get())
            })
            .collect();
        if sort.unwrap_or_default() {
            properties.sort_by(|a, b| a.name.cmp(&b.name));
        }
        properties
    }

    /// The components which have been actually applied on the entity instance including
    /// components which have been added after creation.
    async fn components(&self, context: &Context<'_>) -> Vec<GraphQLComponent> {
        match context.data::<Arc<dyn ComponentManager + Send + Sync>>() {
            Ok(component_manager) => self
                .entity_instance
                .components
                .iter()
                .map(|p| p.key().clone())
                .filter_map(|component_ty| {
                    component_manager.get(&component_ty).map(|component| {
                        let component: GraphQLComponent = component.into();
                        component
                    })
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// List of entity behaviours which have been actually applied on the entity instance
    /// including behaviours which have been applied after creation.
    async fn behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLEntityBehaviour>> {
        let entity_behaviour_registry = context.data::<Arc<dyn EntityBehaviourRegistry + Send + Sync>>()?;
        Ok(self
            .entity_instance
            .behaviours
            .iter()
            .filter_map(move |p| {
                let behaviour_ty = p.key();
                entity_behaviour_registry.get_by_behaviour_type(behaviour_ty).map(GraphQLEntityBehaviour::from)
            })
            .collect())
    }

    /// List of component behaviours which have been actually applied on the entity instance
    /// including behaviours which have been applied after creation.
    async fn component_behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponentBehaviour>> {
        let entity_component_behaviour_registry = context.data::<Arc<dyn EntityComponentBehaviourRegistry + Send + Sync>>()?;
        Ok(self
            .entity_instance
            .behaviours
            .iter()
            .filter_map(move |p| {
                let behaviour_ty = p.key();
                entity_component_behaviour_registry
                    .get_by_behaviour_type(behaviour_ty)
                    .map(GraphQLComponentBehaviour::from)
            })
            .collect())
    }

    /// List of relation instances which starts at this entity instance.
    async fn outbound(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The outbound relation type")] outbound_namespace: Option<String>,
    ) -> Result<Vec<GraphQLRelationInstance>> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let outbound_ty = if let Some(outbound_namespace) = outbound_namespace {
            Some(RelationTypeId::parse_namespace(&outbound_namespace)?)
        } else {
            None
        };
        let relation_instances = relation_instance_manager
            .get_by_outbound_entity(self.entity_instance.id)
            .iter()
            .filter(|relation_instance| match &outbound_ty {
                Some(outbound_ty) => outbound_ty == &relation_instance.relation_type_id(),
                None => true,
            })
            .map(|relation_instance| relation_instance.clone().into())
            .collect();
        Ok(relation_instances)
    }

    /// List of relation instances which ends at this entity instance.
    async fn inbound(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The inbound relation type")] inbound_namespace: Option<String>,
    ) -> Result<Vec<GraphQLRelationInstance>> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let inbound_ty = if let Some(inbound_namespace) = inbound_namespace {
            Some(RelationTypeId::parse_namespace(&inbound_namespace)?)
        } else {
            None
        };
        let relation_instances = relation_instance_manager
            .get_by_inbound_entity(self.entity_instance.id)
            .iter()
            .filter(|relation_instance| match &inbound_ty {
                Some(inbound_ty) => inbound_ty == &relation_instance.relation_type_id(),
                None => true,
            })
            .map(|relation_instance| relation_instance.clone().into())
            .collect();
        Ok(relation_instances)
    }
}

impl From<ReactiveEntity> for GraphQLEntityInstance {
    fn from(entity_instance: ReactiveEntity) -> Self {
        GraphQLEntityInstance { entity_instance }
    }
}
