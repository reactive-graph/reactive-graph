use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use indradb::EdgeKey;
use serde_json::json;
use serde_json::Value;
use uuid::Uuid;

use crate::api::ComponentBehaviourManager;
use crate::api::ComponentManager;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceCreationError;
use crate::api::ReactiveRelationInstanceImportError;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationBehaviourManager;
use crate::api::RelationEdgeManager;
use crate::api::RelationInstanceManager;
use crate::api::SystemEvent;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::model::ReactivePropertyInstance;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstance;

#[wrapper]
pub struct ReactiveRelationInstances(RwLock<BTreeMap<EdgeKey, Arc<ReactiveRelationInstance>>>);

#[provides]
fn create_reactive_relation_instance_storage() -> ReactiveRelationInstances {
    ReactiveRelationInstances(RwLock::new(BTreeMap::new()))
}

#[component]
pub struct ReactiveRelationInstanceManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    relation_edge_manager: Wrc<dyn RelationEdgeManager>,

    relation_instance_manager: Wrc<dyn RelationInstanceManager>,

    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    component_behaviour_manager: Wrc<dyn ComponentBehaviourManager>,

    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,

    reactive_relation_instances: ReactiveRelationInstances,
}

#[async_trait]
#[provides]
impl ReactiveRelationInstanceManager for ReactiveRelationInstanceManagerImpl {
    fn has(&self, edge_key: EdgeKey) -> bool {
        self.relation_instance_manager.has(edge_key.clone()) && self.reactive_relation_instances.0.read().unwrap().contains_key(&edge_key)
    }

    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        let reader = self.reactive_relation_instances.0.read().unwrap();
        reader.get(&edge_key).cloned()
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        let reader = self.reactive_relation_instances.0.read().unwrap();
        self.relation_edge_manager
            .get_by_outbound_entity(outbound_entity_id)
            .iter()
            .filter_map(|edge| reader.get(&edge.key.clone()).cloned())
            .collect()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        let reader = self.reactive_relation_instances.0.read().unwrap();
        self.relation_edge_manager
            .get_by_inbound_entity(inbound_entity_id)
            .iter()
            .filter_map(|edge| reader.get(&edge.key.clone()).cloned())
            .collect()
    }

    fn get_relation_instances(&self) -> Vec<Arc<ReactiveRelationInstance>> {
        let reader = self.reactive_relation_instances.0.read().unwrap();
        reader.values().cloned().collect()
    }

    fn get_keys(&self) -> Vec<EdgeKey> {
        let reader = self.reactive_relation_instances.0.read().unwrap();
        reader.keys().cloned().collect()
    }

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError> {
        let result = self.relation_instance_manager.create(edge_key, properties);
        if result.is_err() {
            return Err(ReactiveRelationInstanceCreationError::RelationInstanceCreationError(result.err().unwrap()));
        }
        if let Some(relation_instance) = self.relation_instance_manager.get(result.unwrap()) {
            return self.create_reactive_instance(relation_instance);
        }
        Err(ReactiveRelationInstanceCreationError::MissingInstance)
    }

    fn create_reactive_instance(&self, relation_instance: RelationInstance) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError> {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return Err(ReactiveRelationInstanceCreationError::InvalidEdgeKey);
        }

        let outbound = self.reactive_entity_instance_manager.get(relation_instance.outbound_id);
        if outbound.is_none() {
            return Err(ReactiveRelationInstanceCreationError::MissingOutboundEntityInstance(relation_instance.outbound_id));
        }
        let inbound = self.reactive_entity_instance_manager.get(relation_instance.inbound_id);
        if outbound.is_none() {
            return Err(ReactiveRelationInstanceCreationError::MissingInboundEntityInstance(relation_instance.inbound_id));
        }

        let outbound = outbound.unwrap();
        let inbound = inbound.unwrap();
        let reactive_relation_instance = Arc::new(ReactiveRelationInstance::from_instance(outbound, inbound, relation_instance));
        self.register_reactive_instance(reactive_relation_instance.clone());
        Ok(reactive_relation_instance)
    }

    fn register_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>) {
        if let Some(edge_key) = reactive_relation_instance.get_key() {
            // TODO: propagate error if create wasn't successful
            let _result = self.relation_instance_manager.create_from_instance(reactive_relation_instance.clone().into());
            self.reactive_relation_instances
                .0
                .write()
                .unwrap()
                .insert(edge_key.clone(), reactive_relation_instance.clone());
            // TODO: List of applied components is empty
            self.component_behaviour_manager.add_behaviours_to_relation(reactive_relation_instance.clone());
            self.relation_behaviour_manager.add_behaviours(reactive_relation_instance);
            self.event_manager.emit_event(SystemEvent::RelationInstanceCreated(edge_key))
        }
    }

    fn register_or_merge_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>) -> Arc<ReactiveRelationInstance> {
        let edge_key = reactive_relation_instance.get_key().unwrap();
        if !self.has(edge_key.clone()) {
            // No instance exists with the given edge key
            self.register_reactive_instance(reactive_relation_instance.clone());
            reactive_relation_instance
        } else {
            // Instance with the given edge key exists. Don't register but return the existing reactive instance instead
            self.get(edge_key).unwrap()
        }
    }

    fn add_component(&self, edge_key: EdgeKey, component_name: String) {
        if let Some(component) = self.component_manager.get(component_name.clone()) {
            if let Some(reactive_relation_instance) = self.get(edge_key) {
                // Add component
                reactive_relation_instance.add_component(component_name);
                // Add component properties which doesn't exist yet
                for property in component.properties.iter() {
                    let property_name = property.name.clone();
                    if !reactive_relation_instance.properties.contains_key(property_name.as_str()) {
                        let property_instance = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(0));
                        reactive_relation_instance.properties.insert(property_name, property_instance);
                    }
                }
                // Add component behaviours
                self.component_behaviour_manager
                    .add_behaviours_to_relation_component(reactive_relation_instance, component);
            }
        }
    }

    fn remove_component(&self, edge_key: EdgeKey, component_name: String) {
        if let Some(component) = self.component_manager.get(component_name.clone()) {
            if let Some(reactive_relation_instance) = self.get(edge_key) {
                // Remove component
                reactive_relation_instance.remove_component(component_name);
                // We do not remove properties because we cannot asure that the removal is intended
                // Remove component behaviours
                self.component_behaviour_manager
                    .remove_behaviours_from_relation_component(reactive_relation_instance, component);
            }
        }
    }

    fn commit(&self, edge_key: EdgeKey) {
        if let Some(reactive_relation_instance) = self.get(edge_key) {
            self.relation_instance_manager.commit(reactive_relation_instance.into());
        }
    }

    fn delete(&self, edge_key: EdgeKey) -> bool {
        if self.has(edge_key.clone()) {
            self.unregister_reactive_instance(edge_key.clone());
        }
        let result = self.relation_instance_manager.delete(edge_key.clone());
        self.event_manager.emit_event(SystemEvent::RelationInstanceDeleted(edge_key));
        result
    }

    fn unregister_reactive_instance(&self, edge_key: EdgeKey) {
        match self.get(edge_key.clone()) {
            Some(relation_instance) => {
                self.relation_behaviour_manager.remove_behaviours(relation_instance);
            }
            None => {
                self.relation_behaviour_manager.remove_behaviours_by_key(edge_key.clone());
            }
        }
        self.reactive_relation_instances.0.write().unwrap().remove(&edge_key);
    }

    fn import(&self, path: String) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceImportError> {
        match self.relation_instance_manager.import(path) {
            Ok(relation_instance) => match self.create_reactive_instance(relation_instance) {
                Ok(reactive_relation_instance) => Ok(reactive_relation_instance),
                Err(error) => Err(ReactiveRelationInstanceImportError::ReactiveRelationInstanceCreation(error)),
            },
            Err(error) => Err(ReactiveRelationInstanceImportError::RelationInstanceImport(error)),
        }
    }

    fn export(&self, edge_key: EdgeKey, path: String) {
        if self.has(edge_key.clone()) {
            self.commit(edge_key.clone());
            self.relation_instance_manager.export(edge_key, path);
        }
    }
}
