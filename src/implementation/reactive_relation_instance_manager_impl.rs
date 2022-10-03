use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use async_trait::async_trait;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use dashmap::DashMap;
use indradb::EdgeKey;
use serde_json::Value;
use tokio::runtime::Runtime;
use tokio::time::sleep;
use tokio::time::Duration;
use uuid::Uuid;

use crate::api::ComponentBehaviourManager;
use crate::api::ComponentManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceCreationError;
use crate::api::ReactiveRelationInstanceImportError;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationBehaviourManager;
use crate::api::RelationEdgeManager;
use crate::api::RelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::api::SYSTEM_EVENT_PROPERTY_EVENT;
use crate::di::*;
use crate::implementation::reactive_type_identifiers::TypeComponentIdentifier;
use crate::implementation::reactive_type_identifiers::TypePropertyIdentifier;
use crate::model::ComponentContainer;
use crate::model::ReactivePropertyContainer;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstance;
use crate::model::TypeContainer;
use crate::plugins::SystemEvent;
use crate::plugins::SystemEventTypes;

static HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED: u128 = 0x6ba7b9210e1513d350b300c04fe530c7;
static HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED: u128 = 0x6ba8b8119e1513ee59b300c04fe630c7;
static HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED: u128 = 0x6bb9b9232e1513d350b300c04fe530c7;
static HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED: u128 = 0x6ba8b8339e1535ee5bd300c0410630c7;

#[wrapper]
pub struct ReactiveRelationInstances(Arc<DashMap<EdgeKey, Arc<ReactiveRelationInstance>>>);

#[wrapper]
pub struct RunningState(Arc<AtomicBool>);

#[wrapper]
pub struct SystemEventChannels(HashMap<u128, (Sender<Value>, Receiver<Value>)>);

impl SystemEventChannels {
    fn sender(&self, handle_id: &u128) -> Option<Sender<Value>> {
        self.0.get(handle_id).map(|channel| channel.0.clone())
    }

    fn receiver(&self, handle_id: &u128) -> Option<Receiver<Value>> {
        self.0.get(handle_id).map(|channel| channel.1.clone())
    }
}

#[wrapper]
pub struct RuntimeContainer(Runtime);

#[provides]
fn create_reactive_relation_instance_storage() -> ReactiveRelationInstances {
    ReactiveRelationInstances(Arc::new(DashMap::new()))
}

#[provides]
fn create_running_state() -> RunningState {
    RunningState(Arc::new(AtomicBool::new(true)))
}

#[provides]
fn create_system_event_channels() -> SystemEventChannels {
    let mut system_event_channels = HashMap::new();
    system_event_channels.insert(HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED, crossbeam::channel::unbounded());
    system_event_channels.insert(HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED, crossbeam::channel::unbounded());
    system_event_channels.insert(HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED, crossbeam::channel::unbounded());
    system_event_channels.insert(HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED, crossbeam::channel::unbounded());
    SystemEventChannels(system_event_channels)
}

#[provides]
fn create_runtime() -> RuntimeContainer {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("inexor-system-event-")
        .build()
        .unwrap();
    RuntimeContainer(runtime)
}

#[component]
pub struct ReactiveRelationInstanceManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    relation_edge_manager: Wrc<dyn RelationEdgeManager>,

    relation_instance_manager: Wrc<dyn RelationInstanceManager>,

    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    component_behaviour_manager: Wrc<dyn ComponentBehaviourManager>,

    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,

    reactive_relation_instances: ReactiveRelationInstances,

    running: RunningState,

    system_event_channels: SystemEventChannels,

    runtime: RuntimeContainer,
}

impl ReactiveRelationInstanceManagerImpl {
    fn subscribe_system_event(&self, system_event_type: SystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.event_manager.get_system_event_instance(system_event_type) {
            if let Some(sender) = self.system_event_channels.sender(&handle_id) {
                entity_instance.observe_with_handle(
                    SYSTEM_EVENT_PROPERTY_EVENT,
                    move |v| {
                        let _ = sender.send(v.clone());
                    },
                    handle_id,
                );
            }
        }
    }

    fn unsubscribe_system_event(&self, system_event_type: SystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.event_manager.get_system_event_instance(system_event_type) {
            entity_instance.remove_observer(SYSTEM_EVENT_PROPERTY_EVENT, handle_id);
        }
    }
}

