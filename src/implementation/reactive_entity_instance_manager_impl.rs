use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use path_tree::PathTree;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::api::ComponentBehaviourManager;
use crate::api::ComponentManager;
use crate::api::EntityBehaviourManager;
use crate::api::EntityInstanceManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceCreationError;
use crate::api::ReactiveEntityInstanceImportError;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::SystemEvent;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::model::ComponentContainer;
use crate::model::EntityInstance;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactivePropertyInstance;

#[wrapper]
pub struct ReactiveEntityInstances(RwLock<BTreeMap<Uuid, Arc<ReactiveEntityInstance>>>);

#[wrapper]
pub struct LabelPathTree(RwLock<PathTree<Uuid>>);

#[provides]
fn create_reactive_entity_instances_storage() -> ReactiveEntityInstances {
    ReactiveEntityInstances(RwLock::new(BTreeMap::new()))
}

#[provides]
fn create_label_path_tree() -> LabelPathTree {
    LabelPathTree(RwLock::new(PathTree::<Uuid>::new()))
}

#[component]
pub struct ReactiveEntityInstanceManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    entity_instance_manager: Wrc<dyn EntityInstanceManager>,

    component_behaviour_manager: Wrc<dyn ComponentBehaviourManager>,

    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,

    reactive_entity_instances: ReactiveEntityInstances,

    label_path_tree: LabelPathTree,
    // TODO: Type Cache
}

