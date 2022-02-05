use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};

use crate::di::*;
use async_trait::async_trait;
use inexor_rgf_core_model::PropertyInstanceGetter;
use path_tree::PathTree;
use serde_json::Value;
use uuid::Uuid;

use crate::api::{
    ComponentBehaviourManager, EntityBehaviourManager, EntityInstanceManager, ReactiveEntityInstanceCreationError, ReactiveEntityInstanceImportError,
    ReactiveEntityInstanceManager,
};
use crate::model::{EntityInstance, ReactiveEntityInstance};

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
        self.component_behaviour_manager.add_behaviours_to_entity(reactive_entity_instance.clone());
        self.entity_behaviour_manager.add_behaviours(reactive_entity_instance.clone());
        if let Some(value) = reactive_entity_instance.get("label") {
            if !value.is_string() {
                return;
            }
            let mut writer = self.label_path_tree.0.write().unwrap();
            writer.insert(value.as_str().unwrap(), reactive_entity_instance.id);
        }
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
        self.entity_instance_manager.delete(id);
    }

    // TODO: fn delete_and_delete_relations(&self, id: Uuid) {}

    fn unregister_reactive_instance(&self, id: Uuid) {
        self.entity_behaviour_manager.remove_behaviours_by_id(id);
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