#[async_trait]
#[provides]
impl ReactiveRelationInstanceManager for ReactiveRelationInstanceManagerImpl {
    fn has(&self, edge_key: EdgeKey) -> bool {
        self.relation_instance_manager.has(edge_key.clone()) && self.reactive_relation_instances.0.contains_key(&edge_key)
    }

    fn get(&self, edge_key: EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instances.0.get(&edge_key).map(|instance| instance.value().clone())
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.relation_edge_manager
            .get_by_outbound_entity(outbound_entity_id)
            .iter()
            .filter_map(|edge| self.reactive_relation_instances.0.get(&edge.key).map(|instance| instance.value().clone()))
            .collect()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.relation_edge_manager
            .get_by_inbound_entity(inbound_entity_id)
            .iter()
            .filter_map(|edge| self.reactive_relation_instances.0.get(&edge.key).map(|instance| instance.value().clone()))
            .collect()
    }

    fn get_relation_instances(&self) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instances.0.iter().map(|instance| instance.value().clone()).collect()
    }

    fn count_relation_instances(&self) -> usize {
        self.reactive_relation_instances.0.len()
    }

    fn get_keys(&self) -> Vec<EdgeKey> {
        self.reactive_relation_instances.0.iter().map(|e| e.key().clone()).collect()
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
        let outbound = self
            .reactive_entity_instance_manager
            .get(relation_instance.outbound_id)
            .ok_or(ReactiveRelationInstanceCreationError::MissingOutboundEntityInstance(relation_instance.outbound_id))?;
        let inbound = self
            .reactive_entity_instance_manager
            .get(relation_instance.inbound_id)
            .ok_or(ReactiveRelationInstanceCreationError::MissingInboundEntityInstance(relation_instance.inbound_id))?;
        let relation_type = self
            .relation_type_manager
            .get_starts_with(&relation_instance.type_name)
            .ok_or_else(|| ReactiveRelationInstanceCreationError::UnknownRelationType(relation_instance.type_name.clone()))?;

        if !relation_type.outbound_type.eq("*")
            && !outbound.type_name.eq(&relation_type.outbound_type)
            && !outbound.components.contains(&relation_type.outbound_type)
        {
            return Err(ReactiveRelationInstanceCreationError::OutboundEntityIsNotOfType(
                outbound.type_name.clone(),
                relation_type.outbound_type,
            ));
        }

        if !relation_type.inbound_type.eq("*")
            && !inbound.type_name.eq(&relation_type.inbound_type)
            && !inbound.components.contains(&relation_type.inbound_type)
        {
            return Err(ReactiveRelationInstanceCreationError::InboundEntityIsNotOfType(
                inbound.type_name.clone(),
                relation_type.inbound_type,
            ));
        }

        let reactive_relation_instance = Arc::new(ReactiveRelationInstance::from_instance(outbound, inbound, relation_instance));
        self.register_reactive_instance(reactive_relation_instance.clone());
        Ok(reactive_relation_instance)
    }

    fn register_reactive_instance(&self, reactive_relation_instance: Arc<ReactiveRelationInstance>) {
        if let Some(edge_key) = reactive_relation_instance.get_key() {
            // TODO: propagate error if create wasn't successful
            let _result = self.relation_instance_manager.create_from_instance(reactive_relation_instance.clone().into());
            self.reactive_relation_instances.0.insert(edge_key.clone(), reactive_relation_instance.clone());
            // Apply all components that are predefined in the relation type
            if let Some(components) = self
                .relation_type_manager
                .get(&reactive_relation_instance.type_name)
                .map(|entity_type| entity_type.components)
            {
                components.iter().for_each(|component| {
                    reactive_relation_instance.components.insert(component.clone());
                });
            }
            // Add component behaviours
            self.component_behaviour_manager.add_behaviours_to_relation(reactive_relation_instance.clone());
            // Add relation behaviours
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

    fn add_component(&self, edge_key: EdgeKey, component_name: &str) {
        if let Some(component) = self.component_manager.get(component_name) {
            if let Some(reactive_relation_instance) = self.get(edge_key) {
                // Add components with properties
                reactive_relation_instance.add_component_with_properties(&component);
                // Add component behaviours
                self.component_behaviour_manager
                    .add_behaviours_to_relation_component(reactive_relation_instance, component);
            }
        }
    }

    fn remove_component(&self, edge_key: EdgeKey, component_name: &str) {
        if let Some(component) = self.component_manager.get(component_name) {
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
        self.reactive_relation_instances.0.remove(&edge_key);
    }

    fn import(&self, path: &str) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceImportError> {
        match self.relation_instance_manager.import(path) {
            Ok(relation_instance) => match self.create_reactive_instance(relation_instance) {
                Ok(reactive_relation_instance) => Ok(reactive_relation_instance),
                Err(error) => Err(ReactiveRelationInstanceImportError::ReactiveRelationInstanceCreation(error)),
            },
            Err(error) => Err(ReactiveRelationInstanceImportError::RelationInstanceImport(error)),
        }
    }

    fn export(&self, edge_key: EdgeKey, path: &str) {
        if self.has(edge_key.clone()) {
            self.commit(edge_key.clone());
            self.relation_instance_manager.export(edge_key, path);
        }
    }

    fn handle_component_added_events(&self) {
        let component_manager = self.component_manager.clone();
        let component_behaviour_manager = self.component_behaviour_manager.clone();
        let reactive_relation_instances = self.reactive_relation_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_id) = TypeComponentIdentifier::try_from(v.clone()) {
                                if let Some(component) = component_manager.get(&type_id.component) {
                                    for instance in reactive_relation_instances
                                        .iter()
                                        .filter(|instance| instance.type_name == type_id.name)
                                        .map(|instance| instance.value().clone())
                                    {
                                        instance.add_component_with_properties(&component);
                                        component_behaviour_manager.add_behaviours_to_relation_component(instance, component.clone());
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }

    fn handle_component_removed_events(&self) {
        let component_manager = self.component_manager.clone();
        let component_behaviour_manager = self.component_behaviour_manager.clone();
        let reactive_relation_instances = self.reactive_relation_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_id) = TypeComponentIdentifier::try_from(v.clone()) {
                                if let Some(component) = component_manager.get(&type_id.component) {
                                    for instance in reactive_relation_instances
                                        .iter()
                                        .filter(|instance| instance.type_name == type_id.name)
                                        .map(|instance| instance.value().clone())
                                    {
                                        instance.remove_component(&component.name);
                                        component_behaviour_manager.remove_behaviours_from_relation_component(instance, component.clone());
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }

    fn handle_property_added_events(&self) {
        let relation_type_manager = self.relation_type_manager.clone();
        let reactive_relation_instances = self.reactive_relation_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_property_id) = TypePropertyIdentifier::try_from(v.clone()) {
                                if let Some(entity_type) = relation_type_manager.get(&type_property_id.name) {
                                    for instance in reactive_relation_instances
                                        .iter()
                                        .filter(|instance| instance.type_name == type_property_id.name)
                                        .map(|instance| instance.value().clone())
                                    {
                                        if let Some(property_type) = entity_type.get_own_property(&type_property_id.property_name) {
                                            instance.add_property_by_type(&property_type);
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }

    fn handle_property_removed_events(&self) {
        let reactive_relation_instances = self.reactive_relation_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_property_id) = TypePropertyIdentifier::try_from(v.clone()) {
                                for instance in reactive_relation_instances
                                    .iter()
                                    .filter(|instance| instance.type_name == type_property_id.name)
                                    .map(|instance| instance.value().clone())
                                {
                                    instance.remove_property(&type_property_id.property_name);
                                }
                            }
                        }
                        Err(_) => {
                            sleep(Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }
}

impl Lifecycle for ReactiveRelationInstanceManagerImpl {
    fn post_init(&self) {
        self.subscribe_system_event(SystemEventTypes::RelationTypeComponentAdded, HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED);
        self.subscribe_system_event(SystemEventTypes::RelationTypeComponentRemoved, HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED);
        self.subscribe_system_event(SystemEventTypes::RelationTypePropertyAdded, HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED);
        self.subscribe_system_event(SystemEventTypes::RelationTypePropertyRemoved, HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED);

        self.handle_component_added_events();
        self.handle_component_removed_events();
        self.handle_property_added_events();
        self.handle_property_removed_events();
    }

    fn pre_shutdown(&self) {
        self.running.0.store(false, Ordering::Relaxed);

        self.unsubscribe_system_event(SystemEventTypes::RelationTypePropertyRemoved, HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED);
        self.unsubscribe_system_event(SystemEventTypes::RelationTypePropertyAdded, HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED);
        self.unsubscribe_system_event(SystemEventTypes::RelationTypeComponentRemoved, HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED);
        self.unsubscribe_system_event(SystemEventTypes::RelationTypeComponentAdded, HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED);
    }
}
