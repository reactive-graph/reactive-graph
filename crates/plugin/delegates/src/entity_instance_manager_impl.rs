use std::collections::HashMap;
use std::sync::Arc;

use uuid::Uuid;

use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityComponentAddError;
use reactive_graph_reactive_service_api::ReactiveEntityCreationError;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveEntityRegistrationError;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;

pub struct EntityInstanceManagerDelegate {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,
}

impl EntityInstanceManagerDelegate {
    pub fn new(
        component_manager: Arc<dyn ComponentManager + Send + Sync>,
        entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
        reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,
    ) -> Self {
        Self {
            component_manager,
            entity_type_manager,
            reactive_entity_manager,
        }
    }
}

impl reactive_graph_plugin_api::EntityInstanceManager for EntityInstanceManagerDelegate {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_entity_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<ReactiveEntity> {
        self.reactive_entity_manager.get(id)
    }

    fn get_by_label(&self, label: &str) -> Option<ReactiveEntity> {
        self.reactive_entity_manager.get_by_label(label)
    }

    fn get_by_label_with_params(&self, label: &str) -> Option<(ReactiveEntity, HashMap<String, String>)> {
        self.reactive_entity_manager.get_by_label_with_params(label)
    }

    fn get_all(&self) -> Vec<ReactiveEntity> {
        self.reactive_entity_manager.get_all()
    }

    fn get_by_type(&self, ty: &EntityTypeId) -> Vec<ReactiveEntity> {
        self.reactive_entity_manager.get_by_type(ty)
    }

    fn get_ids(&self) -> Vec<Uuid> {
        self.reactive_entity_manager.get_ids()
    }

    fn count(&self) -> usize {
        self.reactive_entity_manager.count()
    }

    fn count_by_type(&self, ty: &EntityTypeId) -> usize {
        self.reactive_entity_manager.count_by_type(ty)
    }

    fn count_by_component(&self, component: &ComponentTypeId) -> usize {
        self.reactive_entity_manager.count_by_component(component)
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_entity_manager.count_by_behaviour(behaviour_ty)
    }

    fn create(&self, entity_instance: EntityInstance) -> Result<ReactiveEntity, ReactiveEntityCreationError> {
        match self.entity_type_manager.get(&entity_instance.ty) {
            Some(entity_type) => {
                let entity_instance = entity_instance;
                // Add properties from entity type if not existing
                for (property_name, property_type) in entity_type.properties {
                    if !entity_instance.properties.contains_key(&property_name) {
                        entity_instance
                            .properties
                            .insert(property_name.clone(), property_type.data_type.default_value());
                    }
                }
                // Add properties from components if not existing
                for component in entity_type.components.iter() {
                    if let Some(component) = self.component_manager.get(component.key()) {
                        for (property_name, property_type) in component.properties {
                            if !entity_instance.properties.contains_key(&property_name) {
                                entity_instance
                                    .properties
                                    .insert(property_name.clone(), property_type.data_type.default_value());
                            }
                        }
                    }
                }
                self.reactive_entity_manager
                    .create_with_id(&entity_instance.ty, entity_instance.id, entity_instance.properties)
            }
            None => Err(ReactiveEntityCreationError::ReactiveEntityRegistrationError(
                ReactiveEntityRegistrationError::UnknownEntityType(entity_instance.ty.clone()),
            )),
        }
    }

    fn register(&self, reactive_entity: ReactiveEntity) -> Result<ReactiveEntity, ReactiveEntityRegistrationError> {
        self.reactive_entity_manager.register_reactive_instance(reactive_entity)
    }

    fn add_component(&self, id: Uuid, component: &ComponentTypeId) -> Result<(), ReactiveEntityComponentAddError> {
        self.reactive_entity_manager.add_component(id, component)
    }

    fn remove_component(&self, id: Uuid, component: &ComponentTypeId) {
        self.reactive_entity_manager.remove_component(id, component);
    }

    fn delete(&self, id: Uuid) -> bool {
        self.reactive_entity_manager.delete(id)
    }
}
