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

use crate::api::system_event_subscriber::SystemEventSubscriber;
use crate::api::ComponentManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveRelationInstanceComponentAddError;
use crate::api::ReactiveRelationInstanceCreationError;
use crate::api::ReactiveRelationInstanceImportError;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::ReactiveRelationInstancePropertyAddError;
use crate::api::ReactiveRelationInstancePropertyRemoveError;
use crate::api::ReactiveRelationInstanceRegistrationError;
use crate::api::RelationBehaviourManager;
use crate::api::RelationComponentBehaviourManager;
use crate::api::RelationEdgeManager;
use crate::api::RelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::implementation::PROPERTY_EVENT;
use crate::model::BehaviourTypeId;
use crate::model::ComponentContainer;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactivePropertyContainer;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstance;
use crate::model::RelationTypeId;
use crate::model::TypeContainer;
use crate::model::TypeDefinitionComponent;
use crate::model::TypeDefinitionGetter;
use crate::model::TypeDefinitionProperty;
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

    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,

    relation_component_behaviour_manager: Wrc<dyn RelationComponentBehaviourManager>,

    reactive_relation_instances: ReactiveRelationInstances,

    running: RunningState,

    system_event_channels: SystemEventChannels,

    runtime: RuntimeContainer,
}

impl SystemEventSubscriber for ReactiveRelationInstanceManagerImpl {
    fn subscribe_system_event(&self, system_event_type: SystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.event_manager.get_system_event_instance(system_event_type) {
            if let Some(sender) = self.system_event_channels.sender(&handle_id) {
                entity_instance.observe_with_handle(
                    PROPERTY_EVENT,
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
            entity_instance.remove_observer(PROPERTY_EVENT, handle_id);
        }
    }
}

#[async_trait]
#[provides]
impl ReactiveRelationInstanceManager for ReactiveRelationInstanceManagerImpl {
    fn has(&self, edge_key: &EdgeKey) -> bool {
        self.relation_instance_manager.has(edge_key) && self.reactive_relation_instances.0.contains_key(edge_key)
    }

    fn get(&self, edge_key: &EdgeKey) -> Option<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instances.0.get(&edge_key).map(|r| r.value().clone())
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.relation_edge_manager
            .get_by_outbound_entity(outbound_entity_id)
            .iter()
            .filter_map(|edge| self.reactive_relation_instances.0.get(&edge.key).map(|r| r.value().clone()))
            .collect()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Arc<ReactiveRelationInstance>> {
        self.relation_edge_manager
            .get_by_inbound_entity(inbound_entity_id)
            .iter()
            .filter_map(|edge| self.reactive_relation_instances.0.get(&edge.key).map(|r| r.value().clone()))
            .collect()
    }

    fn get_all(&self) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instances.0.iter().map(|r| r.value().clone()).collect()
    }

    fn get_by_type(&self, ty: &RelationTypeId) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instances
            .0
            .iter()
            .filter(|r| &r.relation_type_id() == ty)
            .map(|r| r.value().clone())
            .collect()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<Arc<ReactiveRelationInstance>> {
        self.reactive_relation_instances
            .0
            .iter()
            .filter(|r| r.namespace() == namespace)
            .map(|r| r.value().clone())
            .collect()
    }

    fn count(&self) -> usize {
        self.reactive_relation_instances.0.len()
    }

    fn count_by_type(&self, ty: &RelationTypeId) -> usize {
        self.reactive_relation_instances.0.iter().filter(|r| &r.relation_type_id() == ty).count()
    }

    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize {
        self.reactive_relation_instances.0.iter().filter(|r| r.is_a(&component_ty)).count()
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_relation_instances.0.iter().filter(|r| r.behaves_as(&behaviour_ty)).count()
    }

    fn get_keys(&self) -> Vec<EdgeKey> {
        self.reactive_relation_instances.0.iter().map(|e| e.key().clone()).collect()
    }

    fn create(&self, edge_key: &EdgeKey, properties: HashMap<String, Value>) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError> {
        match self.relation_instance_manager.create(edge_key, properties) {
            Ok(edge_key) => match self.relation_instance_manager.get(&edge_key) {
                Some(relation_instance) => self.create_reactive_instance(relation_instance),
                None => Err(ReactiveRelationInstanceCreationError::MissingInstance(edge_key)),
            },
            Err(e) => Err(ReactiveRelationInstanceCreationError::RelationInstanceCreationError(e)),
        }
    }

    fn create_reactive_instance(
        &self,
        reactive_relation_instance: RelationInstance,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceCreationError> {
        let outbound = self
            .reactive_entity_instance_manager
            .get(reactive_relation_instance.outbound_id)
            .ok_or(ReactiveRelationInstanceCreationError::MissingOutboundEntityInstance(reactive_relation_instance.outbound_id))?;
        let inbound = self
            .reactive_entity_instance_manager
            .get(reactive_relation_instance.inbound_id)
            .ok_or(ReactiveRelationInstanceCreationError::MissingInboundEntityInstance(reactive_relation_instance.inbound_id))?;
        let ty = reactive_relation_instance.ty.clone();
        let relation_ty = ty.relation_type_id();
        let relation_type = self
            .relation_type_manager
            .get(&relation_ty)
            .ok_or_else(|| ReactiveRelationInstanceCreationError::UnknownRelationType(relation_ty.clone()))?;

        if !relation_type.outbound_type.type_name().eq("*") {
            match &relation_type.outbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !outbound.components.contains(component_ty) {
                        return Err(ReactiveRelationInstanceCreationError::OutboundEntityDoesNotHaveComponent(
                            relation_ty.clone(),
                            component_ty.clone(),
                        ));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if &outbound.ty != entity_ty {
                        return Err(ReactiveRelationInstanceCreationError::OutboundEntityIsNotOfType(relation_ty.clone(), entity_ty.clone()));
                    }
                }
            }
        }
        // if !relation_type.outbound_type.eq("*")
        //     && !outbound.type_name.eq(&relation_type.outbound_type)
        //     && !outbound.components.contains(&relation_type.outbound_type)
        // {
        //     return Err(ReactiveRelationInstanceCreationError::OutboundEntityIsNotOfType(
        //         outbound.type_name.clone(),
        //         relation_type.outbound_type,
        //     ));
        // }

        if !relation_type.inbound_type.type_name().eq("*") {
            match &relation_type.inbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !inbound.components.contains(component_ty) {
                        return Err(ReactiveRelationInstanceCreationError::InboundEntityDoesNotHaveComponent(
                            relation_ty.clone(),
                            component_ty.clone(),
                        ));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if &inbound.ty != entity_ty {
                        return Err(ReactiveRelationInstanceCreationError::InboundEntityIsNotOfType(relation_ty.clone(), entity_ty.clone()));
                    }
                }
            }
        }

        // if !relation_type.inbound_type.eq("*")
        //     && !inbound.type_name.eq(&relation_type.inbound_type)
        //     && !inbound.components.contains(&relation_type.inbound_type)
        // {
        //     return Err(ReactiveRelationInstanceCreationError::InboundEntityIsNotOfType(
        //         inbound.type_name.clone(),
        //         relation_type.inbound_type,
        //     ));
        // }

        let relation_instance = Arc::new(ReactiveRelationInstance::new_from_instance(outbound, inbound, reactive_relation_instance));
        self.register_reactive_instance(relation_instance)
            .map_err(|e| ReactiveRelationInstanceCreationError::ReactiveRelationInstanceRegistrationError(e))
    }

