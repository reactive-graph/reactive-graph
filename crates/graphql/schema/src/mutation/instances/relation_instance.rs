use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use log::debug;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourManager;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::CreatePropertyConnectorError;
use reactive_graph_graph::CreateRelationInstanceError;
use reactive_graph_graph::NamespacedTypeConstructor;
use reactive_graph_graph::NamespacedTypeIdContainer;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RemoveRelationInstanceError;
use reactive_graph_graph::TriggerRelationInstanceError;
use reactive_graph_graph::UpdateRelationInstanceError;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::RelationTypeManager;
use serde_json::json;

use crate::mutation::GraphQLExtensionDefinition;
use crate::mutation::GraphQLExtensionDefinitions;
use crate::mutation::GraphQLPropertyConnectorId;
use crate::mutation::GraphQLRelationInstanceId;
use crate::mutation::PropertyTypeDefinitions;
use crate::query::GraphQLPropertyInstance;
use crate::query::GraphQLRelationInstance;

#[derive(Default)]
pub struct MutationRelationInstances;

/// Mutation of relation instances.
#[Object]
impl MutationRelationInstances {
    /// Creates a new relation instance with the given relation_instance_id.
    ///
    /// The relation instance id is the primary key of a relation instance and consists of the id of the
    /// outbound entity instance, the name of the relation type and the id of the inbound
    /// entity instance.
    ///
    /// The relation type must exist and the given type name is matched by a prefix search.
    /// For example a given type name "default_connector--property_name--property_name" will match
    /// as relation type "default_connector".
    ///
    /// Furthermore, the outbound and the inbound entity instance must exist.
    ///
    /// The given properties consists of a list of pairs of property name and property value.
    /// If properties are not provided, default values will be used depending on the data type
    /// of the property.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Specifies the outbound id, the inbound id, the relation type and the instance_id.")] relation_instance_id: GraphQLRelationInstanceId,
        #[graphql(desc = "Name of the relation instance.")] name: Option<String>,
        #[graphql(desc = "Description of the relation instance.")] description: Option<String>,
        #[graphql(desc = "Creates the relation instance with the given components.")] components: Option<Vec<String>>,
        #[graphql(
            desc = "Initial value of the properties to set. If properties are not provided, default values will be used depending on the data type of the property."
        )]
        properties: Option<Vec<GraphQLPropertyInstance>>,
        #[graphql(desc = "The extensions of the relation instance.")] extensions: Option<Vec<GraphQLExtensionDefinition>>,
    ) -> Result<GraphQLRelationInstance> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let relation_ty = relation_instance_id.ty.relation_type_id();

        let component_tys = ComponentTypeIds::parse_optional_namespaces(components)?;
        let extensions = GraphQLExtensionDefinitions::parse_optional_definitions(extensions)?;

        let relation_type = relation_type_manager
            .get(&relation_ty)
            .ok_or(CreateRelationInstanceError::RelationTypeDoesNotExist(relation_ty.clone()))?;

        if !reactive_entity_manager.has(relation_instance_id.outbound_id) {
            return Err(CreateRelationInstanceError::OutboundEntityInstanceDoesNotExist(relation_ty, relation_instance_id.outbound_id).into());
        }

        if !reactive_entity_manager.has(relation_instance_id.inbound_id) {
            return Err(CreateRelationInstanceError::InboundEntityInstanceDoesNotExist(relation_ty, relation_instance_id.inbound_id).into());
        }

        if !component_manager.has_all(&component_tys) {
            return Err(CreateRelationInstanceError::ComponentsDoesNotExist.into());
        }

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, relation_type.properties);

        let relation_instance = RelationInstance::builder()
            .outbound_id(relation_instance_id.outbound_id)
            .ty(relation_instance_id.ty.clone())
            .inbound_id(relation_instance_id.inbound_id)
            .name(name.unwrap_or_default())
            .description(description.unwrap_or_default())
            .properties(properties)
            .extensions(extensions)
            .build();

        // let id = RelationInstanceId::from(relation_instance_id);
        // let id: RelationInstanceId = .into();

        let reactive_relation = reactive_relation_manager.create_reactive_instance(relation_instance)?;

        for component_ty in component_tys {
            // TODO: handle components which have not been added
            let _ = reactive_relation_manager.add_component(&relation_instance_id, &component_ty);
        }
        Ok(reactive_relation.into())
    }

    /// Creates a connector from a property of the outbound entity instance to a property of the inbound entity instance.
    ///
    /// The type_name must match a relation type exactly.
    #[allow(clippy::too_many_arguments)]
    async fn create_connector(
        &self,
        context: &Context<'_>,
        #[graphql(name = "id", desc = "The id of the property connector instance")] connector_id: GraphQLPropertyConnectorId,
        #[graphql(desc = "Name of the relation instance.")] name: Option<String>,
        #[graphql(desc = "Description of the relation instance.")] description: Option<String>,
        #[graphql(desc = "Creates the relation instance with the given components.")] components: Option<Vec<String>>,
        #[graphql(desc = "The initial property values")] properties: Option<Vec<GraphQLPropertyInstance>>,
        #[graphql(desc = "The extensions of the relation instance.")] extensions: Option<Vec<GraphQLExtensionDefinition>>,
    ) -> Result<GraphQLRelationInstance> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;

        let ty = connector_id.parse()?;
        let relation_ty = ty.relation_type_id();
        let component_tys = ComponentTypeIds::parse_optional_namespaces(components)?;
        let extensions = GraphQLExtensionDefinitions::parse_optional_definitions(extensions)?;

        // Resolve the relation type or throw error
        let relation_type = relation_type_manager
            .get(&relation_ty)
            .ok_or(CreateRelationInstanceError::RelationTypeDoesNotExist(relation_ty.clone()))?;

        // The outbound entity instance must exist
        let outbound = reactive_entity_manager
            .get(connector_id.outbound.id)
            .ok_or(CreateRelationInstanceError::OutboundEntityInstanceDoesNotExist(relation_ty.clone(), connector_id.outbound.id))?;

        // The inbound entity instance must exist
        let inbound = reactive_entity_manager
            .get(connector_id.inbound.id)
            .ok_or(CreateRelationInstanceError::InboundEntityInstanceDoesNotExist(relation_ty.clone(), connector_id.inbound.id))?;

        // The outbound entity instance's property must exist
        if !outbound.has_property(&connector_id.outbound.property_name) {
            return Err(CreatePropertyConnectorError::OutboundPropertyDoesNotExist(
                relation_ty.clone(),
                connector_id.outbound.id,
                connector_id.outbound.property_name.clone(),
            )
            .into());
        }

        // The inbound entity instance's property must exist
        if !inbound.has_property(&connector_id.inbound.property_name) {
            return Err(CreatePropertyConnectorError::InboundPropertyDoesNotExist(
                relation_ty.clone(),
                connector_id.inbound.id,
                connector_id.inbound.property_name.clone(),
            )
            .into());
        }

        if !component_manager.has_all(&component_tys) {
            return Err(CreateRelationInstanceError::ComponentsDoesNotExist.into());
        }

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, relation_type.properties);
        // The relationship stores references to the outbound property and inbound property in its
        // own properties:
        properties.insert("outbound_property_name".to_string(), json!(connector_id.outbound.property_name));
        properties.insert("inbound_property_name".to_string(), json!(connector_id.inbound.property_name));

        let relation_instance = RelationInstance::builder()
            .outbound_id(connector_id.outbound.id)
            .ty(ty)
            .inbound_id(connector_id.inbound.id)
            .name(name.unwrap_or_default())
            .description(description.unwrap_or_default())
            .properties(properties)
            .extensions(extensions)
            .build();

        let id = relation_instance.id();

        let reactive_relation = reactive_relation_manager.create_reactive_instance(relation_instance)?;

        for component_ty in component_tys {
            // TODO: handle components which have not been added
            let _ = reactive_relation_manager.add_component(&id, &component_ty);
        }
        Ok(reactive_relation.into())
    }

    /// Updates the properties of the given relation instance by its relation instance id.
    #[allow(clippy::too_many_arguments)]
    async fn update(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Specifies the outbound id, the inbound id, the relation type and the instance_id.")] relation_instance_id: GraphQLRelationInstanceId,
        #[graphql(desc = "Adds the components with the given name")] add_components: Option<Vec<String>>,
        #[graphql(desc = "Removes the components with the given name")] remove_components: Option<Vec<String>>,
        #[graphql(desc = "Updates the given properties")] properties: Option<Vec<GraphQLPropertyInstance>>,
        #[graphql(desc = "Adds the given properties")] add_properties: Option<Vec<crate::mutation::PropertyTypeDefinition>>,
        #[graphql(desc = "Removes the given properties")] remove_properties: Option<Vec<String>>,
    ) -> Result<GraphQLRelationInstance> {
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;

        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let relation_ty = relation_instance_id.ty.relation_type_id();
        let add_component_tys = ComponentTypeIds::parse_optional_namespaces(add_components)?;
        let remove_component_tys = ComponentTypeIds::parse_optional_namespaces(remove_components)?;
        let add_properties = PropertyTypeDefinitions::parse_optional_definitions(add_properties)?;

        if !relation_type_manager.has(&relation_ty) {
            return Err(UpdateRelationInstanceError::RelationTypeDoesNotExist(relation_ty.clone()).into());
        }

        if !reactive_entity_manager.has(relation_instance_id.outbound_id) {
            return Err(UpdateRelationInstanceError::OutboundEntityInstanceDoesNotExist(relation_instance_id.outbound_id).into());
        }

        if !reactive_entity_manager.has(relation_instance_id.inbound_id) {
            return Err(UpdateRelationInstanceError::InboundEntityInstanceDoesNotExist(relation_instance_id.inbound_id).into());
        }

        if !component_manager.has_all(&add_component_tys) || !component_manager.has_all(&remove_component_tys) {
            return Err(CreateRelationInstanceError::ComponentsDoesNotExist.into());
        }

        let reactive_relation = reactive_relation_manager
            .get(&relation_instance_id)
            .ok_or(UpdateRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id.clone()))?;

        for component_ty in add_component_tys {
            // TODO: handle components which have not been added
            let _ = reactive_relation_manager.add_component(&relation_instance_id, &component_ty);
        }
        for component_ty in remove_component_tys {
            // TODO: handle components which have not been removed
            let _ = reactive_relation_manager.remove_component(&relation_instance_id, &component_ty);
        }
        if let Some(properties) = properties {
            // fill all values first without propagation
            for property in properties.clone() {
                debug!("set property {} = {}", property.name.clone(), property.value.clone());
                // Set with respect to the mutability state
                reactive_relation.set_no_propagate_checked(property.name.clone(), property.value.clone());
            }
            // tick every property that has been changed before, this is still not transactional
            for property in properties {
                debug!("tick property {} = {}", property.name.clone(), property.value.clone());
                if let Some(property_instance) = reactive_relation.properties.get(property.name.as_str()) {
                    // Tick with respect to the mutability state
                    property_instance.tick_checked();
                }
            }
            // TODO: it's still not a transactional mutation
        }
        for property_type in add_properties.iter() {
            // let property_type: PropertyType = property_type.into();
            debug!("add property {} ({})", &property_type.name, &property_type.data_type);
            reactive_relation.add_property_by_type(&property_type);
        }
        if let Some(remove_properties) = remove_properties {
            for property_name in remove_properties.clone() {
                debug!("remove property {}", &property_name);
                reactive_relation.remove_property(property_name);
            }
        }
        Ok(reactive_relation.into())
    }

    /// Triggers the relation instance with the given id.
    async fn trigger(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Specifies the outbound id, the inbound id, the relation type and the instance_id.")] relation_instance_id: GraphQLRelationInstanceId,
    ) -> Result<GraphQLRelationInstance> {
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let reactive_relation = reactive_relation_manager
            .get(&relation_instance_id)
            .ok_or(TriggerRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id.clone()))?;
        if !reactive_relation.has_property(&TRIGGER.property_name()) {
            return Err(TriggerRelationInstanceError::TriggerPropertyMissing(relation_instance_id).into());
        }
        reactive_relation.set_checked(TRIGGER.property_name(), json!(true));
        Ok(reactive_relation.into())
    }

    /// Manually tick the relation instance. This means for each property of the entity instance
    /// the corresponding reactive stream will be activated with its last value.
    ///
    /// This leads to a recalculation if the relation instance is controlled by a behaviour which
    /// consumes the reactive streams.
    ///
    /// In case of the default_connector it does NOT lead to a new value propagation, because the
    /// reactive streams are not consumed by the default_connector behaviour.
    async fn tick(&self, context: &Context<'_>, relation_instance_id: GraphQLRelationInstanceId) -> Result<GraphQLRelationInstance> {
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let relation_instance = reactive_relation_manager
            .get(&relation_instance_id)
            .ok_or(UpdateRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id))?;
        relation_instance.tick();
        Ok(relation_instance.into())
    }

    /// Deletes a relation instance.
    async fn delete(&self, context: &Context<'_>, relation_instance_id: GraphQLRelationInstanceId) -> Result<bool> {
        let reactive_relation_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        if !reactive_relation_manager.has(&relation_instance_id) {
            return Err(RemoveRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id).into());
        }
        if !entity_instance_manager.has(relation_instance_id.outbound_id) {
            return Err(RemoveRelationInstanceError::OutboundEntityInstanceDoesNotExist(relation_instance_id.outbound_id).into());
        }
        if !entity_instance_manager.has(relation_instance_id.inbound_id) {
            return Err(RemoveRelationInstanceError::InboundEntityInstanceDoesNotExist(relation_instance_id.inbound_id).into());
        }
        Ok(reactive_relation_manager.delete(&relation_instance_id))
    }

    async fn connect(
        &self,
        context: &Context<'_>,
        relation_instance_id: GraphQLRelationInstanceId,
        #[graphql(name = "behaviour", desc = "The fully qualified namespace of the behaviour")] behaviour_namespace: String,
    ) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager + Send + Sync>>()?;
        let relation_component_behaviour_manager = context.data::<Arc<dyn RelationComponentBehaviourManager + Send + Sync>>()?;
        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let behaviour_ty = BehaviourTypeId::parse_namespace(&behaviour_namespace)?;
        let reactive_instance = relation_instance_manager
            .get(&relation_instance_id)
            .ok_or(UpdateRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if relation_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_behaviour_manager.connect(reactive_instance.clone(), &behaviour_ty)?;
        }
        if relation_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_component_behaviour_manager.connect(reactive_instance.clone(), &behaviour_ty)?;
        }
        Ok(reactive_instance.into())
    }

    async fn disconnect(
        &self,
        context: &Context<'_>,
        relation_instance_id: GraphQLRelationInstanceId,
        #[graphql(name = "behaviour", desc = "The fully qualified namespace of the behaviour")] behaviour_namespace: String,
    ) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager + Send + Sync>>()?;
        let relation_component_behaviour_manager = context.data::<Arc<dyn RelationComponentBehaviourManager + Send + Sync>>()?;
        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let behaviour_ty = BehaviourTypeId::parse_namespace(&behaviour_namespace)?;
        let reactive_instance = relation_instance_manager
            .get(&relation_instance_id)
            .ok_or(UpdateRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if relation_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_behaviour_manager.disconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        if relation_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_component_behaviour_manager.disconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        Ok(reactive_instance.into())
    }

    async fn reconnect(
        &self,
        context: &Context<'_>,
        relation_instance_id: GraphQLRelationInstanceId,
        #[graphql(name = "behaviour", desc = "The fully qualified namespace of the behaviour")] behaviour_namespace: String,
    ) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager + Send + Sync>>()?;
        let relation_component_behaviour_manager = context.data::<Arc<dyn RelationComponentBehaviourManager + Send + Sync>>()?;
        let relation_instance_id = RelationInstanceId::try_from(relation_instance_id)?;
        let behaviour_ty = BehaviourTypeId::parse_namespace(&behaviour_namespace)?;
        let reactive_instance = relation_instance_manager
            .get(&relation_instance_id)
            .ok_or(UpdateRelationInstanceError::RelationInstanceDoesNotExist(relation_instance_id))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if relation_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_behaviour_manager.reconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        if relation_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_component_behaviour_manager.reconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        Ok(reactive_instance.into())
    }
}
