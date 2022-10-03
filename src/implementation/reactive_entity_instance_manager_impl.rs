use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use dashmap::DashMap;
use path_tree::PathTree;
use serde_json::Value;
use tokio::runtime::Runtime;
use tokio::time::sleep;
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
use crate::api::SystemEventManager;
use crate::api::SYSTEM_EVENT_PROPERTY_EVENT;
use crate::di::*;
use crate::implementation::reactive_type_identifiers::TypeComponentIdentifier;
use crate::implementation::reactive_type_identifiers::TypePropertyIdentifier;
use crate::model::ComponentContainer;
use crate::model::EntityInstance;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactivePropertyContainer;
use crate::model::TypeContainer;
use crate::plugins::SystemEvent;
use crate::plugins::SystemEventTypes;

static HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED: u128 = 0x6ba7b8109e1513d350b300c04fe530c7;
static HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED: u128 = 0x6ba8b8119e1513d350b300c04fe630c7;
static HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED: u128 = 0x6ba7b8109e2613d350b300c04fe640c7;
static HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED: u128 = 0x7ca8b8119e1523d361b311c050e630c7;

#[wrapper]
pub struct ReactiveEntityInstances(Arc<DashMap<Uuid, Arc<ReactiveEntityInstance>>>);

#[wrapper]
pub struct LabelPathTree(RwLock<PathTree<Uuid>>);

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
fn create_reactive_entity_instances_storage() -> ReactiveEntityInstances {
    ReactiveEntityInstances(Arc::new(DashMap::new()))
}

#[provides]
fn create_label_path_tree() -> LabelPathTree {
    LabelPathTree(RwLock::new(PathTree::<Uuid>::new()))
}

#[provides]
fn create_running_state() -> RunningState {
    RunningState(Arc::new(AtomicBool::new(true)))
}

#[provides]
fn create_system_event_channels() -> SystemEventChannels {
    let mut system_event_channels = HashMap::new();
    system_event_channels.insert(HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED, crossbeam::channel::unbounded());
    system_event_channels.insert(HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED, crossbeam::channel::unbounded());
    system_event_channels.insert(HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED, crossbeam::channel::unbounded());
    system_event_channels.insert(HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED, crossbeam::channel::unbounded());
    SystemEventChannels(system_event_channels)
}

#[provides]
fn create_runtime() -> RuntimeContainer {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("inexor-reim")
        .build()
        .unwrap();
    RuntimeContainer(runtime)
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

    running: RunningState,

    system_event_channels: SystemEventChannels,

    runtime: RuntimeContainer,
    // TODO: Type Cache
}

impl ReactiveEntityInstanceManagerImpl {
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
impl ReactiveEntityInstanceManager for ReactiveEntityInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.entity_instance_manager.has(id) && self.reactive_entity_instances.0.contains_key(&id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instances.0.get(&id).map(|entity_instance| entity_instance.value().clone())
    }

    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveEntityInstance>> {
        let reader = self.label_path_tree.0.read().unwrap();
        reader.find(label).and_then(|result| self.get(*result.0))
    }

    fn get_by_label_with_params(&self, label: &str) -> Option<(Arc<ReactiveEntityInstance>, HashMap<String, String>)> {
        let reader = self.label_path_tree.0.read().unwrap();
        reader.find(label).and_then(|result| match self.get(*result.0) {
            Some(instance) => {
                let params: HashMap<String, String> = result.1.into_iter().map(|(a, b)| (String::from(a), String::from(b))).collect();
                Some((instance, params))
            }
            None => None,
        })
    }

    fn get_entity_instances(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instances.0.iter().map(|e| e.value().clone()).collect()
    }

    fn count_entity_instances(&self) -> usize {
        self.reactive_entity_instances.0.len()
    }

    fn get_ids(&self) -> Vec<Uuid> {
        self.reactive_entity_instances.0.iter().map(|e| e.key().clone()).collect()
    }

