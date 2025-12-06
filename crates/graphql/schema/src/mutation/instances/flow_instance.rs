use std::str::FromStr;
use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use uuid::Uuid;

use reactive_graph_graph::CreateFlowInstanceError;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::error::flow::FlowMutationError;
use crate::mutation::GraphQLFlowInstanceDefinition;
use crate::mutation::GraphQLRelationInstanceId;
use crate::query::GraphQLFlowInstance;
use crate::query::GraphQLPropertyInstance;
use crate::validator::NamespacedTypeValidator;

#[derive(Default)]
pub struct MutationFlowInstances;

/// Mutations for flows and their contained instances.
#[Object]
impl MutationFlowInstances {
    /// Creates a new flow and a corresponding wrapper entity instance.
    ///
    /// The given entity type must exist. It provides the properties for the wrapper entity instance
    /// and therefore defines which properties of the flow are the inputs and outputs.
    ///
    /// Optionally, a UUID can be specified.
    ///
    /// Optionally, the initial values of the properties can be specified. Specified properties
    /// which are not provided by the given entity type are lacking of a definition (data type,
    /// socket type).
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        #[graphql(desc = "The unique identifier of the flow instance and the wrapper entity instance")] flow_id: Option<Uuid>,
        #[graphql(desc = "The properties of the flow instance and the wrapper entity instance")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        let entity_ty = EntityTypeId::from_str(&_type)?;
        let entity_type = entity_type_manager
            .get(&entity_ty)
            .ok_or(CreateFlowInstanceError::EntityTypeDoesNotExist(entity_ty.clone()))?;

