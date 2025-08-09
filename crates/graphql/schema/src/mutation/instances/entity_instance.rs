use std::sync::Arc;

use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use log::debug;
use serde_json::json;
use uuid::Uuid;

use crate::mutation::PropertyTypeDefinitions;
use crate::query::GraphQLEntityInstance;
use crate::query::GraphQLPropertyInstance;
use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_service_api::EntityBehaviourManager;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourManager;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::CreateEntityInstanceError;
use reactive_graph_graph::CreateRelationInstanceError;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeConstructor;
use reactive_graph_graph::NamespacedTypeIdContainer;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_graph::TriggerEntityInstanceError;
use reactive_graph_graph::UpdateEntityInstanceError;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;

#[derive(Default)]
pub struct MutationEntityInstances;

impl MutationEntityInstances {
    fn get_reactive_entity_by_id_or_label(&self, context: &Context<'_>, id: Option<Uuid>, label: Option<String>) -> Result<ReactiveEntity> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        if let Some(id) = id {
            Ok(reactive_entity_manager
                .get(id)
                .ok_or(UpdateEntityInstanceError::EntityInstanceDoesNotExist(id))?)
        } else if let Some(label) = label {
            Ok(reactive_entity_manager
                .get_by_label(label.as_str())
                .ok_or(UpdateEntityInstanceError::EntityInstanceWithLabelDoesNotExist(label.clone()))?)
        } else {
            Err(UpdateEntityInstanceError::EitherUuidOrLabelMustBeGiven.into())
        }
    }
}

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
        #[graphql(name = "type", desc = "The fully qualified namespace of the entity type.")] namespace: String,
        #[graphql(desc = "The id of the entity instance. If none is given a random uuid will be generated.")] id: Option<Uuid>,
        #[graphql(desc = "Name of the entity instance.")] name: Option<String>,
        #[graphql(desc = "Description of the entity instance.")] description: Option<String>,
        #[graphql(desc = "Creates the entity instance with the given components.")] components: Option<Vec<String>>,
        properties: Option<Vec<GraphQLPropertyInstance>>,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_type_manager = context.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;

        let entity_ty = EntityTypeId::parse_namespace(&namespace)?;
        let component_tys = ComponentTypeIds::parse_optional_namespaces(components)?;

        let entity_type = entity_type_manager
            .get(&entity_ty)
            .ok_or(CreateEntityInstanceError::EntityTypeDoesNotExist(entity_ty.clone()))?;
        if !component_manager.has_all(&component_tys) {
            return Err(CreateEntityInstanceError::ComponentsDoesNotExist.into());
        }

        let properties = GraphQLPropertyInstance::to_property_instances_with_defaults(properties, entity_type.properties);

        let entity_instance = EntityInstance::builder()
            .ty(&entity_ty)
            .id(id.unwrap_or(Uuid::new_v4()))
            .name(name.unwrap_or_default())
            .description(description.unwrap_or_default())
            .properties(properties)
            .build();
        let entity_instance = reactive_entity_manager.create_reactive_instance(entity_instance)?;
        for component_ty in component_tys {
            // TODO: handle the case when one or multiple components wasn't added
            let _ = reactive_entity_manager.add_component(entity_instance.id, &component_ty);
        }
        Ok(entity_instance.into())
    }

    /// Updates the properties of the entity instance with the given id.
    #[allow(clippy::too_many_arguments)]
    async fn update(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Updates the entity instance with the given id.")] id: Option<Uuid>,
        #[graphql(desc = "Updates the entity instance with the given label.")] label: Option<String>,
        #[graphql(name = "add_components", desc = "Adds the given components.")] add_component_namespaces: Option<Vec<String>>,
        #[graphql(name = "remove_components", desc = "Removes the given components.")] remove_component_namespaces: Option<Vec<String>>,
        #[graphql(desc = "Updates the given properties")] properties: Option<Vec<GraphQLPropertyInstance>>,
        #[graphql(desc = "Adds the given properties")] add_properties: Option<Vec<crate::mutation::PropertyTypeDefinition>>,
        #[graphql(desc = "Removes the given properties")] remove_properties: Option<Vec<String>>,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let component_manager = context.data::<Arc<dyn ComponentManager + Send + Sync>>()?;

        let add_component_tys = ComponentTypeIds::parse_optional_namespaces(add_component_namespaces)?;
        let remove_component_tys = ComponentTypeIds::parse_optional_namespaces(remove_component_namespaces)?;
        let add_properties = PropertyTypeDefinitions::parse_optional_definitions(add_properties)?;

        if !component_manager.has_all(&add_component_tys) || !component_manager.has_all(&remove_component_tys) {
            return Err(CreateRelationInstanceError::ComponentsDoesNotExist.into());
        }

        let reactive_entity: ReactiveEntity = self.get_reactive_entity_by_id_or_label(context, id, label)?;

        for component in add_component_tys {
            // TODO: handle the case when one or multiple components wasn't added
            let _ = reactive_entity_manager.add_component(reactive_entity.id, &component);
        }
        for component in remove_component_tys {
            reactive_entity_manager.remove_component(reactive_entity.id, &component);
        }

        if let Some(properties) = properties {
            // fill all values first without propagation
            for property in properties.clone() {
                debug!("set property {} = {}", property.name.clone(), property.value.clone());
                // Set with respect to the mutability state
                reactive_entity.set_no_propagate_checked(property.name.clone(), property.value.clone());
            }
            // tick every property that has been changed before, this is still not transactional
            for property in properties {
                debug!("tick property {} = {}", property.name.clone(), property.value.clone());
                if let Some(property_instance) = reactive_entity.properties.get(property.name.as_str()) {
                    // Tick with respect to the mutability state
                    property_instance.tick_checked();
                }
            }
        }
        for property_type in add_properties.iter() {
            // let property_type: PropertyType = property_type.into();
            debug!("add property {} ({})", &property_type.name, &property_type.data_type);
            reactive_entity.add_property_by_type(&property_type);
        }
        if let Some(remove_properties) = remove_properties {
            for property_name in remove_properties.clone() {
                debug!("remove property {}", &property_name);
                reactive_entity.remove_property(property_name);
            }
        }
        // TODO: it's still not a transactional mutation
        // entity_instance.tick();
        Ok(reactive_entity.into())
    }

    /// Triggers the entity instance with the given id.
    async fn trigger(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "Triggers the entity instance with the given id")] id: Option<Uuid>,
        #[graphql(desc = "Triggers the entity instance with the given label")] label: Option<String>,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity = self.get_reactive_entity_by_id_or_label(context, id, label)?;
        if !reactive_entity.has_property(&TRIGGER.property_name()) {
            return Err(TriggerEntityInstanceError::TriggerPropertyMissing(reactive_entity.id).into());
        }
        reactive_entity.set_checked(TRIGGER.property_name(), json!(true));
        Ok(reactive_entity.into())
    }

    /// Manually tick the entity instance. This means for each property of the entity instance
    /// the corresponding reactive stream will be activated with its last value.
    ///
    /// This leads to a recalculation if the entity instance is controlled by a behaviour which
    /// consumes the reactive streams.
    ///
    /// Furthermore, this leads to a new value propagation if the output property is connected
    /// to other properties.
    async fn tick(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The id of the entity instance")] id: Option<Uuid>,
        #[graphql(desc = "The label of the entity instance")] label: Option<String>,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity = self.get_reactive_entity_by_id_or_label(context, id, label)?;
        reactive_entity.tick();
        Ok(reactive_entity.into())
    }

    /// Deletes an entity instance.
    async fn delete(
        &self,
        context: &Context<'_>,
        #[graphql(desc = "The id of the entity instance")] id: Uuid,
        #[graphql(desc = "If true, all relations to and from the entity instance will be deleted as well")] delete_relations: Option<bool>,
    ) -> Result<bool> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        if delete_relations.unwrap_or_default() {
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
        #[graphql(name = "behaviour", desc = "The fully qualified namespace of the behaviour")] behaviour_namespace: String,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager + Send + Sync>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager + Send + Sync>>()?;
        let behaviour_ty = BehaviourTypeId::parse_namespace(&behaviour_namespace)?;
        let reactive_instance = reactive_entity_manager
            .get(id)
            .ok_or(UpdateEntityInstanceError::EntityInstanceDoesNotExist(id))?;
        if entity_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_behaviour_manager.connect(reactive_instance.clone(), &behaviour_ty)?;
        }
        if entity_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_component_behaviour_manager.connect(reactive_instance.clone(), &behaviour_ty)?;
        }
        Ok(reactive_instance.into())
    }

    async fn disconnect(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(name = "behaviour", desc = "The fully qualified namespace of the behaviour")] behaviour_namespace: String,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager + Send + Sync>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager + Send + Sync>>()?;
        let behaviour_ty = BehaviourTypeId::parse_namespace(&behaviour_namespace)?;
        let reactive_instance = reactive_entity_manager
            .get(id)
            .ok_or(UpdateEntityInstanceError::EntityInstanceDoesNotExist(id))?;
        if entity_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_behaviour_manager.disconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        if entity_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_component_behaviour_manager.disconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        Ok(reactive_instance.into())
    }

    async fn reconnect(
        &self,
        context: &Context<'_>,
        id: Uuid,
        #[graphql(name = "behaviour", desc = "The fully qualified namespace of the behaviour")] behaviour_namespace: String,
    ) -> Result<GraphQLEntityInstance> {
        let reactive_entity_manager = context.data::<Arc<dyn ReactiveEntityManager + Send + Sync>>()?;
        let entity_behaviour_manager = context.data::<Arc<dyn EntityBehaviourManager + Send + Sync>>()?;
        let entity_component_behaviour_manager = context.data::<Arc<dyn EntityComponentBehaviourManager + Send + Sync>>()?;
        let behaviour_ty = BehaviourTypeId::parse_namespace(&behaviour_namespace)?;
        let reactive_instance = reactive_entity_manager
            .get(id)
            .ok_or(UpdateEntityInstanceError::EntityInstanceDoesNotExist(id))?;
        if entity_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_behaviour_manager.reconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        if entity_component_behaviour_manager.has(reactive_instance.clone(), &behaviour_ty) {
            entity_component_behaviour_manager.reconnect(reactive_instance.clone(), &behaviour_ty)?;
        }
        Ok(reactive_instance.into())
    }
}
