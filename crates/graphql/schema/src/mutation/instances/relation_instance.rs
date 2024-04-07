use std::sync::Arc;

use async_graphql::*;
use log::debug;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourManager;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::RelationTypeManager;

use crate::mutation::BehaviourTypeIdDefinition;
use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::GraphQLRelationInstanceId;
use crate::mutation::RelationTypeIdDefinition;
use crate::query::GraphQLPropertyInstance;
use crate::query::GraphQLRelationInstance;

#[derive(Default)]
pub struct MutationRelationInstances;

/// Mutation of relation instances.
#[Object]
impl MutationRelationInstances {
    /// Creates a new relation instance with the given edge_key.
    ///
    /// The edge key is the primary key of a relation instance and consists of the id of the
    /// outbound entity instance, the name of the relation type and the id of the inbound
    /// entity instance.
    ///
    /// The relation type must exist and the given type name is matched by a prefix search.
    /// For example a given type name "default_connector--property_name--property_name" will match
    /// as relation type "default_connector".
    ///
    /// Furthermore the outbound and the inbound entity instance must exist.
    ///
    /// The given properties consists of a list of pairs of property name and property value.
    /// If properties are not provided, default values will be used depending on the data type
    /// of the property.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Specifies the outbound id, the inbound id, the relation type and the instance_id.")] edge_key: GraphQLRelationInstanceId,
        #[graphql(desc = "Creates the relation instance with the given components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        let relation_instance_ty = edge_key.ty();
        let relation_ty = relation_instance_ty.relation_type_id();

