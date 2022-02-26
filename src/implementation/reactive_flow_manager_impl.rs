use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::{debug, error};
use path_tree::PathTree;
use uuid::Uuid;

use crate::api::SystemEventManager;
use crate::api::FlowManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowCreationError;
use crate::api::ReactiveFlowImportError;
use crate::api::ReactiveFlowManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::SystemEvent;
use crate::di::*;
use crate::model::Flow;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveFlow;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstance;
use crate::plugins::FlowProvider;

#[wrapper]
pub struct ReactiveFlows(RwLock<BTreeMap<Uuid, Arc<ReactiveFlow>>>);

#[provides]
fn create_reactive_flow_storage() -> ReactiveFlows {
    ReactiveFlows(RwLock::new(BTreeMap::new()))
}

#[wrapper]
pub struct FlowProviders(RwLock<Vec<Arc<dyn FlowProvider>>>);

#[provides]
fn create_flow_providers() -> FlowProviders {
    FlowProviders(RwLock::new(Vec::new()))
}

#[wrapper]
pub struct LabelPathTree(RwLock<PathTree<Uuid>>);

#[provides]
fn create_label_path_tree() -> LabelPathTree {
    LabelPathTree(RwLock::new(PathTree::<Uuid>::new()))
}

#[component]
pub struct ReactiveFlowManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    flow_manager: Wrc<dyn FlowManager>,

    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    reactive_flows: ReactiveFlows,

    flow_providers: FlowProviders,

    label_path_tree: LabelPathTree,
}

