use std::fmt;
use std::sync::Arc;

use async_graphql::*;
use indradb::EdgeKey;
use log::debug;
use uuid::Uuid;

use crate::api::{
    EntityTypeManager, ReactiveEntityInstanceManager, ReactiveFlowManager,
    ReactiveRelationInstanceManager, RelationTypeManager,
};
use crate::builder::{ReactiveEntityInstanceBuilder, ReactiveRelationInstanceBuilder};
use crate::graphql::mutation::{GraphQLEdgeKey, GraphQLFlowDefinition};
use crate::graphql::query::{GraphQLFlow, GraphQLPropertyInstance};
use crate::model::ReactiveFlow;

#[derive(Debug)]
pub enum FlowMutationError {
    MissingFlow(Uuid),
    FlowAlreadyExists(Uuid),
    // MissingWrapperEntityInstance(Uuid),
    WrapperEntityInstanceAlreadyExists(Uuid),
    MissingEntityType(String),
    MissingRelationType(String),
    MissingEntityInstance(Uuid),
    MissingRelationInstance(EdgeKey),
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
    FlowDoesNotContainEntityInstance(Uuid),
    FlowDoesNotContainRelationInstance(EdgeKey),
}

impl fmt::Display for FlowMutationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.clone() {
            FlowMutationError::MissingFlow(id) => write!(f, "The flow {} does not exist!", id),
            FlowMutationError::FlowAlreadyExists(id) => {
                write!(f, "Can't create flow: The flow {} already exist!", id)
            }
            // FlowMutationError::MissingWrapperEntityInstance(id) => write!(f, "Missing wrapper entity instance with the id {}", id),
            FlowMutationError::WrapperEntityInstanceAlreadyExists(id) => write!(
                f,
                "Can't create flow: An entity instance with the id {} already exists!",
                id
            ),
            FlowMutationError::MissingEntityType(type_name) => {
                write!(f, "Entity type {} does not exist", type_name.clone())
            }
            FlowMutationError::MissingRelationType(type_name) => {
                write!(f, "Relation type {} does not exist", type_name.clone())
            }
            FlowMutationError::MissingEntityInstance(id) => {
                write!(f, "Entity instance {} does not exist", id)
            }
            FlowMutationError::MissingRelationInstance(edge_key) => {
                write!(f, "Relation instance {:?} does not exist", edge_key)
            }
            FlowMutationError::MissingOutboundEntityInstance(id) => {
                write!(f, "Outbound entity instance {} does not exist", id)
            }
            FlowMutationError::MissingInboundEntityInstance(id) => {
                write!(f, "Inbound entity instance {} does not exist", id)
            }
            FlowMutationError::FlowDoesNotContainEntityInstance(id) => {
                write!(f, "Flow doesn't contain entity instance {}", id)
            }
            FlowMutationError::FlowDoesNotContainRelationInstance(edge_key) => write!(
                f,
                "Flow doesn't contain relation instance {:?}",
                edge_key.clone()
            ),
        }
    }
}

#[derive(Default)]
pub struct MutationFlows;

/// Mutations for flows and their contained instances.
#[Object]
impl MutationFlows {
    /// Creates a new flow and a corresponding wrapper entity instance.
    ///
    /// The given entity type must exist. It provides the properties for the wrapper entity instance
    /// and therefore defines which properties of the flow are the inputs and outputs.
    ///
    /// Optionally, an UUID can be specified.
    ///
    /// Optionally, the initial values of the properties can be specified. Specified properties
    /// which are not provided by the given entity type are lacking of a definition (data type,
    /// socket type).
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type")] type_name: String,
        flow_id: Option<Uuid>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        let entity_type = entity_type_manager.get(type_name.clone());
        if entity_type.is_none() {
            return Err(FlowMutationError::MissingEntityType(type_name.clone()).into());
        }
        let entity_type = entity_type.unwrap();

