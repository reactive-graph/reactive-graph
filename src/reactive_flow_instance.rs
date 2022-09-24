use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::ReactivePropertyContainer;

use indradb::EdgeKey;
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::FlowInstance;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::ReactiveEntityInstance;
use crate::ReactiveRelationInstance;

#[derive(Debug)]
pub enum ReactiveFlowInstanceConstructionError {
    MissingWrapperInstance,
    MissingOutboundEntityInstance(Uuid),
    MissingInboundEntityInstance(Uuid),
}

impl fmt::Display for ReactiveFlowInstanceConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ReactiveFlowInstanceConstructionError::MissingWrapperInstance => {
                write!(f, "Missing the wrapper entity instance. Check if an entity instance exists with the same id as the flow id")
            }
            ReactiveFlowInstanceConstructionError::MissingOutboundEntityInstance(id) => write!(f, "The outbound entity instance {} cannot be found", id),
            ReactiveFlowInstanceConstructionError::MissingInboundEntityInstance(id) => write!(f, "The inbound entity instance {} cannot be found", id),
        }
    }
}

pub struct ReactiveFlowInstance {
    /// The id of the flow corresponds to the id of the wrapper entity instance.
    pub id: Uuid,

    /// The entity type of the flow.
    pub type_name: String,

    /// The flow contains entity instances. The entity instance may also
    /// be contained in other flows.
    pub entity_instances: RwLock<HashMap<Uuid, Arc<ReactiveEntityInstance>>>,

    /// The flow contains relation instances. The relation instances may also
    /// be contained in other flows.
    pub relation_instances: RwLock<HashMap<EdgeKey, Arc<ReactiveRelationInstance>>>,

    /// List of entities that has been added since creation of the flow.
    pub entities_added: RwLock<Vec<Uuid>>,

    /// List of entities that has been removed since creation of the flow.
    pub entities_removed: RwLock<Vec<Uuid>>,

    /// List of relations that has been added since creation of the flow.
    pub relations_added: RwLock<Vec<EdgeKey>>,

    /// List of relations that has been removed since creation of the flow.
    pub relations_removed: RwLock<Vec<EdgeKey>>,
}

impl ReactiveFlowInstance {
    pub fn new(wrapper_entity_instance: Arc<ReactiveEntityInstance>) -> ReactiveFlowInstance {
        let type_name = wrapper_entity_instance.type_name.clone();
        let mut entity_instances = HashMap::new();
        entity_instances.insert(wrapper_entity_instance.id, wrapper_entity_instance.clone());
        ReactiveFlowInstance {
            id: wrapper_entity_instance.id,
            type_name,
            entity_instances: RwLock::new(entity_instances),
            relation_instances: RwLock::new(HashMap::new()),
            // wrapper,
            entities_added: RwLock::new(Vec::new()),
            entities_removed: RwLock::new(Vec::new()),
            relations_added: RwLock::new(Vec::new()),
            relations_removed: RwLock::new(Vec::new()),
        }
    }

    pub fn has_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) -> bool {
        self.entity_instances.read().unwrap().contains_key(&entity_instance.id)
    }

    pub fn has_entity_by_id(&self, id: Uuid) -> bool {
        self.entity_instances.read().unwrap().contains_key(&id)
    }

    pub fn get_entity(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.entity_instances.read().unwrap();
        reader.get(&id).cloned()
    }

    pub fn get_wrapper_entity_instance(&self) -> Option<Arc<ReactiveEntityInstance>> {
        self.get_entity(self.id)
    }

    pub fn add_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if !self.has_entity_by_id(entity_instance.id) {
            self.entity_instances.write().unwrap().insert(entity_instance.id, entity_instance.clone());
            self.entities_added.write().unwrap().push(entity_instance.id);
            // self.entities_removed.write().unwrap().remove(entity_instance.id);
        }
    }

    pub fn remove_entity(&self, id: Uuid) {
        self.entity_instances.write().unwrap().remove(&id);
        self.entities_removed.write().unwrap().push(id);
    }

    pub fn has_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) -> bool {
        if let Some(edge_key) = relation_instance.get_key() {
            return self.relation_instances.read().unwrap().contains_key(&edge_key);
        }
        false
    }

    pub fn has_relation_by_key(&self, edge_key: EdgeKey) -> bool {
        self.relation_instances.read().unwrap().contains_key(&edge_key)
    }

    pub fn get_relation(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        let reader = self.relation_instances.read().unwrap();
        reader.get(&edge_key).cloned()
    }

    pub fn add_relation(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        if let Some(edge_key) = relation_instance.get_key() {
            if !self.has_relation_by_key(edge_key.clone()) {
                self.relation_instances.write().unwrap().insert(edge_key.clone(), relation_instance.clone());
                self.relations_added.write().unwrap().push(edge_key);
            }
        }
    }

    pub fn remove_relation(&self, edge_key: EdgeKey) {
        self.relation_instances.write().unwrap().remove(&edge_key);
        self.relations_removed.write().unwrap().push(edge_key);
    }

    pub fn tick(&self) {
        let reader = self.entity_instances.read().unwrap();
        for (_, entity_instance) in reader.iter() {
            entity_instance.tick();
        }
    }
}