#[async_trait]
#[provides]
impl ReactiveEntityInstanceManager for ReactiveEntityInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.entity_instance_manager.has(id) && self.reactive_entity_instances.0.read().unwrap().contains_key(&id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.reactive_entity_instances.0.read().unwrap();
        reader.get(&id).cloned()
    }

    fn get_by_label(&self, label: String) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.label_path_tree.0.read().unwrap();
        reader.find(label.as_str()).and_then(|result| self.get(*result.0))
    }

    fn get_by_label_with_params(&self, label: String) -> Option<(Arc<ReactiveEntityInstance>, HashMap<String, String>)> {
        let reader = self.label_path_tree.0.read().unwrap();
        reader.find(label.as_str()).and_then(|result| match self.get(*result.0) {
            Some(instance) => {
                let params: HashMap<String, String> = result.1.into_iter().map(|(a, b)| (String::from(a), String::from(b))).collect();
                Some((instance, params))
            }
            None => None,
        })
    }

    fn get_entity_instances(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        let reader = self.reactive_entity_instances.0.read().unwrap();
        reader.values().cloned().collect()
    }

    fn count_entity_instances(&self) -> usize {
        self.reactive_entity_instances.0.read().unwrap().len()
    }

    fn get_ids(&self) -> Vec<Uuid> {
        let reader = self.reactive_entity_instances.0.read().unwrap();
        reader.keys().cloned().collect()
    }

    fn create(&self, type_name: String, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        let result = self.entity_instance_manager.create(type_name, properties);
        if result.is_err() {
            return Err(ReactiveEntityInstanceCreationError::EntityInstanceCreationError(result.err().unwrap()));
        }
        if let Some(entity_instance) = self.entity_instance_manager.get(result.unwrap()) {
            return self.create_reactive_instance(entity_instance);
        }
        Err(ReactiveEntityInstanceCreationError::MissingInstance)
    }

    fn create_with_id(
        &self,
        type_name: String,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        if self.has(id) {
            return Err(ReactiveEntityInstanceCreationError::UuidTaken(id));
        }
        let entity_instance = self.entity_instance_manager.get(id);
        match entity_instance {
            Some(entity_instance) => {
                // TODO: update properties first?
                self.create_reactive_instance(entity_instance)
            }
            None => {
                let result = self.entity_instance_manager.create_with_id(type_name, id, properties);
                if result.is_err() {
                    return Err(ReactiveEntityInstanceCreationError::EntityInstanceCreationError(result.err().unwrap()));
                }
                if let Some(entity_instance) = self.entity_instance_manager.get(id) {
                    return self.create_reactive_instance(entity_instance);
                }
                Err(ReactiveEntityInstanceCreationError::MissingInstance)
            }
        }
    }

    fn create_reactive_instance(&self, entity_instance: EntityInstance) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        let reactive_entity_instance = Arc::new(ReactiveEntityInstance::from(entity_instance));
        self.register_reactive_instance(reactive_entity_instance.clone());
        Ok(reactive_entity_instance)
    }

    fn register_reactive_instance(&self, reactive_entity_instance: Arc<ReactiveEntityInstance>) {
        // TODO: propagate error if create wasn't successful
        let _result = self.entity_instance_manager.create_from_instance(reactive_entity_instance.clone().into());
        self.reactive_entity_instances
            .0
            .write()
            .unwrap()
            .insert(reactive_entity_instance.id, reactive_entity_instance.clone());
        // Apply all components that are predefined in the entity type
        if let Some(components) = self
            .entity_type_manager
            .get(&reactive_entity_instance.type_name)
            .map(|entity_type| entity_type.components)
        {
            components.iter().for_each(|component| {
                reactive_entity_instance.components.insert(component.clone());
            });
        }
        // Add component behaviours
        self.component_behaviour_manager.add_behaviours_to_entity(reactive_entity_instance.clone());
        // Add entity behaviours
        self.entity_behaviour_manager.add_behaviours(reactive_entity_instance.clone());
        // Register label
        if let Some(value) = reactive_entity_instance.get("label") {
            if !value.is_string() {
                return;
            }
            let mut writer = self.label_path_tree.0.write().unwrap();
            writer.insert(value.as_str().unwrap(), reactive_entity_instance.id);
        }
        self.event_manager.emit_event(SystemEvent::EntityInstanceCreated(reactive_entity_instance.id))
    }

    fn register_or_merge_reactive_instance(&self, reactive_entity_instance: Arc<ReactiveEntityInstance>) -> Arc<ReactiveEntityInstance> {
        if !self.has(reactive_entity_instance.id) {
            // No instance exists with the given uuid: register as new instance and return it
            self.register_reactive_instance(reactive_entity_instance.clone());
            reactive_entity_instance
        } else {
            // Instance with the given uuid exists: don't register but return the existing instance instead
            self.get(reactive_entity_instance.id).unwrap()
        }
    }

    fn add_component(&self, id: Uuid, component_name: String) {
        if let Some(component) = self.component_manager.get(&component_name) {
            if let Some(reactive_entity_instance) = self.get(id) {
                // Add component
                reactive_entity_instance.add_component(component_name);
                // Add component properties which doesn't exist yet
                for property in component.properties.iter() {
                    let property_name = property.name.clone();
                    if !reactive_entity_instance.properties.contains_key(property_name.as_str()) {
                        let property_instance = ReactivePropertyInstance::new(reactive_entity_instance.id, property_name.clone(), json!(0));
                        reactive_entity_instance.properties.insert(property_name, property_instance);
                    }
                }
                // Add component behaviours
                self.component_behaviour_manager
                    .add_behaviours_to_entity_component(reactive_entity_instance, component);
            }
        }
    }

    fn remove_component(&self, id: Uuid, component_name: String) {
        if let Some(component) = self.component_manager.get(&component_name) {
            if let Some(reactive_entity_instance) = self.get(id) {
                // Remove component
                reactive_entity_instance.remove_component(component_name);
                // We do not remove properties because we cannot asure that the removal is intended
                // Remove component behaviours
                self.component_behaviour_manager
                    .remove_behaviours_from_entity_component(reactive_entity_instance, component);
            }
        }
    }

    fn commit(&self, id: Uuid) {
        if let Some(reactive_entity_instance) = self.get(id) {
            self.entity_instance_manager.commit(reactive_entity_instance.into());
        }
    }

    // TODO: Important: Check if the entity is part of relations
    // TODO: Return true only if the entity instance has been deleted successfully
    fn delete(&self, id: Uuid) {
        if self.has(id) {
            // TODO: check for relations
            self.unregister_reactive_instance(id);
        }
        // TODO: remove label
        self.entity_instance_manager.delete(id);
        self.event_manager.emit_event(SystemEvent::EntityInstanceDeleted(id))
    }

    // TODO: fn delete_and_delete_relations(&self, id: Uuid) {}

    fn unregister_reactive_instance(&self, id: Uuid) {
        match self.get(id) {
            Some(entity_instance) => {
                self.entity_behaviour_manager.remove_behaviours(entity_instance);
            }
            None => {
                self.entity_behaviour_manager.remove_behaviours_by_id(id);
            }
        }
        let id = &id;
        self.reactive_entity_instances.0.write().unwrap().remove(id);
    }

    fn import(&self, path: String) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceImportError> {
        match self.entity_instance_manager.import(path) {
            Ok(uuid) => match self.entity_instance_manager.get(uuid) {
                Some(entity_instance) => match self.create_reactive_instance(entity_instance) {
                    Ok(reactive_entity_instance) => Ok(reactive_entity_instance),
                    Err(error) => Err(ReactiveEntityInstanceImportError::ReactiveEntityInstanceCreation(error)),
                },
                None => Err(ReactiveEntityInstanceImportError::MissingEntityInstance(uuid)),
            },
            Err(error) => Err(ReactiveEntityInstanceImportError::EntityInstanceImport(error)),
        }
    }

    fn export(&self, id: Uuid, path: String) {
        if self.has(id) {
            self.commit(id);
            self.entity_instance_manager.export(id, path);
        }
    }
}

impl Lifecycle for ReactiveEntityInstanceManagerImpl {
    fn post_init(&self) {
        for event_instance in self.event_manager.get_system_event_instances() {
            self.register_reactive_instance(event_instance);
        }
    }

    fn pre_shutdown(&self) {
        for event_instance in self.event_manager.get_system_event_instances() {
            self.unregister_reactive_instance(event_instance.id);
        }
    }
}
