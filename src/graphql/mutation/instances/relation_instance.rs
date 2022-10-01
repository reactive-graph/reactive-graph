use std::sync::Arc;

use async_graphql::*;
use indradb::EdgeKey;
use log::debug;
use serde_json::json;
use uuid::Uuid;

use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceCreationError;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::graphql::mutation::GraphQLEdgeKey;
use crate::graphql::query::GraphQLPropertyInstance;
use crate::graphql::query::GraphQLRelationInstance;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactivePropertyContainer;

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
        edge_key: GraphQLEdgeKey,
        #[graphql(desc = "Creates the relation instance with the given components.")] components: Option<Vec<String>>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        let relation_type = relation_type_manager.get_starts_with(&edge_key.type_name);
        if relation_type.is_none() {
            return Err(Error::new(format!("Relation type {} does not exist!", edge_key.type_name)));
        }

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        let properties = GraphQLPropertyInstance::to_map_with_defaults(properties, relation_type.unwrap().properties);

        let relation_instance = match indradb::Identifier::new(edge_key.type_name.clone()) {
            Ok(_) => {
                let edge_key: EdgeKey = edge_key.clone().into();
                relation_instance_manager.create(edge_key, properties)
            }
            Err(err) => Err(ReactiveRelationInstanceCreationError::ValidationError(err)),
        };

        if relation_instance.is_err() {
            return Err(Error::new(format!("Failed to create relation instance: {:?}", relation_instance.err().unwrap())));
        }
        let relation_instance = relation_instance.unwrap();
        let edge_key: EdgeKey = edge_key.into();
        if let Some(components) = components {
            for component in components {
                relation_instance_manager.add_component(edge_key.clone(), &component);
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
        #[graphql(desc = "The name of the connector relation type")] type_name: String,
        #[graphql(desc = "The id of the inbound entity instance")] inbound_id: Uuid,
        #[graphql(desc = "The name of the property of the inbound entity instance")] inbound_property_name: String,
        #[graphql(desc = "Creates the relation instance with the given components.")] components: Option<Vec<String>>,
        #[graphql(desc = "The initial property values")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        // Requires an exact match of a type name!
        let relation_type = relation_type_manager.get(&type_name);
        if relation_type.is_none() {
            return Err(Error::new(format!("Connector relation type {} does not exist!", type_name)));
        }

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

        // Construct the instanceTypeName because between two nodes only one edge with the same type can exist
        // Therefore we construct an unique type name which contains the names of the outbound property and the inbound property
        // This allows exactly one connector (of the given connector type) between the two properties.
        let instance_type_name = format!("{}--{}--{}", type_name, outbound_property_name, inbound_property_name);

        match indradb::Identifier::new(instance_type_name) {
            Ok(instance_type_identifier) => {
                let edge_key = EdgeKey::new(outbound_id, instance_type_identifier, inbound_id);
                let mut properties = GraphQLPropertyInstance::to_map_with_defaults(properties, relation_type.unwrap().properties);
                properties.insert("outbound_property_name".to_string(), json!(outbound_property_name));
                properties.insert("inbound_property_name".to_string(), json!(inbound_property_name));
                match relation_instance_manager.create(edge_key.clone(), properties) {
                    Ok(relation_instance) => {
                        // If created successfully, add additional components
                        if let Some(components) = components {
                            for component in components {
                                relation_instance_manager.add_component(edge_key.clone(), &component);
                            }
                        }
                        Ok(relation_instance.into())
                    }
                    Err(creation_error) => Err(Error::new(format!("Failed to create relation instance: {:?}", creation_error))),
                }
            }
            Err(validation_error) => Err(Error::new(format!("Failed to create relation instance: {:?}", validation_error))),
        }
    }

    /// Updates the properties of the given relation instance by edge key.
    async fn update(
        &self,
        context: &Context<'_>,
        edge_key: GraphQLEdgeKey,
        #[graphql(desc = "Adds the components with the given name")] add_components: Option<Vec<String>>,
        #[graphql(desc = "Removes the components with the given name")] remove_components: Option<Vec<String>>,
        #[graphql(desc = "Updates the given properties")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        let relation_type = relation_type_manager.get_starts_with(&edge_key.type_name);
        if relation_type.is_none() {
            return Err(Error::new(format!("Relation type {} does not exist!", edge_key.type_name)));
        }

        let relation_instance = relation_instance_manager.get(edge_key.clone().into());
        if relation_instance.is_none() {
            return Err(Error::new(format!("Relation instance {} does not exist!", edge_key)));
        }
        let relation_instance = relation_instance.unwrap();

        if let Some(components) = add_components {
            for component in components {
                relation_instance_manager.add_component(edge_key.clone().into(), &component);
            }
        }
        if let Some(components) = remove_components {
            for component in components {
                relation_instance_manager.remove_component(edge_key.clone().into(), &component);
            }
        }
        if let Some(properties) = properties {
            // fill all values first without propagation
            for property in properties.clone() {
                debug!("set property {} = {}", property.name.clone(), property.value.clone().to_string());
                relation_instance.set_no_propagate(property.name.clone(), property.value.clone());
            }
            // tick every property that has been changed before, this is still not transactional
            for property in properties {
                debug!("tick property {} = {}", property.name.clone(), property.value.clone().to_string());
                if let Some(property_instance) = relation_instance.properties.get(property.name.as_str()) {
                    property_instance.tick();
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
    async fn tick(&self, context: &Context<'_>, edge_key: GraphQLEdgeKey) -> Result<GraphQLRelationInstance> {
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
        let relation_instance = relation_instance_manager.get(edge_key.clone().into());
        if relation_instance.is_none() {
            return Err(Error::new(format!("Relation instance {} does not exist!", edge_key)));
        }
        let relation_instance = relation_instance.unwrap();
        relation_instance.tick();
        Ok(relation_instance.into())
    }

    /// Deletes an relation instance.
    async fn delete(&self, context: &Context<'_>, edge_key: GraphQLEdgeKey) -> Result<bool> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        debug!("Deleting relation instance {:?}", edge_key);

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        let relation_type = relation_type_manager.get_starts_with(&edge_key.type_name);
        if relation_type.is_none() {
            return Err(Error::new(format!("Relation type {} does not exist!", edge_key.type_name)));
        }

        Ok(relation_instance_manager.delete(edge_key.into()))
    }
}