        let mut entity_instance_builder = ReactiveEntityInstanceBuilder::from(entity_type.clone());
        // let mut entity_instance_builder = ReactiveEntityInstanceBuilder::new(type_name.clone());
        if flow_id.is_some() {
            let flow_id = flow_id.unwrap();
            if flow_manager.has(flow_id) {
                return Err(FlowMutationError::FlowAlreadyExists(flow_id).into());
            }
            if entity_instance_manager.has(flow_id) {
                return Err(FlowMutationError::WrapperEntityInstanceAlreadyExists(flow_id).into());
            }
            entity_instance_builder.id(flow_id);
        }

        if properties.is_some() {
            for property in properties.unwrap() {
                debug!(
                    "set property {} = {}",
                    property.name.clone(),
                    property.value.clone().to_string()
                );
                entity_instance_builder.property(property.name.clone(), property.value.clone());
            }
        }

        let wrapper_entity_instance =
            entity_instance_builder.create(entity_instance_manager.clone())?;

        let flow: Arc<ReactiveFlow> = Arc::new(wrapper_entity_instance.into());
        flow_manager.register_flow(flow.clone());

        Ok(flow.clone().into())
    }

    /// Manually ticks all entity instances and relation instances of this flow. This means, for
    /// each property of each entity instance and relation instance the corresponding reactive
    /// stream will be activated with it's last value.
    ///
    /// This leads to a recalculation if the instance is controlled by an behaviour which
    /// consumes the reactive streams.
    ///
    /// In case of entity instances, it furthermore leads to a new value propagation if the output
    /// property is connected to other properties.
    async fn commit(&self, context: &Context<'_>, id: Uuid) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let flow = flow_manager.get(id);
        if flow.is_none() {
            return Err(FlowMutationError::MissingFlow(id).into());
        }
        let flow = flow.unwrap();
        flow_manager.commit(flow.id);
        Ok(flow.into())
    }

    /// Creates a new entity instance and adds the entity instance to the given flow by id.
    async fn create_entity(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        #[graphql(name = "type")] type_name: String,
        entity_id: Option<Uuid>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        let flow = flow_manager.get(flow_id);
        if flow.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow = flow.unwrap();

        let entity_type = entity_type_manager.get(type_name.clone());
        if entity_type.is_none() {
            return Err(FlowMutationError::MissingEntityType(type_name.clone()).into());
        }
        let entity_type = entity_type.unwrap();

        // Get the properties pre-initialized with default values
        let mut entity_instance_builder: ReactiveEntityInstanceBuilder = entity_type.into();

        // If no id has been provided, a new id will be generated
        if entity_id.is_some() {
            entity_instance_builder.id(entity_id.unwrap());
        }

        if properties.is_some() {
            for property in properties.unwrap() {
                debug!(
                    "set property {} = {}",
                    property.name.clone(),
                    property.value.clone().to_string()
                );
                entity_instance_builder.property(property.name.clone(), property.value.clone());
            }
        }

        let entity_instance = entity_instance_builder.create(entity_instance_manager.clone())?;
        flow.add_entity(entity_instance.clone());
        // No commit necessary _> The newly created entity_instance has already been registered in the reactive_entity_instance_manager
        Ok(flow.into())
    }

    /// Adds an existing entity instance by id to the given flow by id
    async fn add_entity(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        entity_id: Uuid,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        let flow = flow_manager.get(flow_id);
        if flow.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow = flow.unwrap();

        let entity_instance = entity_instance_manager.get(entity_id);
        if entity_instance.is_none() {
            return Err(FlowMutationError::MissingEntityInstance(entity_id).into());
        }
        let entity_instance = entity_instance.unwrap();

        flow.add_entity(entity_instance.clone());
        // No commit necessary _> The entity_instance is registered in the reactive_entity_instance_manager

        Ok(flow.into())
    }

    /// Removes an entity instance from flow.
    async fn remove_entity(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        entity_id: Uuid,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        let flow = flow_manager.get(flow_id);
        if flow.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow = flow.unwrap();

        let entity_instance = entity_instance_manager.get(entity_id);
        if entity_instance.is_none() {
            return Err(FlowMutationError::MissingEntityInstance(entity_id).into());
        }
        let entity_instance = entity_instance.unwrap();

        if !flow.has_entity_by_id(entity_id) {
            return Err(FlowMutationError::FlowDoesNotContainEntityInstance(entity_id).into());
        }

        flow.remove_entity(entity_instance.id);
        // The entity is removed from the flow but not yet deleted
        // TODO: How to handle this? It may be that a entity is used in multiple flows?
        // Orphaned instances / Do not delete instances used in other flows?

        Ok(flow.into())
    }

    /// Creates a new relation instance and adds the relation instance to the given flow by id.
    async fn create_relation(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        edge_key: GraphQLEdgeKey,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_instance_manager =
            context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;

        let relation_type = relation_type_manager.get_starts_with(edge_key.type_name.clone());
        if relation_type.is_none() {
            return Err(FlowMutationError::MissingRelationType(edge_key.type_name.clone()).into());
        }
        let relation_type = relation_type.unwrap();

        let flow = flow_manager.get(flow_id);
        if flow.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow = flow.unwrap();

        if !flow.has_entity_by_id(edge_key.outbound_id) {
            return Err(
                FlowMutationError::MissingOutboundEntityInstance(edge_key.outbound_id).into(),
            );
        }

        if !flow.has_entity_by_id(edge_key.inbound_id) {
            return Err(
                FlowMutationError::MissingInboundEntityInstance(edge_key.inbound_id).into(),
            );
        }

        // TODO: optionally we could check if the entity_instance_manager contains the outbound_id and inbound_id

        // Build a new reactive relation instance
        let mut relation_instance_builder = ReactiveRelationInstanceBuilder::new(
            edge_key.outbound_id,
            relation_type.type_name.clone(),
            edge_key.inbound_id,
        );

        // Pre-initialize the properties with default values
        relation_instance_builder.set_properties_defaults(relation_type.clone());

        if properties.is_some() {
            for property in properties.unwrap() {
                debug!(
                    "set property {} = {}",
                    property.name.clone(),
                    property.value.clone().to_string()
                );
                relation_instance_builder.property(property.name.clone(), property.value.clone());
            }
        }

        // Create and register the relation instance in the reactive_relation_instance_manager
        let relation_instance =
            relation_instance_builder.create(relation_instance_manager.clone())?;

        // Add relation to flow
        flow.add_relation(relation_instance.clone());

        Ok(flow.into())
    }

    /// Adds an existing relation instance by edge_key to the given flow by id
    async fn add_relation(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        edge_key: GraphQLEdgeKey,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let relation_instance_manager =
            context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;

        let flow = flow_manager.get(flow_id);
        if flow.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow = flow.unwrap();

        let edge_key: EdgeKey = edge_key.into();
        let relation_instance = relation_instance_manager.get(edge_key.clone());
        if relation_instance.is_none() {
            return Err(FlowMutationError::MissingRelationInstance(edge_key.clone()).into());
        }
        let relation_instance = relation_instance.unwrap();

        flow.add_relation(relation_instance.clone());

        Ok(flow.into())
    }

    /// Removes an existing relation instance by edge_key from the given flow by id
    async fn remove_relation(
        &self,
        context: &Context<'_>,
        flow_id: Uuid,
        edge_key: GraphQLEdgeKey,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;

        let flow = flow_manager.get(flow_id);
        if flow.is_none() {
            return Err(FlowMutationError::MissingFlow(flow_id).into());
        }
        let flow = flow.unwrap();

        let edge_key: EdgeKey = edge_key.into();

        if !flow.has_relation_by_key(edge_key.clone()) {
            return Err(
                FlowMutationError::FlowDoesNotContainRelationInstance(edge_key.clone()).into(),
            );
        }

        flow.remove_relation(edge_key.clone());
        // The relation is removed from flow, but not yet deleted
        // TODO: How to handle this? It may be that a relation is used in multiple flows?
        // Orphaned instances / Do not delete instances used in other flows?

        Ok(flow.into())
    }

    /// Imports the given flow. Creates entity instances and relation instances which are contained
    /// in the given flow.
    async fn import(
        &self,
        context: &Context<'_>,
        flow: GraphQLFlowDefinition,
    ) -> Result<GraphQLFlow> {
        let flow_manager = context.data::<Arc<dyn ReactiveFlowManager>>()?;
        let flow = flow_manager.create(flow.into())?;
        Ok(flow.into())
    }
}
