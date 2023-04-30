use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use dashmap::DashMap;
use log::error;
use path_tree::PathTree;
use serde_json::Value;
use tokio::time::sleep;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::EntityBehaviourManager;
use crate::api::EntityComponentBehaviourManager;
use crate::api::EntityInstanceManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceComponentAddError;
use crate::api::ReactiveEntityInstanceCreationError;
use crate::api::ReactiveEntityInstanceImportError;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveEntityInstancePropertyAddError;
use crate::api::ReactiveEntityInstancePropertyRemoveError;
use crate::api::ReactiveEntityInstanceRegistrationError;
use crate::api::SystemEventManager;
use crate::api::SystemEventSubscriber;
use crate::di::*;
use crate::model::BehaviourTypeId;
use crate::model::ComponentBehaviourTypeId;
use crate::model::ComponentContainer;
use crate::model::ComponentTypeId;
use crate::model::EntityBehaviourTypeId;
use crate::model::EntityInstance;
use crate::model::EntityTypeId;
use crate::model::Mutability;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyTypeContainer;
use crate::model::PropertyTypeDefinition;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactivePropertyContainer;
use crate::model::TypeDefinitionComponent;
use crate::model::TypeDefinitionGetter;
use crate::model::TypeDefinitionProperty;
use crate::model_runtime::EventProperties::EVENT;
use crate::model_runtime::LabeledProperties::LABEL;
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

#[component]
pub struct ReactiveEntityInstanceManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    entity_instance_manager: Wrc<dyn EntityInstanceManager>,

    entity_behaviour_manager: Wrc<dyn EntityBehaviourManager>,

    entity_component_behaviour_manager: Wrc<dyn EntityComponentBehaviourManager>,

    reactive_entity_instances: ReactiveEntityInstances,

    label_path_tree: LabelPathTree,

    running: RunningState,

    system_event_channels: SystemEventChannels,
    // TODO: Type Cache
}

impl SystemEventSubscriber for ReactiveEntityInstanceManagerImpl {
    fn subscribe_system_event(&self, system_event_type: SystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.event_manager.get_system_event_instance(system_event_type) {
            if let Some(sender) = self.system_event_channels.sender(&handle_id) {
                entity_instance.observe_with_handle(
                    &EVENT.property_name(),
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
            entity_instance.remove_observer(&EVENT.property_name(), handle_id);
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
                let params = result.1.params_iter().map(|param| (param.0.to_string(), param.1.to_string())).collect();
                Some((instance, params))
            }
            None => None,
        })
    }

