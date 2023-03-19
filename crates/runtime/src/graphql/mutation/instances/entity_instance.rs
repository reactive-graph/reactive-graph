use std::sync::Arc;

use async_graphql::*;
use log::debug;
use serde_json::json;
use uuid::Uuid;

use crate::api::EntityBehaviourManager;
use crate::api::EntityComponentBehaviourManager;
use crate::api::EntityTypeManager;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::graphql::mutation::BehaviourTypeIdDefinition;
use crate::graphql::mutation::ComponentTypeIdDefinition;
use crate::graphql::mutation::EntityTypeIdDefinition;
use crate::graphql::query::GraphQLEntityInstance;
use crate::graphql::query::GraphQLPropertyInstance;
use crate::model::BehaviourTypeId;
use crate::model::PropertyInstanceSetter;
use crate::model::ReactivePropertyContainer;
use crate::model_runtime::PROPERTY_TRIGGER;

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
        #[graphql(name = "type", desc = "The entity type")] entity_ty: EntityTypeIdDefinition,
        #[graphql(desc = "The id of the entity instance. If none is given a random uuid will be generated.")] id: Option<Uuid>,
        #[graphql(desc = "Creates the entity instance with the given components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager>>()?;

        let entity_ty = entity_ty.into();
        let entity_type = entity_type_manager
            .get(&entity_ty)
            .ok_or(Error::new(format!("Entity type {} does not exist", entity_ty)))?;

        let properties = GraphQLPropertyInstance::to_map_with_defaults(properties, entity_type.properties);

        let entity_instance = match id {
            Some(id) => entity_instance_manager.create_with_id(&entity_ty, id, properties),
            None => entity_instance_manager.create(&entity_ty, properties),
        };
        match entity_instance {
            Ok(entity_instance) => {
                if let Some(components) = components {
                    for component in components {
                        // TODO: handle the case when one or multiple components wasn't added
                        // How to handle this?
                        let component_ty = component.into();
                        let _ = entity_instance_manager.add_component(entity_instance.id, &component_ty);
                    }
                }
                Ok(entity_instance.into())
            }
            Err(e) => Err(Error::new(format!("Failed to create entity instance: {}", e))),
        }
    }

    /// Updates the properties of the entity instance with the given id.
    async fn update(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Updates the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Updates the entity instance with the given label.")] label: Option<String>,
        #[graphql(desc = "Adds the given components.")] add_components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Removes the given components.")] remove_components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Updates the given properties")] properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_instance;
        if id.is_some() {
            entity_instance = entity_instance_manager.get(id.unwrap());
        } else if label.is_some() {
            entity_instance = entity_instance_manager.get_by_label(label.unwrap().as_str());
        } else {
            return Err("Either id or label must be given!".into());
        }
        if entity_instance.is_none() {
            return Err("Entity instance not found!".into());
        }
        let entity_instance = entity_instance.unwrap();

        if let Some(components) = add_components {
            for component in components {
                // TODO: handle the case when one or multiple components wasn't added
                let component = component.into();
                let _ = entity_instance_manager.add_component(entity_instance.id, &component);
            }
        }
        if let Some(components) = remove_components {
            for component in components {
                let component = component.into();
                entity_instance_manager.remove_component(entity_instance.id, &component);
            }
        }
        if let Some(properties) = properties {
            // fill all values first without propagation
            for property in properties.clone() {
                debug!("set property {} = {}", property.name.clone(), property.value.clone().to_string());
                // Set with respect to the mutability state
                entity_instance.set_no_propagate_checked(property.name.clone(), property.value.clone());
            }
            // tick every property that has been changed before, this is still not transactional
            for property in properties {
                debug!("tick property {} = {}", property.name.clone(), property.value.clone().to_string());
                if let Some(property_instance) = entity_instance.properties.get(property.name.as_str()) {
                    // Tick with respect to the mutability state
                    property_instance.tick_checked();
                }
            }
        }
        // TODO: it's still not a transactional mutation
        // entity_instance.tick();
        Ok(entity_instance.into())
    }

    /// Triggers the entity instance with the given id.
    async fn trigger(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Triggers the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Triggers the entity instance with the given label.")] label: Option<String>,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let Some(entity_instance) = (if id.is_some() {
            entity_instance_manager.get(id.unwrap())
        } else if label.is_some() {
            entity_instance_manager.get_by_label(label.unwrap().as_str())
        } else {
            return Err("Either id or label must be given!".into());
        }) else {
            return Err("Entity instance not found!".into());
        };
        if entity_instance.has_property(PROPERTY_TRIGGER) {
            entity_instance.set_checked(PROPERTY_TRIGGER, json!(true));
            Ok(entity_instance.into())
        } else {
            Err(Error::new(format!("Unable to trigger {}", entity_instance.id)))
        }
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
                relation_instance_manager.delete(&r.get_key());
            });
            relation_instance_manager.get_by_outbound_entity(id).iter().for_each(|r| {
                relation_instance_manager.delete(&r.get_key());
            });
        }
        entity_instance_manager.delete(id);
        Ok(true)
    }

    async fn connect(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(name = "type")] behaviour_ty: BehaviourTypeIdDefinition,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager>>()?;
        let reactive_instance = entity_instance_manager.get(id).ok_or(Error::new("Entity instance not found"))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if entity_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_behaviour_manager
                .connect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to connect entity behaviour {:?}", e)))?;
        }
        if entity_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_component_behaviour_manager
                .connect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to connect entity component behaviour {:?}", e)))?;
        }
        Ok(reactive_instance.into())
    }

    async fn disconnect(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(name = "type")] behaviour_ty: BehaviourTypeIdDefinition,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager>>()?;
        let reactive_instance = entity_instance_manager.get(id).ok_or(Error::new("Entity instance not found"))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if entity_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_behaviour_manager
                .disconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to disconnect entity behaviour {:?}", e)))?;
        }
        if entity_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_component_behaviour_manager
                .disconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to connect entity component behaviour {:?}", e)))?;
        }
        Ok(reactive_instance.into())
    }

    async fn reconnect(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(name = "type")] behaviour_ty: BehaviourTypeIdDefinition,
    ) -> Result<GraphQLEntityInstance> {
        let entity_instance_manager = context.data::<Arc<dyn ReactiveEntityInstanceManager>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager>>()?;
        let reactive_instance = entity_instance_manager.get(id).ok_or(Error::new("Entity instance not found"))?;
        let behaviour_ty = BehaviourTypeId::from(behaviour_ty);
        if entity_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_behaviour_manager
                .reconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to reconnect entity behaviour {:?}", e)))?;
        }
        if entity_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_component_behaviour_manager
                .reconnect(reactive_instance.clone(), &behaviour_ty)
                .map_err(|e| Error::new(format!("Failed to connect entity component behaviour {:?}", e)))?;
        }
        Ok(reactive_instance.into())
    }
}
