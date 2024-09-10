use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;

use reactive_graph_behaviour_service_api::RelationBehaviourRegistry;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourRegistry;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::query::GraphQLComponent;
use crate::query::GraphQLComponentBehaviour;
use crate::query::GraphQLEntityInstance;
use crate::query::GraphQLPropertyInstance;
use crate::query::GraphQLRelationBehaviour;
use crate::query::GraphQLRelationType;

pub struct GraphQLRelationInstance {
    relation_instance: ReactiveRelation,
}

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In contrast to the relation type, the relation instance stores values/
/// documents in it's properties.
#[Object(name = "RelationInstance")]
impl GraphQLRelationInstance {
    /// The outbound entity instance.
    ///
    /// You can use this in order to navigate from the outbound entity instance to the inbound
    /// entity instance or vice versa.
    async fn outbound(&self) -> GraphQLEntityInstance {
        self.relation_instance.outbound.clone().into()
    }

    /// The relation type.
    #[graphql(name = "type")]
    async fn relation_type(&self, context: &Context<'_>) -> Option<GraphQLRelationType> {
        context
            .data::<Arc<dyn RelationTypeManager + Send + Sync>>()
            .ok()?
            .get(&self.relation_instance.relation_type_id())
            .map(|r| r.into())
    }

    /// The instance id of the relation instance type.
    async fn instance_id(&self) -> String {
        self.relation_instance.ty.instance_id()
    }

    /// The inbound entity instance.
    ///
    /// You can use this in order to navigate from the inbound entity instance to the outbound
    /// entity instance or vice versa.
    async fn inbound(&self) -> GraphQLEntityInstance {
        self.relation_instance.inbound.clone().into()
    }

    /// Textual description of the relation instance.
    async fn description(&self) -> String {
        self.relation_instance.description.clone()
    }

    /// The properties of then relation instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    async fn properties(
        &self,
        #[graphql(desc = "Filters by property name.")] name: Option<String>,
        #[graphql(desc = "Filters by property names")] names: Option<Vec<String>>,
        #[graphql(desc = "If true, the properties are sorted by name")] sort: Option<bool>,
    ) -> Vec<GraphQLPropertyInstance> {
        let mut properties: Vec<GraphQLPropertyInstance> = self
            .relation_instance
            .properties
            .iter()
            .filter(|property_instance| name.is_none() || name.clone().unwrap() == property_instance.key().as_str())
            .filter(|property_instance| names.is_none() || names.clone().unwrap().contains(property_instance.key()))
            .map(|property_instance| {
                GraphQLPropertyInstance::new_relation_property(
                    self.relation_instance.relation_type_id(),
                    property_instance.key().clone(),
                    property_instance.get(),
                )
            })
            .collect();
        if sort.unwrap_or_default() {
            properties.sort_by(|a, b| a.name.cmp(&b.name));
        }
        properties
    }

    /// The components which have been actually applied on the relation instance including
    /// components which have been added after creation.
    async fn components(&self, context: &Context<'_>) -> Vec<GraphQLComponent> {
        match context.data::<Arc<dyn ComponentManager + Send + Sync>>() {
            Ok(component_manager) => self
                .relation_instance
                .components
                .iter()
                .map(|p| p.key().clone())
                .filter_map(|component_name| {
                    component_manager.get(&component_name).map(|component| {
                        let component: GraphQLComponent = component.into();
                        component
                    })
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    /// List of relation behaviours which have been actually applied on the relation instance
    /// including behaviours which have been applied after creation.
    async fn behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLRelationBehaviour>> {
        let relation_behaviour_registry = context.data::<Arc<dyn RelationBehaviourRegistry + Send + Sync>>()?;
        Ok(self
            .relation_instance
            .behaviours
            .iter()
            .filter_map(move |p| {
                let behaviour_ty = p.key();
                relation_behaviour_registry
                    .get_by_behaviour_type(behaviour_ty)
                    .map(GraphQLRelationBehaviour::from)
            })
            .collect())
    }

    /// List of component behaviours which have been actually applied on the entity instance
    /// including behaviours which have been applied after creation.
    async fn component_behaviours(&self, context: &Context<'_>) -> Result<Vec<GraphQLComponentBehaviour>> {
        let relation_component_behaviour_registry = context.data::<Arc<dyn RelationComponentBehaviourRegistry + Send + Sync>>()?;
        Ok(self
            .relation_instance
            .behaviours
            .iter()
            .filter_map(move |p| {
                let behaviour_ty = p.key();
                relation_component_behaviour_registry
                    .get_by_behaviour_type(behaviour_ty)
                    .map(GraphQLComponentBehaviour::from)
            })
            .collect())
    }
}

impl From<ReactiveRelation> for GraphQLRelationInstance {
    fn from(relation_instance: ReactiveRelation) -> Self {
        GraphQLRelationInstance { relation_instance }
    }
}
