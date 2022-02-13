use std::sync::Arc;

use async_graphql::*;
use log::debug;
use uuid::Uuid;

use crate::api::{EntityTypeManager, ReactiveEntityInstanceManager};
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
        type_name: String,
        id: Option<Uuid>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        let entity_type = entity_type_manager.get(type_name.clone());

        if entity_type.is_none() {
            return Err(Error::new(format!("Entity type {} does not exist", type_name)));
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
        Ok(entity_instance.into())
    }

    /// Updates the properties of the entity instance with the given id.
    async fn update(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Updates the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Updates the entity instance with the given label.")] label: Option<String>,
        properties: Vec<GraphQLPropertyInstance>,
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

        for property in properties {
            debug!("set property {} = {}", property.name.clone(), property.value.clone().to_string());
            entity_instance.set_no_propagate(property.name.clone(), property.value.clone());
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
    ///
    /// TODO: Check if the entity instance is part of relation instances.
    /// TODO: delete_relations: Option<bool>
    async fn delete(&self, context: &Context<'_>, id: Uuid) -> Result<bool> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        entity_instance_manager.delete(id);
        Ok(true)
    }
}
