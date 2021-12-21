use std::sync::Arc;

use async_graphql::*;
use log::debug;

use crate::api::{ReactiveEntityInstanceManager, ReactiveRelationInstanceCreationError, ReactiveRelationInstanceManager, RelationTypeManager};
use crate::graphql::mutation::GraphQLEdgeKey;
use crate::graphql::query::{GraphQLPropertyInstance, GraphQLRelationInstance};
use crate::model::PropertyInstanceSetter;
use indradb::EdgeKey;

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
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        let relation_type = relation_type_manager.get_starts_with(edge_key.type_name.clone());
        if relation_type.is_none() {
            return Err(Error::new(format!("Relation type {} does not exist!", edge_key.type_name.clone())));
        }

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        let properties = GraphQLPropertyInstance::to_map_with_defaults(properties, relation_type.unwrap().properties);

        let relation_instance = match indradb::Type::new(edge_key.type_name.clone()) {
            Ok(_) => {
                let edge_key: EdgeKey = edge_key.into();
                relation_instance_manager.create(edge_key, properties)
            }
            Err(err) => Err(ReactiveRelationInstanceCreationError::ValidationError(err).into()),
        };

        if relation_instance.is_err() {
            return Err(Error::new("Failed to create relation instance"));
        }
        let relation_instance = relation_instance.unwrap();
        Ok(relation_instance.into())
    }

    // TODO: clone(id) -> GraphQLRelationInstance

    /// Updates the properties of the given relation instance by edge key.
    async fn update(&self, context: &Context<'_>, edge_key: GraphQLEdgeKey, properties: Vec<GraphQLPropertyInstance>) -> Result<GraphQLRelationInstance> {
        let relation_type_manager = context.data::<Arc<dyn RelationTypeManager>>()?;
        let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;

        if !entity_instance_manager.has(edge_key.outbound_id) {
            return Err(Error::new(format!("Outbound entity {} does not exist!", edge_key.outbound_id)));
        }

        if !entity_instance_manager.has(edge_key.inbound_id) {
            return Err(Error::new(format!("Inbound entity {} does not exist!", edge_key.inbound_id)));
        }

        let relation_type = relation_type_manager.get_starts_with(edge_key.type_name.clone());
        if relation_type.is_none() {
            return Err(Error::new(format!("Relation type {} does not exist!", edge_key.type_name.clone())));
        }

        let relation_instance = relation_instance_manager.get(edge_key.clone().into());
        if relation_instance.is_none() {
            return Err(Error::new(format!("Relation instance {} does not exist!", edge_key.to_string())));
        }
        let relation_instance = relation_instance.unwrap();

        for property in properties {
            debug!("set property {} = {}", property.name.clone(), property.value.clone().to_string());
            relation_instance.set_no_propagate(property.name.clone(), property.value.clone());
        }
        // TODO: it's still not a transactional mutation
        relation_instance.tick();
        Ok(relation_instance.clone().into())
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
        Ok(relation_instance.clone().into())
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

        let relation_type = relation_type_manager.get_starts_with(edge_key.type_name.clone());
        if relation_type.is_none() {
            return Err(Error::new(format!("Relation type {} does not exist!", edge_key.type_name.clone())));
        }

        Ok(relation_instance_manager.delete(edge_key.into()))
    }
}
