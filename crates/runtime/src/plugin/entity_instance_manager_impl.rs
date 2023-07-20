use std::collections::HashMap;
use std::sync::Arc;

use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::ReactiveEntityInstanceComponentAddError;
use crate::api::ReactiveEntityInstanceManager;
use crate::model::BehaviourTypeId;
use crate::model::ComponentTypeId;
use crate::model::EntityInstance;
use crate::model::EntityTypeId;
use crate::model::ReactiveEntityInstance;
use crate::plugins::entity_instance_manager::EntityInstanceComponentAddError;
use crate::plugins::entity_instance_manager::EntityInstanceCreationError;
use crate::plugins::EntityInstanceManager;

pub struct EntityInstanceManagerImpl {
    component_manager: Arc<dyn ComponentManager>,
    entity_type_manager: Arc<dyn EntityTypeManager>,
    reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>,
}

impl EntityInstanceManagerImpl {
    pub fn new(
        component_manager: Arc<dyn ComponentManager>,
        entity_type_manager: Arc<dyn EntityTypeManager>,
        reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>,
    ) -> Self {
        Self {
            component_manager,
            entity_type_manager,
            reactive_entity_instance_manager,
        }
    }
}

impl EntityInstanceManager for EntityInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_entity_instance_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instance_manager.get(id)
    }

    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instance_manager.get_by_label(label)
    }

    fn get_by_label_with_params(&self, label: &str) -> Option<(Arc<ReactiveEntityInstance>, HashMap<String, String>)> {
        self.reactive_entity_instance_manager.get_by_label_with_params(label)
    }

    fn get_all(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instance_manager.get_all()
    }

    fn get_by_type(&self, ty: &EntityTypeId) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instance_manager.get_by_type(ty)
    }

    fn get_ids(&self) -> Vec<Uuid> {
        self.reactive_entity_instance_manager.get_ids()
    }

    fn count(&self) -> usize {
        self.reactive_entity_instance_manager.count()
    }

    fn count_by_type(&self, ty: &EntityTypeId) -> usize {
        self.reactive_entity_instance_manager.count_by_type(ty)
    }

    fn count_by_component(&self, component: &ComponentTypeId) -> usize {
        self.reactive_entity_instance_manager.count_by_component(component)
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_entity_instance_manager.count_by_behaviour(behaviour_ty)
    }

    fn create(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, EntityInstanceCreationError> {
        match self.entity_type_manager.get(&entity_instance.ty) {
            Some(entity_type) => {
                let mut entity_instance = entity_instance;
                // Add properties from entity type if not existing
                for property in entity_type.properties.iter() {
                    if !entity_instance.properties.contains_key(&property.name) {
                        entity_instance.properties.insert(property.name.clone(), property.data_type.default_value());
                    }
                }
                // Add properties from components if not existing
                for component in entity_type.components.iter() {
                    if let Some(component) = self.component_manager.get(component) {
                        for property in component.properties {
                            if !entity_instance.properties.contains_key(&property.name) {
                                entity_instance.properties.insert(property.name.clone(), property.data_type.default_value());
                            }
                        }
                    }
                }
                let reactive_entity_instance =
                    self.reactive_entity_instance_manager
                        .create_with_id(&entity_instance.ty, entity_instance.id, entity_instance.properties);
                match reactive_entity_instance {
                    Ok(reactive_entity_instance) => Ok(reactive_entity_instance),
                    Err(_) => Err(EntityInstanceCreationError::Failed),
                }
            }
            None => Err(EntityInstanceCreationError::Failed),
        }
    }

    fn add_component(&self, id: Uuid, component: &ComponentTypeId) -> Result<(), EntityInstanceComponentAddError> {
        self.reactive_entity_instance_manager.add_component(id, component).map_err(|e| e.into())
    }

    fn remove_component(&self, id: Uuid, component: &ComponentTypeId) {
        self.reactive_entity_instance_manager.remove_component(id, component);
    }

    fn delete(&self, id: Uuid) -> bool {
        self.reactive_entity_instance_manager.delete(id)
    }
}

impl From<ReactiveEntityInstanceComponentAddError> for EntityInstanceComponentAddError {
    fn from(e: ReactiveEntityInstanceComponentAddError) -> Self {
        match e {
            ReactiveEntityInstanceComponentAddError::MissingComponent(component_ty) => EntityInstanceComponentAddError::MissingComponent(component_ty),
            ReactiveEntityInstanceComponentAddError::MissingInstance(id) => EntityInstanceComponentAddError::MissingInstance(id),
        }
    }
}