        let relation_type = relation_type_manager
            .get(&relation_ty)
            .ok_or_else(|| Error::new(format!("Relation type {} does not exist!", &relation_ty)))?;

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, relation_type.properties);
        let id: RelationInstanceId = edge_key.into();

        let relation_instance = relation_instance_manager
            .create_reactive_relation(&id, properties)
            .map_err(|e| Error::new(format!("Failed to create relation instance: {:?}", e)))?;

        if let Some(components) = components {
            for component in components {
                let component = component.into();
                // TODO: handle components which have not been added
                let _ = relation_instance_manager.add_component(&id, &component);
            }
        }
        Ok(relation_instance.into())
    }

    /// Creates a connector from a property of the outbound entity instance to a property of the inbound entity instance.
    ///
    /// The type_name must match a relation type exactly.
    async fn create_connector(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The id of the outbound entity instance")] outbound_id: Uuid,
        #[graphql(desc = "The name of the property of the outbound entity instance")] outbound_property_name: String,
        #[graphql(name = "type", desc = "The name of the connector relation type")] relation_ty: RelationTypeIdDefinition,
        #[graphql(desc = "The id of the inbound entity instance")] inbound_id: Uuid,
        #[graphql(desc = "The name of the property of the inbound entity instance")] inbound_property_name: String,
        #[graphql(desc = "Creates the relation instance with the given components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "The initial property values")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        let relation_ty = relation_ty.into();

        // Resolve the relation type or throw error
        let relation_type = relation_type_manager
            .get(&relation_ty)
            .ok_or(Error::new(format!("Connector relation type {} does not exist!", &relation_ty)))?;

        // The outbound entity instance must exist
        if !entity_instance_manager.has(outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", outbound_id)));
        }

        // The inbound entity instance must exist
        if !entity_instance_manager.has(inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", inbound_id)));
        }

        // The outbound entity instance's property must exist
        if entity_instance_manager.get(outbound_id).map(|e| e.get(&outbound_property_name)).is_none() {
            return Err(Error::new(format!("Outbound entity {} has no property named {}!", outbound_id, outbound_property_name)));
        }

        // The inbound entity instance's property must exist
        if entity_instance_manager.get(inbound_id).map(|e| e.get(&inbound_property_name)).is_none() {
            return Err(Error::new(format!("Inbound entity {} has no property named {}!", inbound_id, inbound_property_name)));
        }

        // Construct the instance_id because between two nodes only one edge with the same type
        // can exist. Therefore we construct an unique type which contains the names of the outbound
        // property and the inbound property. This allows *exactly one* connector (of the given
        // connector type) between the two properties.
        let instance_id = format!("{}__{}", outbound_property_name, inbound_property_name);
        let ty = RelationInstanceTypeId::new_unique_for_instance_id(relation_ty, instance_id);

        // Construct an edge key using the outbound id, the type identifier (containing the
        // previously generated instance_id) and the inbound id.
        let id = RelationInstanceId::new(outbound_id, ty, inbound_id);

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, relation_type.properties);
        properties.insert("outbound_property_name".to_string(), json!(outbound_property_name));
        properties.insert("inbound_property_name".to_string(), json!(inbound_property_name));
        match relation_instance_manager.create_reactive_relation(&id, properties) {
            Ok(relation_instance) => {
                // If created successfully, add additional components
                if let Some(components) = components {
                    for component in components {
                        // TODO: handle components which have not been added
                        let _ = relation_instance_manager.add_component(&id, &component.into());
                    }
                }
                Ok(relation_instance.into())
            }
            Err(creation_error) => Err(Error::new(format!("Failed to create relation instance: {:?}", creation_error))),
        }
    }

    /// Updates the properties of the given relation instance by edge key.
    async fn update(
        &self,
        context: &Context<'_>,
        edge_key: GraphQLRelationInstanceId,
        #[graphql(desc = "Adds the components with the given name")] add_components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Removes the components with the given name")] remove_components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Updates the given properties")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        let ty = edge_key.ty();
        let relation_ty = ty.relation_type_id();

        if relation_type_manager.get(&relation_ty).is_none() {
            return Err(Error::new(format!("Relation type {} does not exist!", edge_key)));
        }

        let id: RelationInstanceId = edge_key.into();
        let relation_instance = relation_instance_manager
            .get(&id)
            .ok_or_else(|| Error::new(format!("Relation instance {} does not exist!", id)))?;

        if let Some(components) = add_components {
            for component in components {
                // TODO: handle components which have not been added
                let _ = relation_instance_manager.add_component(&id, &component.into());
            }
        }
        if let Some(components) = remove_components {
            for component in components {
                // TODO: handle components which have not been removed
                let _ = relation_instance_manager.remove_component(&id, &component.into());
            }
        }
        if let Some(properties) = properties {
            // fill all values first without propagation
            for property in properties.clone() {
                debug!("set property {} = {}", property.name.clone(), property.value.clone().to_string());
                // Set with respect to the mutability state
                relation_instance.set_no_propagate_checked(property.name.clone(), property.value.clone());
            }
            // tick every property that has been changed before, this is still not transactional
            for property in properties {
                debug!("tick property {} = {}", property.name.clone(), property.value.clone().to_string());
                if let Some(property_instance) = relation_instance.properties.get(property.name.as_str()) {
                    // Tick with respect to the mutability state
                    property_instance.tick_checked();
                }
            }
        }
        // TODO: it's still not a transactional mutation
        // relation_instance.tick();
        Ok(relation_instance.into())
    }

    /// Manually tick the relation instance. This means for each property of the entity instance
    /// the corresponding reactive stream will be activated with it's last value.
    ///
    /// This leads to a recalculation if the relation instance is controlled by an behaviour which
    /// consumes the reactive streams.
    ///
    /// In case of the default_connector it does NOT lead to a new value propagation, because the
    /// reactive streams are not consumed by the default_connector behaviour.
    async fn tick(&self, context: &Context<'_>, edge_key: GraphQLRelationInstanceId) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let id = edge_key.into();
        let relation_instance = relation_instance_manager
            .get(&id)
            .ok_or_else(|| Error::new(format!("Relation instance {} does not exist!", id)))?;
        relation_instance.tick();
        Ok(relation_instance.into())
    }

    /// Deletes an relation instance.
    async fn delete(&self, context: &Context<'_>, edge_key: GraphQLRelationInstanceId) -> Result<bool> {
        // let relation_type_manager = context.data::<Arc<dyn RelationTypeManager + Send + Sync>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;

        debug!("Deleting relation instance {:?}", edge_key);

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        Ok(relation_instance_manager.delete(&edge_key.into()))
    }

    async fn connect(
        &self,
        context: &Context<'_>,
        edge_key: GraphQLRelationInstanceId,
        #[graphql(name = "type")] behaviour_ty: BehaviourTypeIdDefinition,
    ) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager + Send + Sync>>()?;
        let relation_component_behaviour_manager = context.data::<Arc<dyn RelationComponentBehaviourManager + Send + Sync>>()?;
        let edge_key = RelationInstanceId::from(edge_key);
        let reactive_instance = relation_instance_manager.get(&edge_key).ok_or(Error::new("Relation instance not found"))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if relation_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_behaviour_manager
                .connect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to connect relation behaviour {:?}", e)))?;
        }
        if relation_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_component_behaviour_manager
                .connect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to connect relation component behaviour {:?}", e)))?;
        }
        Ok(reactive_instance.into())
    }

    async fn disconnect(
        &self,
        context: &Context<'_>,
        edge_key: GraphQLRelationInstanceId,
        #[graphql(name = "type")] behaviour_ty: BehaviourTypeIdDefinition,
    ) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager + Send + Sync>>()?;
        let relation_component_behaviour_manager = context.data::<Arc<dyn RelationComponentBehaviourManager + Send + Sync>>()?;
        let edge_key = RelationInstanceId::from(edge_key);
        let reactive_instance = relation_instance_manager.get(&edge_key).ok_or(Error::new("Relation instance not found"))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if relation_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_behaviour_manager
                .disconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to disconnect relation behaviour {:?}", e)))?;
        }
        if relation_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_component_behaviour_manager
                .disconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to disconnect relation component behaviour {:?}", e)))?;
        }
        Ok(reactive_instance.into())
    }

    async fn reconnect(
        &self,
        context: &Context<'_>,
        edge_key: GraphQLRelationInstanceId,
        #[graphql(name = "type")] behaviour_ty: BehaviourTypeIdDefinition,
    ) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
        let relation_behaviour_manager = context.data::<Arc<dyn RelationBehaviourManager + Send + Sync>>()?;
        let relation_component_behaviour_manager = context.data::<Arc<dyn RelationComponentBehaviourManager + Send + Sync>>()?;
        let edge_key = RelationInstanceId::from(edge_key);
        let reactive_instance = relation_instance_manager.get(&edge_key).ok_or(Error::new("Relation instance not found"))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if relation_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_behaviour_manager
                .reconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to reconnect relation behaviour {:?}", e)))?;
        }
        if relation_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            relation_component_behaviour_manager
                .reconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to reconnect relation component behaviour {:?}", e)))?;
        }
        Ok(reactive_instance.into())
    }
}