    fn create(&self, type_name: &str, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
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
        type_name: &str,
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

    fn add_component(&self, id: Uuid, component_name: &str) {
        if let Some(component) = self.component_manager.get(component_name) {
            if let Some(reactive_entity_instance) = self.get(id) {
                // Add components with properties
                reactive_entity_instance.add_component_with_properties(&component);
                // Add component behaviours
                self.component_behaviour_manager
                    .add_behaviours_to_entity_component(reactive_entity_instance, component);
            }
        }
    }

    fn remove_component(&self, id: Uuid, component_name: &str) {
        if let Some(component) = self.component_manager.get(component_name) {
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
        self.reactive_entity_instances.0.remove(&id);
    }

    fn import(&self, path: &str) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceImportError> {
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

    fn export(&self, id: Uuid, path: &str) {
        if self.has(id) {
            self.commit(id);
            self.entity_instance_manager.export(id, path);
        }
    }

    fn handle_component_added_events(&self) {
        let component_manager = self.component_manager.clone();
        let component_behaviour_manager = self.component_behaviour_manager.clone();
        let reactive_entity_instances = self.reactive_entity_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_id) = TypeComponentIdentifier::try_from(v.clone()) {
                                if let Some(component) = component_manager.get(&type_id.component) {
                                    for instance in reactive_entity_instances
                                        .iter()
                                        .filter(|instance| instance.type_name == type_id.name)
                                        .map(|instance| instance.value().clone())
                                    {
                                        instance.add_component_with_properties(&component);
                                        component_behaviour_manager.add_behaviours_to_entity_component(instance, component.clone());
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            sleep(tokio::time::Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }

    fn handle_component_removed_events(&self) {
        let component_manager = self.component_manager.clone();
        let component_behaviour_manager = self.component_behaviour_manager.clone();
        let reactive_entity_instances = self.reactive_entity_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_id) = TypeComponentIdentifier::try_from(v.clone()) {
                                if let Some(component) = component_manager.get(&type_id.component) {
                                    for reactive_entity_instance in reactive_entity_instances
                                        .iter()
                                        .filter(|entity_instance| entity_instance.type_name == type_id.name)
                                        .map(|entity_instance| entity_instance.value().clone())
                                    {
                                        reactive_entity_instance.remove_component(&component.name);
                                        component_behaviour_manager.remove_behaviours_from_entity_component(reactive_entity_instance, component.clone());
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            sleep(tokio::time::Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }

    fn handle_property_added_events(&self) {
        let entity_type_manager = self.entity_type_manager.clone();
        let reactive_entity_instances = self.reactive_entity_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_property_id) = TypePropertyIdentifier::try_from(v.clone()) {
                                if let Some(entity_type) = entity_type_manager.get(&type_property_id.name) {
                                    for instance in reactive_entity_instances
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
                            sleep(tokio::time::Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }

    fn handle_property_removed_events(&self) {
        let reactive_entity_instances = self.reactive_entity_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(v) => {
                            if let Ok(type_property_id) = TypePropertyIdentifier::try_from(v.clone()) {
                                for instance in reactive_entity_instances
                                    .iter()
                                    .filter(|instance| instance.type_name == type_property_id.name)
                                    .map(|instance| instance.value().clone())
                                {
                                    instance.remove_property(&type_property_id.property_name);
                                }
                            }
                        }
                        Err(_) => {
                            sleep(tokio::time::Duration::from_millis(100)).await;
                        }
                    }
                }
            });
        }
    }
}

impl Lifecycle for ReactiveEntityInstanceManagerImpl {
    fn post_init(&self) {
        for event_instance in self.event_manager.get_system_event_instances() {
            self.register_reactive_instance(event_instance);
        }

        self.subscribe_system_event(SystemEventTypes::EntityTypeComponentAdded, HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED);
        self.subscribe_system_event(SystemEventTypes::EntityTypeComponentRemoved, HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED);
        self.subscribe_system_event(SystemEventTypes::EntityTypePropertyAdded, HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED);
        self.subscribe_system_event(SystemEventTypes::EntityTypePropertyRemoved, HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED);

        self.handle_component_added_events();
        self.handle_component_removed_events();
        self.handle_property_added_events();
        self.handle_property_removed_events();
    }

    fn pre_shutdown(&self) {
        self.running.0.store(false, Ordering::Relaxed);

        self.subscribe_system_event(SystemEventTypes::EntityTypePropertyRemoved, HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED);
        self.subscribe_system_event(SystemEventTypes::EntityTypePropertyAdded, HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED);
        self.unsubscribe_system_event(SystemEventTypes::EntityTypeComponentRemoved, HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED);
        self.unsubscribe_system_event(SystemEventTypes::EntityTypeComponentAdded, HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED);
        for event_instance in self.event_manager.get_system_event_instances() {
            self.unregister_reactive_instance(event_instance.id);
        }
    }
}
