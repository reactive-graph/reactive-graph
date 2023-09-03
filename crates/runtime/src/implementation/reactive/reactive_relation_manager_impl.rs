use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use dashmap::DashMap;
use dashmap::DashSet;
use serde_json::Value;
use tokio::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use inexor_rgf_core_model::{PropertyInstances, PropertyTypeContainer};
use inexor_rgf_reactive::ReactiveInstance;

use crate::api::ComponentManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityManager;
use crate::api::ReactiveRelationComponentAddError;
use crate::api::ReactiveRelationCreationError;
use crate::api::ReactiveRelationManager;
use crate::api::ReactiveRelationPropertyAddError;
use crate::api::ReactiveRelationPropertyRemoveError;
use crate::api::ReactiveRelationRegistrationError;
use crate::api::RelationBehaviourManager;
use crate::api::RelationComponentBehaviourManager;
use crate::api::RelationTypeManager;
use crate::api::system_event_subscriber::SystemEventSubscriber;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::reactive::BehaviourTypeId;
use crate::reactive::ComponentBehaviourTypeId;
use crate::reactive::ComponentContainer;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::Mutability;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyTypeDefinition;
use crate::reactive::ReactiveBehaviourContainer;
use crate::reactive::ReactivePropertyContainer;
use crate::reactive::ReactiveRelation;
use crate::model::RelationInstanceId;
use crate::reactive::RelationBehaviourTypeId;
use crate::model::RelationInstance;
use crate::model::RelationTypeId;
use crate::model::TypeDefinitionComponent;
use crate::model::TypeDefinitionGetter;
use crate::model::TypeDefinitionProperty;
use crate::model_runtime::EventProperties::EVENT;
use crate::plugins::SystemEvent;
use crate::plugins::SystemEventTypes;

static HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED: u128 = 0x6ba7b9210e1513d350b300c04fe530c7;
static HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED: u128 = 0x6ba8b8119e1513ee59b300c04fe630c7;
static HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED: u128 = 0x6bb9b9232e1513d350b300c04fe530c7;
static HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED: u128 = 0x6ba8b8339e1535ee5bd300c0410630c7;

#[wrapper]
pub struct ReactiveRelations(DashMap<RelationInstanceId, ReactiveRelation>); // Arc<DashMap<>>

#[wrapper]
pub struct OutboundInstances(DashMap<Uuid, DashSet<RelationInstanceId>>);

impl OutboundInstances {
    pub fn insert(&self, id: &RelationInstanceId) {
        match self.0.get(&id.outbound_id) {
            Some(outbound_instances) => {
                outbound_instances.insert(id.clone());
            }
            None => {
                let outbound_instances = DashSet::new();
                outbound_instances.insert(id.clone());
                self.0.insert(id.outbound_id, outbound_instances);
            }
        }
    }

    pub fn remove(&self, id: &RelationInstanceId) {
        self.0.get(&id.outbound_id).and_then(|outbound_instances| outbound_instances.remove(id));
    }
}

#[wrapper]
pub struct InboundInstances(DashMap<Uuid, DashSet<RelationInstanceId>>);

impl InboundInstances {
    pub fn insert(&self, id: &RelationInstanceId) {
        match self.0.get(&id.inbound_id) {
            Some(inbound_instances) => {
                inbound_instances.insert(id.clone());
            }
            None => {
                let inbound_instances = DashSet::new();
                inbound_instances.insert(id.clone());
                self.0.insert(id.inbound_id, inbound_instances);
            }
        }
    }

    pub fn remove(&self, id: &RelationInstanceId) {
        self.0.get(&id.inbound_id).and_then(|inbound_instances| inbound_instances.remove(id));
    }
}

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
fn create_reactive_relation_instance_storage() -> ReactiveRelations {
    // ReactiveRelations(Arc::new(DashMap::new()))
    ReactiveRelations(DashMap::new())
}

#[provides]
fn create_outbound_instances_storage() -> OutboundInstances {
    OutboundInstances(DashMap::new())
}

#[provides]
fn create_inbound_instances_storage() -> InboundInstances {
    InboundInstances(DashMap::new())
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

#[component]
pub struct ReactiveRelationManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    reactive_entity_manager: Wrc<dyn ReactiveEntityManager>,

    relation_behaviour_manager: Wrc<dyn RelationBehaviourManager>,

    relation_component_behaviour_manager: Wrc<dyn RelationComponentBehaviourManager>,

    reactive_relation_instances: ReactiveRelations,

    outbound_instances: OutboundInstances,

    inbound_instances: InboundInstances,

    running: RunningState,

    system_event_channels: SystemEventChannels,
}