    fn register_reactive_instance(
        &self,
        reactive_relation_instance: Arc<ReactiveRelationInstance>,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceRegistrationError> {
        let edge_key = reactive_relation_instance.get_key();
        match self
            .relation_instance_manager
            .create_from_instance_if_not_exist(reactive_relation_instance.clone().into())
        {
            Ok(_edge_key) => {
                self.reactive_relation_instances.0.insert(edge_key.clone(), reactive_relation_instance.clone());
                // Apply all components that are predefined in the relation type
                let relation_ty = reactive_relation_instance.relation_type_id();
                if let Some(components) = self.relation_type_manager.get(&relation_ty).map(|relation_type| relation_type.components) {
                    components.iter().for_each(|component_ty| {
                        reactive_relation_instance.components.insert(component_ty.clone());
                    });
                }
                // Add component behaviours
                self.relation_component_behaviour_manager
                    .add_behaviours_to_relation(reactive_relation_instance.clone());
                // Add relation behaviours
                self.relation_behaviour_manager.add_behaviours(reactive_relation_instance.clone());
                self.event_manager.emit_event(SystemEvent::RelationInstanceCreated(edge_key));
                Ok(reactive_relation_instance)
            }
            Err(e) => Err(ReactiveRelationInstanceRegistrationError::RelationInstanceCreationError(e)),
        }
    }

    fn register_or_merge_reactive_instance(
        &self,
        relation_instance: Arc<ReactiveRelationInstance>,
    ) -> Result<Arc<ReactiveRelationInstance>, ReactiveRelationInstanceRegistrationError> {
        let edge_key = relation_instance.get_key();
        match self.get(&edge_key) {
            // No instance with the given edge key exists yet, try to register the given reactive instance
            None => self.register_reactive_instance(relation_instance),
            // Instance with the given edge key exists. Don't register but return the existing reactive instance instead of the given instance
            Some(reactive_relation_instance) => Ok(reactive_relation_instance),
        }
    }

    fn add_component(&self, edge_key: &EdgeKey, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationInstanceComponentAddError> {
        match self.component_manager.get(component_ty) {
            Some(component) => {
                match self.get(edge_key) {
                    Some(relation_instance) => {
                        // Add components with properties
                        relation_instance.add_component_with_properties(&component);
                        // Add component behaviours
                        self.relation_component_behaviour_manager
                            .add_behaviours_to_relation_component(relation_instance, component);
                        Ok(())
                    }
                    None => Err(ReactiveRelationInstanceComponentAddError::MissingInstance(edge_key.clone())),
                }
            }
            None => Err(ReactiveRelationInstanceComponentAddError::MissingComponent(component_ty.clone())),
        }
    }

    fn remove_component(&self, edge_key: &EdgeKey, component_ty: &ComponentTypeId) {
        if let Some(component) = self.component_manager.get(component_ty) {
            if let Some(reactive_relation_instance) = self.get(edge_key) {
                // Remove component
                reactive_relation_instance.remove_component(component_ty);
                //
                // We do not remove properties because we cannot ensure that the removal is intended
                // (At least yet)
                //
                // Remove component behaviours
                self.relation_component_behaviour_manager
                    .remove_behaviours_from_relation_component(reactive_relation_instance, component);
            }
        }
    }

    fn add_property(&self, edge_key: &EdgeKey, property_name: &str, value: Value) -> Result<(), ReactiveRelationInstancePropertyAddError> {
        match self.get(edge_key) {
            Some(relation_instance) => {
                if relation_instance.has_property(property_name) {
                    return Err(ReactiveRelationInstancePropertyAddError::PropertyAlreadyExists(property_name.to_string()));
                }
                relation_instance.add_property(property_name, value);
                Ok(())
            }
            None => Err(ReactiveRelationInstancePropertyAddError::MissingInstance(edge_key.clone())),
        }
    }

    fn remove_property(&self, edge_key: &EdgeKey, property_name: &str) -> Result<(), ReactiveRelationInstancePropertyRemoveError> {
        match self.get(edge_key) {
            Some(relation_instance) => {
                if !relation_instance.has_property(property_name) {
                    return Err(ReactiveRelationInstancePropertyRemoveError::MissingProperty(property_name.to_string()));
                }
                for component_ty in relation_instance.get_components() {
                    if let Some(component) = self.component_manager.get(&component_ty) {
                        if component.has_property(property_name) {
                            return Err(ReactiveRelationInstancePropertyRemoveError::PropertyInUseByComponent(component_ty.clone()));
                        }
                    }
                }
                relation_instance.remove_property(property_name);
                Ok(())
            }
            None => Err(ReactiveRelationInstancePropertyRemoveError::MissingInstance(edge_key.clone())),
        }
    }

    fn commit(&self, edge_key: &EdgeKey) {
        if let Some(reactive_relation_instance) = self.get(edge_key) {
            self.relation_instance_manager.commit(reactive_relation_instance.into());
        }
    }

    fn delete(&self, edge_key: &EdgeKey) -> bool {
        if self.has(edge_key) {
            self.unregister_reactive_instance(edge_key);
        }
        let result = self.relation_instance_manager.delete(edge_key);
        self.event_manager.emit_event(SystemEvent::RelationInstanceDeleted(edge_key.clone()));
        result
    }

    fn unregister_reactive_instance(&self, edge_key: &EdgeKey) {
        match self.get(edge_key) {
            Some(relation_instance) => {
                // Remove relation behaviours
                self.relation_behaviour_manager.remove_behaviours(relation_instance.clone());
                // Remove component behaviours
                self.relation_component_behaviour_manager.remove_behaviours_from_relation(relation_instance);
            }
            None => {
                // Remove relation behaviours
                self.relation_behaviour_manager.remove_behaviours_by_key(edge_key);
                // Remove component behaviours
                self.relation_component_behaviour_manager.remove_behaviours_by_key(edge_key);
            }
        }
        self.reactive_relation_instances.0.remove(edge_key);
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

    fn export(&self, edge_key: &EdgeKey, path: &str) {
        if self.has(edge_key) {
            self.commit(edge_key);
            self.relation_instance_manager.export(edge_key, path);
        }
    }

    fn handle_component_added_events(&self) {
        let component_manager = self.component_manager.clone();
        let relation_component_behaviour_manager = self.relation_component_behaviour_manager.clone();
        let reactive_relation_instances = self.reactive_relation_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_component_event) => {
                            if let Ok(type_definition_component) = TypeDefinitionComponent::try_from(type_definition_component_event.clone()) {
                                if let Some(component) = component_manager.get(&type_definition_component.component) {
                                    for reactive_relation_instance in reactive_relation_instances
                                        .iter()
                                        .filter(|relation_instance| {
                                            &relation_instance.relation_type_id().type_definition() == &type_definition_component.type_definition
                                        })
                                        .map(|relation_instance| relation_instance.value().clone())
                                    {
                                        reactive_relation_instance.add_component_with_properties(&component);
                                        relation_component_behaviour_manager
                                            .add_behaviours_to_relation_component(reactive_relation_instance, component.clone());
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
        let relation_component_behaviour_manager = self.relation_component_behaviour_manager.clone();
        let reactive_relation_instances = self.reactive_relation_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_component_event) => {
                            if let Ok(type_definition_component) = TypeDefinitionComponent::try_from(type_definition_component_event.clone()) {
                                if let Some(component) = component_manager.get(&type_definition_component.component) {
                                    for reactive_relation_instance in reactive_relation_instances
                                        .iter()
                                        .filter(|relation_instance| {
                                            &relation_instance.relation_type_id().type_definition() == &type_definition_component.type_definition
                                        })
                                        .map(|relation_instance| relation_instance.value().clone())
                                    {
                                        reactive_relation_instance.remove_component(&component.ty);
                                        relation_component_behaviour_manager
                                            .remove_behaviours_from_relation_component(reactive_relation_instance, component.clone());
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
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_property_event) => {
                            if let Ok(type_definition_property) = TypeDefinitionProperty::try_from(type_definition_property_event.clone()) {
                                if let Ok(relation_ty) = RelationTypeId::try_from(&type_definition_property.type_definition) {
                                    if let Some(relation_type) = relation_type_manager.get(&relation_ty) {
                                        for reactive_relation_instance in reactive_relation_instances
                                            .iter()
                                            .filter(|relation_instance| &relation_instance.relation_type_id() == &relation_ty)
                                            .map(|relation_instance| relation_instance.value().clone())
                                        {
                                            if let Some(property_type) = relation_type.get_own_property(&type_definition_property.property) {
                                                reactive_relation_instance.add_property_by_type(&property_type);
                                            }
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
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED) {
            self.runtime.0.spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_property_event) => {
                            if let Ok(type_definition_property) = TypeDefinitionProperty::try_from(type_definition_property_event.clone()) {
                                for reactive_relation_instance in reactive_relation_instances
                                    .iter()
                                    .filter(|relation_instance| {
                                        &relation_instance.relation_type_id().type_definition() == &type_definition_property.type_definition
                                    })
                                    .map(|relation_instance| relation_instance.value().clone())
                                {
                                    reactive_relation_instance.remove_property(&type_definition_property.property);
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