        if let Some(flow_id) = flow_id {
            if reactive_flow_manager.has(flow_id) {
                return Err(CreateFlowInstanceError::FlowAlreadyExists(flow_id).into());
            }
            if reactive_entity_manager.has(flow_id) {
                return Err(CreateFlowInstanceError::WrapperEntityInstanceAlreadyExists(flow_id).into());
            }
        }

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, entity_type.properties);

        let reactive_entity = match flow_id {
            Some(id) => reactive_entity_manager.create_with_id(&entity_ty, id, properties),
            None => reactive_entity_manager.create_reactive_entity(&entity_ty, properties),
        }?;

        let flow_instance = ReactiveFlow::new(reactive_entity);
        reactive_flow_manager.register_flow_instance(flow_instance.clone());

        Ok(flow_instance.into())
    }

    /// Creates a new flow from the given type.
    ///
    /// The corresponding wrapper entity instance will be created with the type.
    ///
    /// The given entity type must exist. It provides the properties for the wrapper entity instance
    /// and therefore defines which properties of the flow are the inputs and outputs.
    ///
    /// Optionally, an UUID can be specified.
    ///
    /// Optionally, the initial values of the properties can be specified. Specified properties
    /// which are not provided by the given entity type are lacking of a definition (data type,
    /// socket type).
    async fn create_from_type(
        &self,
        context: &Context<'_>,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the flow type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        id: Option<Uuid>,
        #[graphql(desc = "Parametrized construction of a flow instance using variables of a flow type.")] variables: Option<Vec<GraphQLPropertyInstance>>,
        #[graphql(desc = "A list of properties of the wrapper entity instance.")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLFlowInstance> {
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let flow_type_manager = context.data::<Arc<dyn FlowTypeManager + Send + Sync>>()?;
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;

        let flow_ty = FlowTypeId::from_str(&_type)?;

        let Some(flow_type) = flow_type_manager.get(&flow_ty) else {
            return Err(FlowMutationError::MissingFlowType(flow_ty.clone()).into());
        };

        let entity_ty = flow_type.wrapper_type();
        let Some(entity_type) = entity_type_manager.get(&entity_ty) else {
            return Err(FlowMutationError::MissingEntityType(entity_ty).into());
        };

        let variables = GraphQLPropertyInstance::to_property_instances_with_defaults(variables, flow_type.variables);
        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, entity_type.properties);

        let flow_instance = reactive_flow_manager.create_from_type(&flow_ty, id, variables, properties)?;
        Ok(flow_instance.into())
    }

    // /// Manually ticks all entity instances and relation instances of this flow. This means, for
    // /// each property of each entity instance and relation instance the corresponding reactive
    // /// stream will be activated with it's last value.
    // ///
    // /// This leads to a recalculation if the instance is controlled by an behaviour which
    // /// consumes the reactive streams.
    // ///
    // /// In case of entity instances, it furthermore leads to a new value propagation if the output
    // /// property is connected to other properties.
    // async fn commit(&self, context: &Context<'_>, id: Uuid) -> Result<GraphQLFlowInstance> {
    //     let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
    //     let reactive_flow = reactive_flow_manager.get(id);
    //     if reactive_flow.is_none() {
    //         return Err(FlowMutationError::MissingFlow(id).into());
    //     }
    //     let reactive_flow: ReactiveFlow = reactive_flow.unwrap();
    //     reactive_flow_manager.commit(reactive_flow.id);
    //     Ok(reactive_flow.into())
    // }

    /// Creates a new entity instance and adds the entity instance to the given flow by id.
    async fn create_entity(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        #[graphql(
            name = "type",
            desc = "The fully qualified namespace of the entity type.",
            validator(custom = "NamespacedTypeValidator::new()")
        )]
        _type: String,
        entity_id: Option<Uuid>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        let Some(reactive_flow) = reactive_flow_manager.get(flow_id) else {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        };

        let entity_ty = EntityTypeId::from_str(&_type)?;
        let Some(entity_type) = entity_type_manager.get(&entity_ty) else {
            return Err(FlowMutationError::MissingEntityType(entity_ty).into());
        };

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, entity_type.properties);

        let Ok(reactive_entity) = (match entity_id {
            Some(id) => reactive_entity_manager.create_with_id(&entity_ty, id, properties),
            None => reactive_entity_manager.create_reactive_entity(&entity_ty, properties),
        }) else {
            return Err(FlowMutationError::EntityInstanceCreationError().into());
        };
        reactive_flow.add_entity(reactive_entity);
        Ok(reactive_flow.into())
    }

    /// Adds an existing entity instance by id to the given flow by id
    async fn add_entity(&self, context: &Context<'_>, flow_id: Uuid, entity_id: Uuid) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let Some(reactive_flow) = reactive_flow_manager.get(flow_id) else {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        };
        let Some(reactive_entity) = reactive_entity_manager.get(entity_id) else {
            return Err(FlowMutationError::MissingEntityInstance(entity_id).into());
        };
        reactive_flow.add_entity(reactive_entity);
        // No commit necessary _> The reactive entity  is registered in the reactive_entity_manager
        Ok(reactive_flow.into())
    }

    /// Removes an entity instance from flow.
    async fn remove_entity(&self, context: &Context<'_>, flow_id: Uuid, entity_id: Uuid) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        let flow_instance = reactive_flow_manager.get(flow_id);
        if flow_instance.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow_instance = flow_instance.unwrap();

        let entity_instance = reactive_entity_manager.get(entity_id);
        if entity_instance.is_none() {
            return Err(FlowMutationError::MissingEntityInstance(entity_id).into());
        }
        let entity_instance = entity_instance.unwrap();

        if !flow_instance.has_entity_by_id(entity_id) {
            return Err(FlowMutationError::FlowInstanceDoesNotContainEntityInstance(entity_id).into());
        }

        flow_instance.remove_entity(entity_instance.id);
        // The entity is removed from the flow but not yet deleted
        // TODO: How to handle this? It may be that a entity is used in multiple flows?
        // Orphaned instances / Do not delete instances used in other flows?

        Ok(flow_instance.into())
    }

    /// Creates a new relation instance and adds the relation instance to the given flow by id.
    async fn create_relation(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        relation_instance_id: GraphQLRelationInstanceId,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;

        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let relation_ty = relation_instance_id.ty.relation_type_id();

        let relation_type = relation_type_manager
            .get(&relation_ty)
            .ok_or::<FlowMutationError>(FlowMutationError::MissingRelationType(relation_ty))?;

        let flow_instance = reactive_flow_manager
            .get(flow_id)
            .ok_or::<FlowMutationError>(FlowMutationError::MissingFlow(flow_id))?;

        if !flow_instance.has_entity_by_id(relation_instance_id.outbound_id) {
            return Err(FlowMutationError::MissingOutboundEntityInstance(relation_instance_id.outbound_id).into());
        }

        if !flow_instance.has_entity_by_id(relation_instance_id.inbound_id) {
            return Err(FlowMutationError::MissingInboundEntityInstance(relation_instance_id.inbound_id).into());
        }

        // TODO: optionally we could check if the reactive_entity_manager contains the outbound_id and inbound_id

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, relation_type.properties);

        let relation_instance = reactive_relation_manager.create_reactive_relation(&relation_instance_id, properties);

        if relation_instance.is_err() {
            return Err(FlowMutationError::RelationInstanceCreationError().into());
        }

        let relation_instance = relation_instance.unwrap();

        // Add relation to flow
        flow_instance.add_relation(relation_instance);

        Ok(flow_instance.into())
    }

    /// Adds an existing relation instance by relation_instance_id to the given flow by id
    async fn add_relation(&self, context: &Context<'_>, flow_id: Uuid, relation_instance_id: GraphQLRelationInstanceId) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;

        let flow_instance = reactive_flow_manager.get(flow_id);
        if flow_instance.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow_instance = flow_instance.unwrap();

        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let relation_instance = reactive_relation_manager.get(&relation_instance_id);
        if relation_instance.is_none() {
            return Err(FlowMutationError::MissingRelationInstance(relation_instance_id).into());
        }
        let relation_instance = relation_instance.unwrap();

        flow_instance.add_relation(relation_instance);

        Ok(flow_instance.into())
    }

    /// Removes an existing relation instance by relation_instance_id from the given flow by id
    async fn remove_relation(&self, context: &Context<'_>, flow_id: Uuid, relation_instance_id: GraphQLRelationInstanceId) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;

        let flow_instance = reactive_flow_manager.get(flow_id);
        if flow_instance.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow_instance = flow_instance.unwrap();

        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;

        if !flow_instance.has_relation_by_key(&relation_instance_id) {
            return Err(FlowMutationError::FlowInstanceDoesNotContainRelationInstance(relation_instance_id).into());
        }

        flow_instance.remove_relation(&relation_instance_id);
        // The relation is removed from flow, but not yet deleted
        // TODO: How to handle this? It may be that a relation is used in multiple flows?
        // Orphaned instances / Do not delete instances used in other flows?

        Ok(flow_instance.into())
    }

    async fn delete(&self, context: &Context<'_>, #[graphql(desc = "The id of the entity instance")] id: Uuid) -> Result<bool> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        Ok(reactive_flow_manager.delete(id))
    }

    /// Imports the given flow. Creates entity instances and relation instances which are contained
    /// in the given flow.
    async fn import(&self, context: &Context<'_>, flow: GraphQLFlowInstanceDefinition) -> Result<GraphQLFlowInstance> {
        let reactive_flow_manager = context.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
        let flow_instance = FlowInstance::try_from(flow)?;
        let reactive_flow = reactive_flow_manager.create_reactive_flow(flow_instance)?;
        Ok(reactive_flow.into())
    }
}