impl SystemEventSubscriber for ReactiveRelationManagerImpl {
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
impl ReactiveRelationManager for ReactiveRelationManagerImpl {
    fn has(&self, id: &RelationInstanceId) -> bool {
        self.reactive_relation_instances.0.contains_key(id)
    }

    fn get(&self, id: &RelationInstanceId) -> Option<ReactiveRelation> {
        self.reactive_relation_instances.0.get(id).map(|r| r.value().clone())
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<ReactiveRelation> {
        self.outbound_instances.get(&outbound_entity_id).and_then(|outbound_instances| outbound_instances.value().iter().map(|id| self.get(id.deref())).collect()).unwrap_or(Vec::new())
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<ReactiveRelation> {
        self.inbound_instances.get(&inbound_entity_id).and_then(|inbound_instances| inbound_instances.value().iter().map(|id| self.get(id.deref())).collect()).unwrap_or(Vec::new())
    }

    fn get_all(&self) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances.0.iter().map(|r| r.value().clone()).collect()
    }

    fn get_by_type(&self, ty: &RelationTypeId) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances
            .0
            .iter()
            .filter(|r| &r.relation_type_id() == ty)
            .map(|r| r.value().clone())
            .collect()
    }

    fn get_by_component(&self, ty: &ComponentTypeId) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances
            .0
            .iter()
            .filter(|e| e.is_a(ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances
            .0
            .iter()
            .filter(|e| e.behaves_as(behaviour_ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<ReactiveRelation> {
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
        self.reactive_relation_instances.0.iter().filter(|r| r.is_a(component_ty)).count()
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_relation_instances.0.iter().filter(|r| r.behaves_as(behaviour_ty)).count()
    }

    fn get_keys(&self) -> Vec<RelationInstanceId> {
        self.reactive_relation_instances.0.iter().map(|e| e.key().clone()).collect()
    }

    fn create(&self, id: &RelationInstanceId, properties: PropertyInstances) -> Result<ReactiveRelation, ReactiveRelationCreationError> {
        let relation_instance = RelationInstance::builder().outbound_id(id.outbound_id).ty(id.ty.clone()).inbound_id(id.inbound_id).properties(properties).build();
        self.create_reactive_instance(relation_instance)
        // match self.relation_instance_manager.create(edge_key, properties) {
        //     Ok(edge_key) => match self.relation_instance_manager.get(&edge_key) {
        //         Some(relation_instance) => self.create_reactive_instance(relation_instance),
        //         None => Err(ReactiveRelationCreationError::MissingInstance(edge_key)),
        //     },
        //     Err(e) => Err(ReactiveRelationCreationError::RelationInstanceCreationError(e)),
        // }
    }

    fn create_reactive_instance(
        &self,
        reactive_relation_instance: RelationInstance,
    ) -> Result<ReactiveRelation, ReactiveRelationCreationError> {
        let outbound = self
            .reactive_entity_manager
            .get(reactive_relation_instance.outbound_id)
            .ok_or(ReactiveRelationCreationError::MissingOutboundEntityInstance(reactive_relation_instance.outbound_id))?;
        let inbound = self
            .reactive_entity_manager
            .get(reactive_relation_instance.inbound_id)
            .ok_or(ReactiveRelationCreationError::MissingInboundEntityInstance(reactive_relation_instance.inbound_id))?;
        let ty = reactive_relation_instance.ty.clone();
        let relation_ty = ty.relation_type_id();
        let relation_type = self
            .relation_type_manager
            .get(&relation_ty)
            .ok_or_else(|| ReactiveRelationCreationError::UnknownRelationType(relation_ty.clone()))?;

        if !relation_type.outbound_type.type_name().eq("*") {
            match &relation_type.outbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !outbound.components.contains(component_ty) {
                        return Err(ReactiveRelationCreationError::OutboundEntityDoesNotHaveComponent(relation_ty, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if &outbound.ty != entity_ty {
                        return Err(ReactiveRelationCreationError::OutboundEntityIsNotOfType(relation_ty, entity_ty.clone()));
                    }
                }
            }
        }
        // if !relation_type.outbound_type.eq("*")
        //     && !outbound.type_name.eq(&relation_type.outbound_type)
        //     && !outbound.components.contains(&relation_type.outbound_type)
        // {
        //     return Err(ReactiveRelationCreationError::OutboundEntityIsNotOfType(
        //         outbound.type_name.clone(),
        //         relation_type.outbound_type,
        //     ));
        // }

        if !relation_type.inbound_type.type_name().eq("*") {
            match &relation_type.inbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !inbound.components.contains(component_ty) {
                        return Err(ReactiveRelationCreationError::InboundEntityDoesNotHaveComponent(relation_ty, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if &inbound.ty != entity_ty {
                        return Err(ReactiveRelationCreationError::InboundEntityIsNotOfType(relation_ty, entity_ty.clone()));
                    }
                }
            }
        }

        // if !relation_type.inbound_type.eq("*")
        //     && !inbound.type_name.eq(&relation_type.inbound_type)
        //     && !inbound.components.contains(&relation_type.inbound_type)
        // {
        //     return Err(ReactiveRelationCreationError::InboundEntityIsNotOfType(
        //         inbound.type_name.clone(),
        //         relation_type.inbound_type,
        //     ));
        // }

        let relation_instance = ReactiveRelation::new_from_instance(outbound, inbound, reactive_relation_instance);

        // Initialize property mutability states
        if let Some(entity_type) = self.relation_type_manager.get(&relation_instance.relation_type_id()) {
            for component_ty in entity_type.components {
                if let Some(component) = self.component_manager.get(&component_ty) {
                    for property_type in component.properties.iter() {
                        if let Some(mut property) = relation_instance.properties.get_mut(&property_type.name) {
                            property.set_mutability(property_type.mutability);
                        }
                    }
                }
            }
            for property_type in entity_type.properties.iter() {
                if let Some(mut property) = relation_instance.properties.get_mut(&property_type.name) {
                    property.set_mutability(property_type.mutability);
                }
            }
        }

        self.register_reactive_instance(relation_instance)
            .map_err(ReactiveRelationCreationError::ReactiveRelationRegistrationError)
    }

    fn register_reactive_instance(
        &self,
        reactive_relation: ReactiveRelation,
    ) -> Result<ReactiveRelation, ReactiveRelationRegistrationError> {
        let id = reactive_relation.id();
        self.reactive_relation_instances.0.insert(id.clone(), reactive_relation.clone());
        self.outbound_instances.insert(&id);
        self.inbound_instances.insert(&id);
        // match self.outbound_instances.0.get(&id.outbound) {
        //     Some(outbound_instances) => {
        //         outbound_instances.insert(id);
        //     }
        //     None => {
        //         let mut outbound_instances = DashSet::new();
        //         outbound_instances.insert(id.clone());
        //         self.outbound_instances.0.insert(id.outbound, outbound_instances);
        //     }
        // }
        // Apply all components that are predefined in the relation type
        let relation_ty = reactive_relation.relation_type_id();
        if let Some(components) = self.relation_type_manager.get(&relation_ty).map(|relation_type| relation_type.components) {
            components.iter().for_each(|component_ty| {
                reactive_relation.components.insert(component_ty.clone());
            });
        }
        // Add component behaviours
        self.relation_component_behaviour_manager
            .add_behaviours_to_relation(reactive_relation.clone());
        // Add relation behaviours
        self.relation_behaviour_manager.add_behaviours(reactive_relation.clone());
        self.event_manager.emit_event(SystemEvent::RelationInstanceCreated(id));
        Ok(reactive_relation)


        // match self
        //     .relation_instance_manager
        //     .create_from_instance_if_not_exist(reactive_relation_instance.clone().into())
        // {
        //     Ok(_edge_key) => {
        //         self.reactive_relation_instances.0.insert(edge_key.clone(), reactive_relation_instance.clone());
        //         // Apply all components that are predefined in the relation type
        //         let relation_ty = reactive_relation_instance.relation_type_id();
        //         if let Some(components) = self.relation_type_manager.get(&relation_ty).map(|relation_type| relation_type.components) {
        //             components.iter().for_each(|component_ty| {
        //                 reactive_relation_instance.components.insert(component_ty.clone());
        //             });
        //         }
        //         // Add component behaviours
        //         self.relation_component_behaviour_manager
        //             .add_behaviours_to_relation(reactive_relation_instance.clone());
        //         // Add relation behaviours
        //         self.relation_behaviour_manager.add_behaviours(reactive_relation_instance.clone());
        //         self.event_manager.emit_event(SystemEvent::RelationInstanceCreated(edge_key));
        //         Ok(reactive_relation_instance)
        //     }
        //     Err(e) => Err(ReactiveRelationRegistrationError::RelationInstanceCreationError(e)),
        // }
    }

    fn register_or_merge_reactive_instance(
        &self,
        relation_instance: ReactiveRelation,
    ) -> Result<ReactiveRelation, ReactiveRelationRegistrationError> {
        let id = relation_instance.id();
        match self.get(&id) {
            // No instance with the given edge key exists yet, try to register the given reactive instance
            None => self.register_reactive_instance(relation_instance),
            // Instance with the given edge key exists. Don't register but return the existing reactive instance instead of the given instance
            Some(reactive_relation_instance) => Ok(reactive_relation_instance),
        }
    }

    fn add_component(&self, edge_key: &RelationInstanceId, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationComponentAddError> {
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
                    None => Err(ReactiveRelationComponentAddError::MissingInstance(edge_key.clone())),
                }
            }
            None => Err(ReactiveRelationComponentAddError::MissingComponent(component_ty.clone())),
        }
    }

    fn remove_component(&self, edge_key: &RelationInstanceId, component_ty: &ComponentTypeId) {
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

    fn add_property(
        &self,
        edge_key: &RelationInstanceId,
        property_name: &str,
        mutability: Mutability,
        value: Value,
    ) -> Result<(), ReactiveRelationPropertyAddError> {
        match self.get(edge_key) {
            Some(relation_instance) => {
                if relation_instance.has_property(property_name) {
                    return Err(ReactiveRelationPropertyAddError::PropertyAlreadyExists(property_name.to_string()));
                }
                relation_instance.add_property(property_name, mutability, value);
                Ok(())
            }
            None => Err(ReactiveRelationPropertyAddError::MissingInstance(edge_key.clone())),
        }
    }

    fn remove_property(&self, edge_key: &RelationInstanceId, property_name: &str) -> Result<(), ReactiveRelationPropertyRemoveError> {
        match self.get(edge_key) {
            Some(relation_instance) => {
                if !relation_instance.has_property(property_name) {
                    return Err(ReactiveRelationPropertyRemoveError::MissingProperty(property_name.to_string()));
                }
                for component_ty in relation_instance.get_components() {
                    if let Some(component) = self.component_manager.get(&component_ty) {
                        if component.has_own_property(property_name) {
                            return Err(ReactiveRelationPropertyRemoveError::PropertyInUseByComponent(component_ty.clone()));
                        }
                    }
                }
                relation_instance.remove_property(property_name);
                Ok(())
            }
            None => Err(ReactiveRelationPropertyRemoveError::MissingInstance(edge_key.clone())),
        }
    }

    fn add_behaviour_to_all_relation_instances(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        for relation_instance in self.reactive_relation_instances.0.iter() {
            if relation_instance.relation_type_id() == relation_behaviour_ty.relation_ty {
                self.relation_behaviour_manager
                    .add_behaviour(relation_instance.clone(), &relation_behaviour_ty.behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_all_relation_components(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        for relation_instance in self.reactive_relation_instances.0.iter() {
            if relation_instance.components.contains(&component_behaviour_ty.component_ty) {
                self.relation_component_behaviour_manager
                    .add_behaviour_to_relation_component(relation_instance.clone(), component_behaviour_ty);
            }
        }
    }

    // fn commit(&self, edge_key: &RelationInstanceId) {
    //     if let Some(reactive_relation_instance) = self.get(edge_key) {
    //         self.relation_instance_manager.commit(reactive_relation_instance.into());
    //     }
    // }

    fn delete(&self, id: &RelationInstanceId) -> bool {
        if self.has(id) {
            self.unregister_reactive_instance(id);
            self.event_manager.emit_event(SystemEvent::RelationInstanceDeleted(id.clone()));
            true
        } else {
            false
        }
        // let result = self.relation_instance_manager.delete(id);
        // self.event_manager.emit_event(SystemEvent::RelationInstanceDeleted(id.clone()));
        // result
    }

    fn unregister_reactive_instance(&self, id: &RelationInstanceId) {
        match self.get(id) {
            Some(relation_instance) => {
                // Remove relation behaviours
                self.relation_behaviour_manager.remove_behaviours(relation_instance.clone());
                // Remove component behaviours
                self.relation_component_behaviour_manager.remove_behaviours_from_relation(relation_instance);
            }
            None => {
                // Remove relation behaviours
                self.relation_behaviour_manager.remove_behaviours_by_key(id);
                // Remove component behaviours
                self.relation_component_behaviour_manager.remove_behaviours_by_key(id);
            }
        }
        self.outbound_instances.remove(&id);
        self.inbound_instances.remove(&id);
        self.reactive_relation_instances.0.remove(id);
    }

    // fn import(&self, path: &str) -> Result<ReactiveRelation, ReactiveRelationImportError> {
    //     match self.relation_instance_manager.import(path) {
    //         Ok(relation_instance) => match self.create_reactive_instance(relation_instance) {
    //             Ok(reactive_relation_instance) => Ok(reactive_relation_instance),
    //             Err(error) => Err(ReactiveRelationImportError::ReactiveRelationCreation(error)),
    //         },
    //         Err(error) => Err(ReactiveRelationImportError::RelationInstanceImport(error)),
    //     }
    // }
    //
    // fn export(&self, edge_key: &RelationInstanceId, path: &str) {
    //     if self.has(edge_key) {
    //         self.commit(edge_key);
    //         self.relation_instance_manager.export(edge_key, path);
    //     }
    // }

    fn handle_component_added_events(&self) {
        let component_manager = self.component_manager.clone();
        let relation_component_behaviour_manager = self.relation_component_behaviour_manager.clone();
        let reactive_relation_instances = self.reactive_relation_instances.0.clone();
        let running = self.running.0.clone();
        if let Some(receiver) = self.system_event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED) {
            tokio::task::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_component_event) => {
                            if let Ok(type_definition_component) = TypeDefinitionComponent::try_from(type_definition_component_event.clone()) {
                                if let Some(component) = component_manager.get(&type_definition_component.component_ty) {
                                    for reactive_relation_instance in reactive_relation_instances
                                        .iter()
                                        .filter(|relation_instance| {
                                            relation_instance.relation_type_id().type_definition() == type_definition_component.type_definition
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
            tokio::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_component_event) => {
                            if let Ok(type_definition_component) = TypeDefinitionComponent::try_from(type_definition_component_event.clone()) {
                                if let Some(component) = component_manager.get(&type_definition_component.component_ty) {
                                    for reactive_relation_instance in reactive_relation_instances
                                        .iter()
                                        .filter(|relation_instance| {
                                            relation_instance.relation_type_id().type_definition() == type_definition_component.type_definition
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
            tokio::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_property_event) => {
                            if let Ok(type_definition_property) = TypeDefinitionProperty::try_from(type_definition_property_event.clone()) {
                                if let Ok(relation_ty) = RelationTypeId::try_from(&type_definition_property.type_definition) {
                                    if let Some(relation_type) = relation_type_manager.get(&relation_ty) {
                                        for reactive_relation_instance in reactive_relation_instances
                                            .iter()
                                            .filter(|relation_instance| relation_instance.relation_type_id() == relation_ty)
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
            tokio::spawn(async move {
                while running.load(Ordering::Relaxed) {
                    match receiver.try_recv() {
                        Ok(type_definition_property_event) => {
                            if let Ok(type_definition_property) = TypeDefinitionProperty::try_from(type_definition_property_event.clone()) {
                                for reactive_relation_instance in reactive_relation_instances
                                    .iter()
                                    .filter(|relation_instance| {
                                        relation_instance.relation_type_id().type_definition() == type_definition_property.type_definition
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

#[async_trait]
impl Lifecycle for ReactiveRelationManagerImpl {
    async fn post_init(&self) {
        self.subscribe_system_event(SystemEventTypes::RelationTypeComponentAdded, HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED);
        self.subscribe_system_event(SystemEventTypes::RelationTypeComponentRemoved, HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED);
        self.subscribe_system_event(SystemEventTypes::RelationTypePropertyAdded, HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED);
        self.subscribe_system_event(SystemEventTypes::RelationTypePropertyRemoved, HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED);

        self.handle_component_added_events();
        self.handle_component_removed_events();
        self.handle_property_added_events();
        self.handle_property_removed_events();
    }

    async fn pre_shutdown(&self) {
        self.running.0.store(false, Ordering::Relaxed);

        self.unsubscribe_system_event(SystemEventTypes::RelationTypePropertyRemoved, HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED);
        self.unsubscribe_system_event(SystemEventTypes::RelationTypePropertyAdded, HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED);
        self.unsubscribe_system_event(SystemEventTypes::RelationTypeComponentRemoved, HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED);
        self.unsubscribe_system_event(SystemEventTypes::RelationTypeComponentAdded, HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED);
    }
}
