use std::sync::Arc;

use async_graphql::*;
use log::debug;
use serde_json::json;
use uuid::Uuid;

use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_service_api::EntityBehaviourManager;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourManager;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_type_system_api::EntityTypeManager;

use crate::mutation::BehaviourTypeIdDefinition;
use crate::mutation::ComponentTypeIdDefinition;
use crate::mutation::EntityTypeIdDefinition;
use crate::query::GraphQLEntityInstance;
use crate::query::GraphQLPropertyInstance;

#[derive(Default)]
pub struct MutationEntityInstances;

/// Mutation of entity instances.
#[Object]
impl MutationEntityInstances {
    /// Creates a new entity instance of the given type.
    ///
    /// The entity type must exist.
    ///
    /// Optionally, a UUID can be specified. If no UUID is specified one will be generated
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
        #[graphql(desc = "Description of the entity instance.")] description: Option<String>,
        #[graphql(desc = "Creates the entity instance with the given components.")] components: Option<Vec<ComponentTypeIdDefinition>>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;

        let entity_ty = entity_ty.into();
        let entity_type = entity_type_manager
            .get(&entity_ty)
            .ok_or(Error::new(format!("Entity type {} does not exist", entity_ty)))?;

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, entity_type.properties);

        let entity_instance = EntityInstance::builder()
            .ty(&entity_ty)
            .id(id.unwrap_or(Uuid::new_v4()))
            .description(description.unwrap_or_default())
            .properties(properties)
            .build();
        let entity_instance = reactive_entity_manager.create_reactive_instance(entity_instance);
        match entity_instance {
            Ok(entity_instance) => {
                if let Some(components) = components {
                    for component in components {
                        // TODO: handle the case when one or multiple components wasn't added
                        // How to handle this?
                        let component_ty = component.into();
                        let _ = reactive_entity_manager.add_component(entity_instance.id, &component_ty);
                    }
                }
                Ok(entity_instance.into())
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Updates the properties of the entity instance with the given id.
    #[allow(clippy::too_many_arguments)]
    async fn update(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Updates the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Updates the entity instance with the given label.")] label: Option<String>,
        #[graphql(desc = "Adds the given components.")] add_components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Removes the given components.")] remove_components: Option<Vec<ComponentTypeIdDefinition>>,
        #[graphql(desc = "Updates the given properties")] properties: Option<Vec<GraphQLPropertyInstance>>,
        #[graphql(desc = "Adds the given properties")] add_properties: Option<Vec<crate::mutation::PropertyTypeDefinition>>,
        #[graphql(desc = "Removes the given properties")] remove_properties: Option<Vec<String>>,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_instance;
        if id.is_some() {
            entity_instance = reactive_entity_manager.get(id.unwrap());
        } else if label.is_some() {
            entity_instance = reactive_entity_manager.get_by_label(label.unwrap().as_str());
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
                let _ = reactive_entity_manager.add_component(entity_instance.id, &component);
            }
        }
        if let Some(components) = remove_components {
            for component in components {
                let component = component.into();
                reactive_entity_manager.remove_component(entity_instance.id, &component);
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
        if let Some(add_properties) = add_properties {
            for property_type in add_properties.clone() {
                let property_type: PropertyType = property_type.into();
                debug!("add property {} ({})", &property_type.name, &property_type.data_type);
                entity_instance.add_property_by_type(&property_type);
            }
        }
        if let Some(remove_properties) = remove_properties {
            for property_name in remove_properties.clone() {
                debug!("remove property {}", &property_name);
                entity_instance.remove_property(property_name);
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
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let Some(entity_instance) = (if let Some(id) = id {
            reactive_entity_manager.get(id)
        } else if let Some(label) = label {
            reactive_entity_manager.get_by_label(label.as_str())
        } else {
            return Err("Either id or label must be given!".into());
        }) else {
            return Err("Entity instance not found!".into());
        };
        if entity_instance.has_property(&TRIGGER.property_name()) {
            entity_instance.set_checked(TRIGGER.property_name(), json!(true));
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
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_instance = reactive_entity_manager.get(id);
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
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        if delete_relations.is_some() && delete_relations.unwrap() {
            let relation_instance_manager = context.data::<Arc<dyn ReactiveRelationManager + Send + Sync>>()?;
            relation_instance_manager.get_by_inbound_entity(id).iter().for_each(|reactive_relation| {
                let id = reactive_relation.id();
                relation_instance_manager.delete(&id);
            });
            relation_instance_manager.get_by_outbound_entity(id).iter().for_each(|reactive_relation| {
                let id = reactive_relation.id();
                relation_instance_manager.delete(&id);
            });
        }
        Ok(reactive_entity_manager.delete(id))
    }

    async fn connect(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(name = "type")] behaviour_ty: BehaviourTypeIdDefinition,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager + Send + Sync>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager + Send + Sync>>()?;
        let reactive_instance = reactive_entity_manager.get(id).ok_or(Error::new("Entity instance not found"))?;
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
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager + Send + Sync>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager + Send + Sync>>()?;
        let reactive_instance = reactive_entity_manager.get(id).ok_or(Error::new("Entity instance not found"))?;
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
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager + Send + Sync>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager + Send + Sync>>()?;
        let reactive_instance = reactive_entity_manager.get(id).ok_or(Error::new("Entity instance not found"))?;
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