impl From<Arc<ReactiveEntityInstance>> for ReactiveFlowInstance {
    fn from(wrapper_entity_instance: Arc<ReactiveEntityInstance>) -> Self {
        ReactiveFlowInstance::new(wrapper_entity_instance)
    }
}

impl TryFrom<FlowInstance> for ReactiveFlowInstance {
    type Error = ReactiveFlowInstanceConstructionError;

    fn try_from(flow: FlowInstance) -> Result<Self, ReactiveFlowInstanceConstructionError> {
        let flow_id = flow.id;
        let mut entity_instances = HashMap::new();
        let mut wrapper = None;
        for entity_instance in flow.entity_instances {
            let id = entity_instance.id;
            let reactive_entity_instance = Arc::new(ReactiveEntityInstance::from(entity_instance));
            entity_instances.insert(id, reactive_entity_instance.clone());
            if id == flow_id {
                wrapper = Some(reactive_entity_instance.clone());
            }
        }
        if wrapper.is_none() {
            return Err(ReactiveFlowInstanceConstructionError::MissingWrapperInstance);
        }
        let mut relation_instances = HashMap::new();
        for relation_instance in flow.relation_instances {
            if let Some(edge_key) = relation_instance.get_key() {
                let outbound = entity_instances.get(&relation_instance.outbound_id);
                if outbound.is_none() {
                    // outbound entity missing
                    return Err(ReactiveFlowInstanceConstructionError::MissingOutboundEntityInstance(relation_instance.outbound_id));
                }
                let inbound = entity_instances.get(&relation_instance.inbound_id);
                if inbound.is_none() {
                    // inbound entity missing
                    return Err(ReactiveFlowInstanceConstructionError::MissingInboundEntityInstance(relation_instance.inbound_id));
                }
                let outbound = outbound.unwrap().clone();
                let inbound = inbound.unwrap().clone();
                let reactive_relation_instance = Arc::new(ReactiveRelationInstance::from_instance(outbound, inbound, relation_instance.clone()));
                relation_instances.insert(edge_key.clone(), reactive_relation_instance);
            }
        }
        Ok(ReactiveFlowInstance {
            id: flow_id,
            type_name: flow.type_name,
            entity_instances: RwLock::new(entity_instances),
            relation_instances: RwLock::new(relation_instances),
            // wrapper: wrapper.unwrap(),
            entities_added: RwLock::new(Vec::new()),
            entities_removed: RwLock::new(Vec::new()),
            relations_added: RwLock::new(Vec::new()),
            relations_removed: RwLock::new(Vec::new()),
        })
    }
}

impl PropertyInstanceGetter for ReactiveFlowInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.get_entity(self.id).and_then(|e| e.properties.get(&property_name.into()).map(|p| p.get()))
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_bool()))
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_u64()))
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_i64()))
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_f64()))
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_string()))
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_array()))
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.get_entity(self.id)
            .and_then(|e| e.properties.get(&property_name.into()).and_then(|p| p.as_object()))
    }
}

impl PropertyInstanceSetter for ReactiveFlowInstance {
    fn set<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(instance) = instance.properties.get(&property_name.into()) {
                instance.set(value);
            }
        }
    }

    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.get_entity(self.id) {
            if let Some(instance) = instance.properties.get(&property_name.into()) {
                instance.set_no_propagate(value);
            }
        }
    }

    // TODO: fn set(&self, Map<String, Value>
    // TODO: Set values transactional: first set all values internally, then send all affected streams
}
