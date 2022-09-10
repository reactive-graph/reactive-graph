use std::sync::Arc;

use async_graphql::*;
use log::debug;
use uuid::Uuid;

use crate::api::{EntityTypeManager, ReactiveEntityInstanceManager, ReactiveRelationInstanceManager};
use crate::graphql::query::{GraphQLEntityInstance, GraphQLPropertyInstance};
use crate::model::PropertyInstanceSetter;

#[derive(Default)]
pub struct MutationEntityInstances;

/// Mutation of entity instances.
#[Object]
impl MutationEntityInstances {
    /// Creates a new entity instance of the given type.
    ///
    /// The entity type must exist.
    ///
    /// Optionally, an UUID can be specified. If no UUID is specified one will be generated
    /// randomly.
    ///
    /// The given properties consists of a list of pairs of property name and property value.
    /// If properties are not provided, default values will be used depending on the data type
    /// of the property.
    async fn create(
        &self,
        context: &Context<'_>,
        #[graphql(name = "type", desc = "The entity type.")] type_name: String,
        #[graphql(desc = "The id of the entity instance. If none is given a random uuid will be generated.")] id: Option<Uuid>,
        #[graphql(desc = "Creates the entity instance with the given components.")] components: Option<Vec<String>>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        let entity_type = entity_type_manager.get(type_name.clone());

        if entity_type.is_none() {
            return Err(Error::new(format!("Entity type {type_name} does not exist")));
        }

        let properties = GraphQLPropertyInstance::to_map_with_defaults(properties, entity_type.unwrap().properties);

        let entity_instance = match id {
            Some(id) => entity_instance_manager.create_with_id(type_name, id, properties),
            None => entity_instance_manager.create(type_name, properties),
        };
        if entity_instance.is_err() {
            return Err(Error::new(entity_instance.err().unwrap().to_string()));
        }
        let entity_instance = entity_instance.unwrap();
        if let Some(components) = components {
            for component in components {
                entity_instance_manager.add_component(entity_instance.id, component.clone());
            }
        }
        Ok(entity_instance.into())
    }

    /// Updates the properties of the entity instance with the given id.
    async fn update(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Updates the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Updates the entity instance with the given label.")] label: Option<String>,
        #[graphql(desc = "Updates the entity instance with the given label.")] add_components: Option<Vec<String>>,
        #[graphql(desc = "Updates the entity instance with the given label.")] remove_components: Option<Vec<String>>,
        #[graphql(desc = "Updates the given properties")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_instance;
        if id.is_some() {
            entity_instance = entity_instance_manager.get(id.unwrap());
        } else if label.is_some() {
            entity_instance = entity_instance_manager.get_by_label(label.unwrap());
        } else {
            return Err("Either id or label must be given!".into());
        }
        if entity_instance.is_none() {
            return Err("Entity instance not found!".into());
        }
        let entity_instance = entity_instance.unwrap();

        if let Some(components) = add_components {
            for component in components {
                entity_instance_manager.add_component(entity_instance.id, component.clone());
            }
        }
        if let Some(components) = remove_components {
            for component in components {
                entity_instance_manager.remove_component(entity_instance.id, component.clone());
            }
        }
        if let Some(properties) = properties {
            for property in properties {
                debug!("set property {} = {}", property.name.clone(), property.value.clone().to_string());
                entity_instance.set_no_propagate(property.name.clone(), property.value.clone());
            }
        }
        // TODO: it's still not a transactional mutation
        entity_instance.tick();
        Ok(entity_instance.into())
    }

    /// Manually tick the entity instance. This means for each property of the entity instance
    /// the corresponding reactive stream will be activated with it's last value.
    ///
    /// This leads to a recalculation if the entity instance is controlled by an behaviour which
    /// consumes the reactive streams.
    ///
    /// Furthermore this leads to a new value propagation if the output property is connected
    /// to other properties.
    async fn tick(&self, context: &Context<'_>, id: Uuid) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_instance = entity_instance_manager.get(id);
        if entity_instance.is_none() {
            return Err(Error::new(format!("Entity instance {} does not exist!", id)));
        }
        let entity_instance = entity_instance.unwrap();
        entity_instance.tick();
        Ok(entity_instance.into())
    }

    /// Deletes an entity instance.
    async fn delete(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The id of the entity instance")] id: Uuid,
        #[graphql(desc = "If true, all relations to and from the entity instance will be deleted as well")] delete_relations: Option<bool>,
    ) -> Result<bool> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        if delete_relations.is_some() && delete_relations.unwrap() {
            let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationInstanceManager>>()?;
            relation_instance_manager.get_by_inbound_entity(id).iter().for_each(|r| {
                relation_instance_manager.delete(r.get_key().unwrap());
            });
            relation_instance_manager.get_by_outbound_entity(id).iter().for_each(|r| {
                relation_instance_manager.delete(r.get_key().unwrap());
            });
        }
        entity_instance_manager.delete(id);
        Ok(true)
    }
}