#[async_trait]
#[provides]
impl ReactiveFlowManager for ReactiveFlowManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_flows.0.read().unwrap().contains_key(&id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlow>> {
        let reader = self.reactive_flows.0.read().unwrap();
        reader.get(&id).cloned()
    }

    fn get_by_label(&self, label: String) -> Option<Arc<ReactiveFlow>> {
        let reader = self.label_path_tree.0.read().unwrap();
        reader.find(label.as_str()).and_then(|result| self.get(*result.0))
    }

    fn get_all(&self) -> Vec<Arc<ReactiveFlow>> {
        let reader = self.reactive_flows.0.read().unwrap();
        reader.values().into_iter().cloned().collect()
    }

    // fn create(&self, type_name: String, properties: HashMap<String, Value, RandomState>) -> Result<Arc<ReactiveFlow>, ReactiveFlowCreationError> {
    fn create(&self, flow: Flow) -> Result<Arc<ReactiveFlow>, ReactiveFlowCreationError> {
        let reactive_flow = ReactiveFlow::try_from(flow);
        if reactive_flow.is_err() {
            return Err(ReactiveFlowCreationError::ReactiveFlowConstructionError(reactive_flow.err().unwrap()));
        }
        let reactive_flow = reactive_flow.unwrap();
        let reactive_flow = Arc::new(reactive_flow);
        self.register_flow_and_reactive_instances(reactive_flow.clone());
        Ok(reactive_flow)

        // let reactive_flow = ReactiveFlow::try_from(flow);
        // if reactive_flow.is_ok() {
        //     let reactive_flow = Arc::new(reactive_flow.unwrap());
        //     self.register_flow_and_reactive_instances(reactive_flow.clone());
        //     return Ok(reactive_flow.clone());
        // }
        // Err(ReactiveFlowCreationError.into())
    }

    fn register_flow_and_reactive_instances(&self, reactive_flow: Arc<ReactiveFlow>) {
        if !self.has(reactive_flow.id) {
            {
                // Step 1: Register all entity instances (if not already registered by uuid)
                let mut entity_instances = reactive_flow.entity_instances.write().unwrap();
                let mut replaced_entity_instances = HashMap::<Uuid, Arc<ReactiveEntityInstance>>::new();
                for (uuid, entity_instance) in entity_instances.iter() {
                    let entity_instance = self
                        .reactive_entity_instance_manager
                        .register_or_merge_reactive_instance(entity_instance.clone());
                    // Replace the entity instance with the actual registered instance instead
                    replaced_entity_instances.insert(*uuid, entity_instance);
                }

                // Step 2: Replace the entity instances of the flow with the actual registered entity instances
                entity_instances.clear();
                for (uuid, entity_instance) in replaced_entity_instances.iter() {
                    entity_instances.insert(*uuid, entity_instance.clone());
                }

                // Step 3: Recreate the reactive relation instances
                // Because the entity instances might have been replaced by the actual registered entity instances
                let mut relation_instances = reactive_flow.relation_instances.write().unwrap();
                let mut replaced_relation_instances = HashMap::<EdgeKey, Arc<ReactiveRelationInstance>>::new();
                for (edge_key, relation_instance) in relation_instances.iter() {
                    let inbound_id = relation_instance.inbound.id;
                    let outbound_id = relation_instance.outbound.id;

                    let recreated_relation_instance = Arc::new(ReactiveRelationInstance::from_instance(
                        entity_instances.get(&outbound_id).unwrap().clone(),
                        entity_instances.get(&inbound_id).unwrap().clone(),
                        RelationInstance::from(relation_instance.clone()),
                    ));
                    replaced_relation_instances.insert(edge_key.clone(), recreated_relation_instance);
                    // relation_instance.inbound = entity_instances.get(&inbound_id).unwrap().clone();
                    // relation_instance.outbound = entity_instances.get(&outbound_id).unwrap().clone();
                }

                // Step 4: Replace the relation instances of the flow with the recreated relation instances
                relation_instances.clear();
                for (edge_key, relation_instance) in replaced_relation_instances.iter() {
                    relation_instances.insert(edge_key.clone(), relation_instance.clone());
                }

                // Step 5: Register all (recreated) relation instances (if not already registered by edge_key)
                let mut replaced_relation_instances = HashMap::<EdgeKey, Arc<ReactiveRelationInstance>>::new();
                for (edge_key, relation_instance) in relation_instances.iter() {
                    let relation_instance = self
                        .reactive_relation_instance_manager
                        .register_or_merge_reactive_instance(relation_instance.clone());
                    // Replace the relation instance with the actual registered instance
                    replaced_relation_instances.insert(edge_key.clone(), relation_instance);
                }

                // Step 6: Replace the relation instances of the flow with the actual registered relation instances
                relation_instances.clear();
                for (edge_key, relation_instance) in replaced_relation_instances.iter() {
                    relation_instances.insert(edge_key.clone(), relation_instance.clone());
                }
            } // Drop rwlock
            self.register_flow(reactive_flow.clone());
        }
    }

    fn register_flow(&self, reactive_flow: Arc<ReactiveFlow>) {
        if !self.reactive_entity_instance_manager.has(reactive_flow.id) {
            if let Some(wrapper_entity_instance) = reactive_flow.get_entity(reactive_flow.id) {
                self.reactive_entity_instance_manager.register_reactive_instance(wrapper_entity_instance);
            }
        }
        self.reactive_flows.0.write().unwrap().insert(reactive_flow.id, reactive_flow.clone());
        // Register label
        if let Some(value) = reactive_flow.get("label") {
            if let Some(label) = value.as_str() {
                let mut writer = self.label_path_tree.0.write().unwrap();
                writer.insert(label, reactive_flow.id);
            }
        }
        self.event_manager.emit_event(SystemEvent::FlowCreated(reactive_flow.id))
    }

    // TODO: how to detect if the flow has removed an entity? => remove behaviour
    // TODO: how to detect if the flow has removed an relation? => remove behaviour
    fn commit(&self, id: Uuid) {
        if let Some(reactive_flow) = self.get(id) {
            // Unregister removed relations
            for edge_key in reactive_flow.relations_removed.read().unwrap().iter() {
                self.reactive_relation_instance_manager.unregister_reactive_instance(edge_key.clone());
            }
            reactive_flow.relations_removed.write().unwrap().clear();

            // Unregister removed entities
            for id in reactive_flow.entities_removed.read().unwrap().iter() {
                self.reactive_entity_instance_manager.unregister_reactive_instance(*id);
            }
            reactive_flow.entities_removed.write().unwrap().clear();

            // Register added entities
            for id in reactive_flow.entities_added.read().unwrap().iter() {
                if let Some(entity_instance) = reactive_flow.get_entity(*id) {
                    self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
                }
            }
            reactive_flow.entities_added.write().unwrap().clear();

            // Register added relations
            for edge_key in reactive_flow.relations_added.read().unwrap().iter() {
                if let Some(relation_instance) = reactive_flow.get_relation(edge_key.clone()) {
                    self.reactive_relation_instance_manager.register_reactive_instance(relation_instance.clone());
                }
            }
            reactive_flow.relations_added.write().unwrap().clear();

            // for (_, entity_instance) in reactive_flow.entity_instances.read().unwrap().iter() {
            //     if !self.reactive_entity_instance_manager.has(entity_instance.id) {
            //         self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
            //     }
            // }
            // for (_, relation_instance) in reactive_flow.relation_instances.read().unwrap().iter() {
            //     let edge_key = relation_instance.get_key();
            //     if edge_key.is_some() {
            //         let edge_key = edge_key.unwrap();
            //         if !self.reactive_relation_instance_manager.has(edge_key.clone()) {
            //             self.reactive_relation_instance_manager.register_reactive_instance(relation_instance.clone());
            //         }
            //     }
            // }

            if let Ok(flow) = Flow::try_from(reactive_flow) {
                self.flow_manager.commit(flow);
            }
        }
    }

    fn delete(&self, id: Uuid) {
        if self.has(id) {
            let reactive_flow = self.get(id).unwrap();
            for (_, entity_instance) in reactive_flow.entity_instances.read().unwrap().iter() {
                self.reactive_entity_instance_manager.unregister_reactive_instance(entity_instance.id);
            }
            for (_, relation_instance) in reactive_flow.relation_instances.read().unwrap().iter() {
                if let Some(edge_key) = relation_instance.get_key() {
                    self.reactive_relation_instance_manager.unregister_reactive_instance(edge_key);
                }
            }
            self.reactive_flows.0.write().unwrap().remove(&id);
            // TODO: remove label
            self.event_manager.emit_event(SystemEvent::FlowDeleted(id))
        }
    }

    fn import(&self, path: String) -> Result<Arc<ReactiveFlow>, ReactiveFlowImportError> {
        if let Ok(flow) = self.flow_manager.import(path) {
            if let Ok(reactive_flow) = self.create(flow) {
                return Ok(reactive_flow);
            }
        }
        Err(ReactiveFlowImportError)
    }

    fn export(&self, id: Uuid, path: String) {
        if self.has(id) {
            self.commit(id);
            if let Ok(flow) = Flow::try_from(self.get(id).unwrap()) {
                self.flow_manager.export(flow, path);
            }
        }
    }

    fn add_provider(&self, provider: Arc<dyn FlowProvider>) {
        self.flow_providers.0.write().unwrap().push(provider);
    }
}

impl Lifecycle for ReactiveFlowManagerImpl {
    fn init(&self) {
        debug!("Importing provided flows");
        for flow in self.flow_providers.0.read().unwrap().iter() {
            for flow in flow.get_flows() {
                debug!("Creating provided flow {}", flow.id);
                let reactive_flow = self.create(flow.clone());
                match reactive_flow {
                    Ok(reactive_flow) => {
                        let created_flow: Result<Flow, _> = reactive_flow.try_into();
                        match created_flow {
                            Ok(created_flow) => {
                                let json = serde_json::to_string_pretty(&created_flow).unwrap();
                                debug!("Successfully created reactive flow:\r\n{}", json);
                            }
                            Err(err) => {
                                debug!("Successfully created reactive flow {}, but failed to serialize: {:?}", flow.id, err);
                            }
                        }
                    }
                    Err(err) => {
                        error!("Failed to create provided flow {}: {}", flow.id, err);
                    }
                }
            }
        }
    }

    fn post_init(&self) {}

    fn pre_shutdown(&self) {}

    fn shutdown(&self) {
        // self.reactive_flows.0.write().unwrap().clear();
        // self.flow_providers.0.write().unwrap().clear();
    }
}
