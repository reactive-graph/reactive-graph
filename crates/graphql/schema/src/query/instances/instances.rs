use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use uuid::Uuid;

use inexor_rgf_behaviour_model_api::BehaviourTypeId;
use inexor_rgf_behaviour_model_api::BehaviourTypesContainer;
use inexor_rgf_graph::ComponentContainer;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_reactive_service_api::ReactiveEntityManager;
use inexor_rgf_reactive_service_api::ReactiveFlowManager;
use inexor_rgf_reactive_service_api::ReactiveRelationManager;

use crate::mutation::BehaviourTypeIdDefinition;
use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::EntityTypeIdDefinition;
use crate::mutation::RelationTypeIdDefinition;
use crate::query::GraphQLEntityInstance;
use crate::query::GraphQLFlowInstance;
use crate::query::GraphQLPropertyInstance;
use crate::query::GraphQLRelationInstance;

#[derive(Default)]
pub struct Instances;

/// Search for instances
#[Object]
impl Instances {
    /// Search for entity instances.
    ///
    /// If an id is given, the entity instance with the given id will be returned.
    ///
    /// If an entity type is given, only entity instances of the given type are returned.
    async fn entities(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Returns only the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Returns the entity instance with the given label.")] label: Option<String>,
        #[graphql(name = "type", desc = "Filters the entity instances by type.")] entity_type: Option<EntityTypeIdDefinition>,
        #[graphql(desc = "Filters the entity instances by applied components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Filters the entity instances by applied behaviours.")] behaviours: Option<Vec<BehaviourTypeIdDefinition>>,
        #[graphql(name = "properties", desc = "Query by properties.")] property_query: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Vec<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>();
        if entity_instance_manager.is_ok() {
            let entity_instance_manager = entity_instance_manager.unwrap();
            if id.is_some() {
                let entity_instance = entity_instance_manager.get(id.unwrap()).map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.into();
                    entity_instance
                });
                return if entity_instance.is_some() {
                    vec![entity_instance.unwrap()]
                } else {
                    Vec::new()
                };
            }
            if label.is_some() {
                let entity_instance = entity_instance_manager.get_by_label(label.unwrap().as_str()).map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.into();
                    entity_instance
                });
                return if entity_instance.is_some() {
                    vec![entity_instance.unwrap()]
                } else {
                    Vec::new()
                };
            }
            return entity_instance_manager
                .get_all()
                .iter()
                .filter(|entity_instance| entity_type.is_none() || entity_instance.ty == EntityTypeId::from(entity_type.clone().unwrap()))
                .filter(|entity_instance| {
                    components.is_none() || {
                        let components = components.clone().unwrap();
                        components.iter().cloned().all(|component_ty| entity_instance.is_a(&component_ty.into()))
                    }
                })
                .filter(|entity_instance| {
                    behaviours.is_none() || {
                        let behaviours = behaviours.clone().unwrap();
                        behaviours.iter().cloned().all(|behaviour_ty| entity_instance.behaves_as(&behaviour_ty.into()))
                    }
                })
                .filter(|entity_instance| {
                    property_query.is_none() || {
                        let property_query = property_query.clone().unwrap();
                        if property_query.is_empty() {
                            return true;
                        }
                        if entity_instance.properties.is_empty() {
                            return false;
                        }
                        property_query
                            .iter()
                            .all(|property_query| match entity_instance.properties.get(property_query.name.as_str()) {
                                Some(property_instance) => property_query.value == property_instance.get(),
                                None => false,
                            })
                    }
                })
                .map(|entity_instance| {
                    let entity_instance: GraphQLEntityInstance = entity_instance.clone().into();
                    entity_instance
                })
                .collect();
        }
        Vec::new()
    }

    async fn count_entity_instances(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "Counts the entity instances of the given type only.")] ty: Option<EntityTypeIdDefinition>,
        #[graphql(name = "component", desc = "Counts the entity instances which are composed by the given component only.")] component_ty: Option<
            ComponentTypeIdDefinition,
        >,
        #[graphql(name = "behaviour", desc = "Counts the entity instances which behaves as the behaviour only.")] behaviour_ty: Option<
            BehaviourTypeIdDefinition,
        >,
    ) -> usize {
        if let Ok(entity_instance_manager) = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>() {
            let ty: Option<EntityTypeId> = ty.map(|ty| ty.into());
            let component_ty: Option<ComponentTypeId> = component_ty.map(|component_ty| component_ty.into());
            let behaviour_ty: Option<BehaviourTypeId> = behaviour_ty.map(|behaviour_ty| behaviour_ty.into());
            if ty.is_none() && component_ty.is_none() && behaviour_ty.is_none() {
                return entity_instance_manager.count();
            }
            if component_ty.is_none() && behaviour_ty.is_none() {
                return entity_instance_manager.count_by_type(&ty.unwrap());
            }
            if ty.is_none() && behaviour_ty.is_none() {
                return entity_instance_manager.count_by_component(&component_ty.unwrap());
            }
            if ty.is_none() && component_ty.is_none() {
                return entity_instance_manager.count_by_behaviour(&behaviour_ty.unwrap());
            }
            return entity_instance_manager
                .get_all()
                .iter()
                .filter(|e| ty.is_none() || { e.ty == ty.clone().unwrap() })
                .filter(|e| component_ty.is_none() || e.is_a(&component_ty.clone().unwrap()))
                .filter(|e| behaviour_ty.is_none() || e.behaves_as(&behaviour_ty.clone().unwrap()))
                .count();
        }
        0
    }

    /// Search for relations instances.
    ///
    /// Relation instances can be searched by relation type name, the entity type of the outbound
    /// entity instance, the entity type of the inbound entity instance, the id of the outbound
    /// entity instance or the id of the inbound entity instance. All of these filters can be
    /// combined.
    async fn relations(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters the relation instances by the entity type of the outbound entity instance.")] outbound_entity_ty: Option<
            EntityTypeIdDefinition,
        >,
        #[graphql(desc = "Filters the relation instances by the component of the outbound entity instance.")] outbound_component_ty: Option<
            ComponentTypeIdDefinition,
        >,
        #[graphql(desc = "Filters the relation instances by the id of the outbound entity instance")] outbound_id: Option<Uuid>,
        #[graphql(name = "type", desc = "Filters the relation instances by relation type")] relation_ty: Option<RelationTypeIdDefinition>, // TODO: RelationInstanceTypeIdDefinition ?
        #[graphql(desc = "Filters the relation instances by the entity type of the inbound entity instance.")] inbound_entity_ty: Option<
            EntityTypeIdDefinition,
        >,
        #[graphql(desc = "Filters the relation instances by the component of the inbound entity instance.")] inbound_component_ty: Option<
            ComponentTypeIdDefinition,
        >,
        #[graphql(desc = "Filters the relation instances by the id of the inbound entity instance")] inbound_id: Option<Uuid>,
        #[graphql(desc = "Filters the relation instances by applied components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Filters the relation instances by applied behaviours.")] behaviours: Option<Vec<BehaviourTypeIdDefinition>>,
        #[graphql(name = "properties", desc = "Query by properties.")] property_query: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<Vec<GraphQLRelationInstance>> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let outbound_entity_ty: Option<EntityTypeId> = outbound_entity_ty.map(|outbound_entity_ty| outbound_entity_ty.into());
        let outbound_component_ty: Option<ComponentTypeId> = outbound_component_ty.map(|outbound_component_ty| outbound_component_ty.into());
        let relation_ty: Option<RelationTypeId> = relation_ty.map(|relation_ty| relation_ty.into());
        let inbound_entity_ty: Option<EntityTypeId> = inbound_entity_ty.map(|inbound_entity_ty| inbound_entity_ty.into());
        let inbound_component_ty: Option<ComponentTypeId> = inbound_component_ty.map(|inbound_component_ty| inbound_component_ty.into());
        let components: Option<Vec<ComponentTypeId>> =
            components.map(|components| components.iter().cloned().map(|component_ty| component_ty.into()).collect());
        let behaviours: Option<Vec<BehaviourTypeId>> =
            behaviours.map(|behaviours| behaviours.iter().cloned().map(|behaviour_ty| behaviour_ty.into()).collect());
        let relation_instances = relation_instance_manager
            .get_all()
            .iter()
            .filter(|relation_instance| outbound_entity_ty.is_none() || relation_instance.outbound.ty == outbound_entity_ty.clone().unwrap())
            .filter(|relation_instance| outbound_component_ty.is_none() || relation_instance.outbound.is_a(&outbound_component_ty.clone().unwrap()))
            .filter(|relation_instance| relation_ty.is_none() || relation_instance.relation_type_id() == relation_ty.clone().unwrap())
            .filter(|relation_instance| inbound_entity_ty.is_none() || relation_instance.inbound.ty == inbound_entity_ty.clone().unwrap())
            .filter(|relation_instance| inbound_component_ty.is_none() || relation_instance.inbound.is_a(&inbound_component_ty.clone().unwrap()))
            .filter(|relation_instance| outbound_id.is_none() || outbound_id.unwrap() == relation_instance.outbound.id)
            .filter(|relation_instance| inbound_id.is_none() || inbound_id.unwrap() == relation_instance.inbound.id)
            .filter(|relation_instance| {
                components.is_none() || {
                    let components = components.clone().unwrap();
                    components.iter().all(|component_ty| relation_instance.is_a(component_ty))
                }
            })
            .filter(|relation_instance| {
                behaviours.is_none() || {
                    let behaviours = behaviours.clone().unwrap();
                    behaviours.iter().all(|behaviour_ty| relation_instance.behaves_as(behaviour_ty))
                }
            })
            .filter(|relation_instance| {
                property_query.is_none() || {
                    let property_query = property_query.clone().unwrap();
                    if property_query.is_empty() {
                        return true;
                    }
                    if relation_instance.properties.is_empty() {
                        return false;
                    }
                    property_query
                        .iter()
                        .all(|property_query| match relation_instance.properties.get(property_query.name.as_str()) {
                            Some(property_instance) => property_query.value == property_instance.get(),
                            None => false,
                        })
                }
            })
            .map(|relation_instance| {
                let relation_instance: GraphQLRelationInstance = relation_instance.clone().into();
                relation_instance
            })
            .collect();
        Ok(relation_instances)
    }

    async fn count_relation_instances(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "Counts the entity instances of the given type only.")] ty: Option<RelationTypeIdDefinition>,
    ) -> usize {
        context
            .data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()
            .map(|relation_instance_manager| match ty {
                Some(ty) => relation_instance_manager.count_by_type(&ty.into()),
                None => relation_instance_manager.count(),
            })
            .unwrap_or(0)
    }

    /// Search for flows and their contained instances.
    async fn flows(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Filters by the id of the flow")] id: Option<Uuid>,
        #[graphql(desc = "Filters by the label of the flow")] label: Option<String>,
        #[graphql(name = "type", desc = "Filters by the flow type")] entity_ty: Option<EntityTypeIdDefinition>,
    ) -> Result<Vec<GraphQLFlowInstance>> {
        let flow_instance_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        if id.is_some() {
            return match flow_instance_manager.get(id.unwrap()).map(|flow| flow.into()) {
                Some(flow) => Ok(vec![flow]),
                None => Ok(Vec::new()),
            };
        }
        if label.is_some() {
            let flow = flow_instance_manager.get_by_label(label.unwrap().as_str()).map(|flow| {
                let flow: GraphQLFlowInstance = flow.into();
                flow
            });
            return if flow.is_some() { Ok(vec![flow.unwrap()]) } else { Ok(Vec::new()) };
        }
        let flow_instances = flow_instance_manager
            .get_all()
            .iter()
            .filter(|flow_instance| match &entity_ty {
                Some(entity_ty) => flow_instance.ty == entity_ty.clone().into(),
                None => true,
            })
            .map(|flow| {
                let flow: GraphQLFlowInstance = flow.clone().into();
                flow
            })
            .collect();
        Ok(flow_instances)
    }

    async fn count_flow_instances(&self, context: &Context<'_>) -> usize {
        context
            .data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()
            .map(|flow_instance_manager| flow_instance_manager.count_flow_instances())
            .unwrap_or(0)
    }
}