    fn get_all(&self) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instances.0.iter().map(|e| e.value().clone()).collect()
    }

    fn get_by_type(&self, ty: &EntityTypeId) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instances
            .0
            .iter()
            .filter(|e| &e.ty == ty)
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_component(&self, ty: &ComponentTypeId) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instances
            .0
            .iter()
            .filter(|e| e.is_a(ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instances
            .0
            .iter()
            .filter(|e| e.behaves_as(behaviour_ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<Arc<ReactiveEntityInstance>> {
        self.reactive_entity_instances
            .0
            .iter()
            .filter(|r| r.namespace() == namespace)
            .map(|r| r.value().clone())
            .collect()
    }

    fn count(&self) -> usize {
        self.reactive_entity_instances.0.len()
    }

    fn count_by_type(&self, ty: &EntityTypeId) -> usize {
        self.reactive_entity_instances.0.iter().filter(|e| &e.ty == ty).count()
    }

    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize {
        self.reactive_entity_instances.0.iter().filter(|e| e.is_a(component_ty)).count()
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_entity_instances.0.iter().filter(|e| e.behaves_as(behaviour_ty)).count()
    }

    fn get_ids(&self) -> Vec<Uuid> {
        self.reactive_entity_instances.0.iter().map(|e| *e.key()).collect()
    }

    fn create(&self, ty: &EntityTypeId, properties: HashMap<String, Value>) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        let result = self.entity_instance_manager.create(ty, properties);
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
        ty: &EntityTypeId,
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
                let result = self.entity_instance_manager.create_with_id(ty, id, properties);
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

        // Initialize property mutability states
        if let Some(entity_type) = self.entity_type_manager.get(&reactive_entity_instance.ty) {
            for component_ty in entity_type.components {
                if let Some(component) = self.component_manager.get(&component_ty) {
                    for property_type in component.properties.iter() {
                        if let Some(mut property) = reactive_entity_instance.properties.get_mut(&property_type.name) {
                            property.set_mutability(property_type.mutability);
                        }
                    }
                }
            }
            for property_type in entity_type.properties.iter() {
                if let Some(mut property) = reactive_entity_instance.properties.get_mut(&property_type.name) {
                    property.set_mutability(property_type.mutability);
                }
            }
        }

        self.register_reactive_instance(reactive_entity_instance)
            .map_err(ReactiveEntityInstanceCreationError::ReactiveEntityInstanceRegistrationError)
    }

    fn register_reactive_instance(
        &self,
        reactive_entity_instance: Arc<ReactiveEntityInstance>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceRegistrationError> {
        match self
            .entity_instance_manager
            .create_from_instance_if_not_exist(reactive_entity_instance.clone().into())
        {
            Ok(_id) => {
                self.reactive_entity_instances
                    .0
                    .insert(reactive_entity_instance.id, reactive_entity_instance.clone());
                // Apply all components that are predefined in the entity type
                if let Some(components) = self
                    .entity_type_manager
                    .get(&reactive_entity_instance.ty)
                    .map(|entity_type| entity_type.components)
                {
                    components.iter().for_each(|component_ty| {
                        reactive_entity_instance.components.insert(component_ty.clone());
                    });
                }
                // Add component behaviours
                self.entity_component_behaviour_manager
                    .add_behaviours_to_entity(reactive_entity_instance.clone());
                // Add entity behaviours
                self.entity_behaviour_manager.add_behaviours(reactive_entity_instance.clone());
                // Register label
                if let Some(value) = reactive_entity_instance
                    .get(LABEL.property_name())
                    .and_then(|v| v.as_str().map(|s| s.to_string()))
                {
                    let mut writer = self.label_path_tree.0.write().unwrap();
                    writer.insert(&value, reactive_entity_instance.id);
                }
                self.event_manager.emit_event(SystemEvent::EntityInstanceCreated(reactive_entity_instance.id));
                Ok(reactive_entity_instance)
            }
            Err(e) => Err(ReactiveEntityInstanceRegistrationError::EntityInstanceCreationError(e)),
        }
    }

    fn register_or_merge_reactive_instance(
        &self,
        entity_instance: Arc<ReactiveEntityInstance>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceRegistrationError> {
        match self.get(entity_instance.id) {
            // No instance with the given id exists: register as new instance and return it
            None => self.register_reactive_instance(entity_instance),
            // Instance with the given id exists. Don't register but return the existing reactive instance instead of the given instance
            Some(entity_instance) => Ok(entity_instance),
        }
    }

    fn add_component(&self, id: Uuid, component_ty: &ComponentTypeId) -> Result<(), ReactiveEntityInstanceComponentAddError> {
        match self.component_manager.get(component_ty) {
            Some(component) => {
                match self.get(id) {
                    Some(entity_instance) => {
                        // Add components with properties
                        entity_instance.add_component_with_properties(&component);
                        // Add component behaviours
                        self.entity_component_behaviour_manager
                            .add_behaviours_to_entity_component(entity_instance, component);
                        Ok(())
                    }
                    None => Err(ReactiveEntityInstanceComponentAddError::MissingInstance(id)),
                }
            }
            None => Err(ReactiveEntityInstanceComponentAddError::MissingComponent(component_ty.clone())),
        }
    }

    fn remove_component(&self, id: Uuid, component_ty: &ComponentTypeId) {
        if let Some(component) = self.component_manager.get(component_ty) {
            if let Some(entity_instance) = self.get(id) {
                // Remove component
                entity_instance.remove_component(component_ty);
                // We do not remove properties because we cannot asure that the removal is intended
                // Remove component behaviours
                self.entity_component_behaviour_manager
                    .remove_behaviours_from_entity_component(entity_instance, component);
            }
        }
    }

    fn add_property(&self, id: Uuid, property_name: &str, mutability: Mutability, value: Value) -> Result<(), ReactiveEntityInstancePropertyAddError> {
        match self.get(id) {
            Some(entity_instance) => {
                if entity_instance.has_property(property_name) {
                    return Err(ReactiveEntityInstancePropertyAddError::PropertyAlreadyExists(property_name.to_string()));
                }
                entity_instance.add_property(property_name, mutability, value);
                Ok(())
            }
            None => Err(ReactiveEntityInstancePropertyAddError::MissingInstance(id)),
        }
    }

    fn remove_property(&self, id: Uuid, property_name: &str) -> Result<(), ReactiveEntityInstancePropertyRemoveError> {
        match self.get(id) {
            Some(entity_instance) => {
                if !entity_instance.has_property(property_name) {
                    return Err(ReactiveEntityInstancePropertyRemoveError::MissingProperty(property_name.to_string()));
                }
                for component_ty in entity_instance.get_components() {
                    if let Some(component) = self.component_manager.get(&component_ty) {
                        if component.has_property(property_name) {
                            return Err(ReactiveEntityInstancePropertyRemoveError::PropertyInUseByComponent(
                                property_name.to_string(),
                                component_ty.clone(),
                            ));
                        }
                    }
                }
                entity_instance.remove_property(property_name);
                Ok(())
            }
            None => Err(ReactiveEntityInstancePropertyRemoveError::MissingInstance(id)),
        }
    }

    fn add_behaviour_to_all_entity_instances(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        for entity_instance in self.reactive_entity_instances.0.iter() {
            if entity_instance.ty == entity_behaviour_ty.entity_ty {
                self.entity_behaviour_manager
                    .add_behaviour(entity_instance.clone(), &entity_behaviour_ty.behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_all_entity_components(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        for entity_instance in self.reactive_entity_instances.0.iter() {
            if entity_instance.components.contains(&component_behaviour_ty.component_ty) {
                self.entity_component_behaviour_manager
                    .add_behaviour_to_entity_component(entity_instance.clone(), component_behaviour_ty);
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
                // Remove entity behaviours
                self.entity_behaviour_manager.remove_behaviours(entity_instance.clone());
                // Remove entity component behaviours
                self.entity_component_behaviour_manager.remove_behaviours_from_entity(entity_instance);
            }
            None => {
                // Remove entity behaviours
                self.entity_behaviour_manager.remove_behaviours_by_id(&id);
                // Remove entity component behaviours
                self.entity_component_behaviour_manager.remove_behaviours_by_id(&id);
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
        let component_behaviour_manager = self.entity_component_behaviour_manager.clone();
        let reactive_entity_instances = self.reactive_entity_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED) {
            tokio::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_component_event) => {
                            if let Ok(type_definition_component) = TypeDefinitionComponent::try_from(type_definition_component_event.clone()) {
                                if let Some(component) = component_manager.get(&type_definition_component.component_ty) {
                                    for instance in reactive_entity_instances
                                        .iter()
                                        .filter(|instance| instance.ty.type_definition() == type_definition_component.type_definition)
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
        let component_behaviour_manager = self.entity_component_behaviour_manager.clone();
        let reactive_entity_instances = self.reactive_entity_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED) {
            tokio::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_component_event) => {
                            if let Ok(type_definition_component) = TypeDefinitionComponent::try_from(type_definition_component_event.clone()) {
                                if let Some(component) = component_manager.get(&type_definition_component.component_ty) {
                                    for reactive_entity_instance in reactive_entity_instances
                                        .iter()
                                        .filter(|entity_instance| entity_instance.type_definition() == type_definition_component.type_definition)
                                        .map(|entity_instance| entity_instance.value().clone())
                                    {
                                        reactive_entity_instance.remove_component(&component.ty);
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
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED) {
            tokio::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_property_event) => {
                            if let Ok(type_definition_property) = TypeDefinitionProperty::try_from(type_definition_property_event.clone()) {
                                if let Ok(entity_ty) = EntityTypeId::try_from(&type_definition_property.type_definition) {
                                    if let Some(entity_type) = entity_type_manager.get(&entity_ty) {
                                        for reactive_entity_instance in reactive_entity_instances
                                            .iter()
                                            .filter(|entity_instance| entity_instance.ty == entity_ty)
                                            .map(|entity_instance| entity_instance.value().clone())
                                        {
                                            if let Some(property_type) = entity_type.get_own_property(&type_definition_property.property) {
                                                reactive_entity_instance.add_property_by_type(&property_type);
                                            }
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
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED) {
            tokio::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_property_event) => {
                            if let Ok(type_definition_property) = TypeDefinitionProperty::try_from(type_definition_property_event.clone()) {
                                if let Ok(entity_ty) = EntityTypeId::try_from(&type_definition_property.type_definition) {
                                    for reactive_entity_instance in reactive_entity_instances
                                        .iter()
                                        .filter(|entity_instance| entity_instance.ty == entity_ty)
                                        .map(|entity_instance| entity_instance.value().clone())
                                    {
                                        reactive_entity_instance.remove_property(&type_definition_property.property);
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
}

#[async_trait]
impl Lifecycle for ReactiveEntityInstanceManagerImpl {
    async fn post_init(&self) {
        for event_instance in self.event_manager.get_system_event_instances() {
            if let Err(e) = self.register_reactive_instance(event_instance) {
                error!("Failed to register system event instance: {:?}", e);
                // TODO: Propagate this error
            }
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

    async fn pre_shutdown(&self) {
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
