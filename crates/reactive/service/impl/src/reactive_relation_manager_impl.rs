use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use dashmap::DashSet;
use reactive_graph_reactive_service_api::EventChannels;
use reactive_graph_reactive_service_api::ReactiveInstanceEvent;
use reactive_graph_reactive_service_api::ReactiveInstanceEventManager;
use reactive_graph_reactive_service_api::ReactiveRelationComponentAddError;
use reactive_graph_reactive_service_api::ReactiveRelationComponentRemoveError;
use reactive_graph_reactive_service_api::ReactiveRelationCreationError;
use reactive_graph_reactive_service_api::ReactiveRelationPropertyAddError;
use reactive_graph_reactive_service_api::ReactiveRelationPropertyRemoveError;
use reactive_graph_reactive_service_api::ReactiveRelationRegistrationError;
use serde_json::Value;
use springtime_di::Component;
use tokio::time::sleep;
use tokio::time::Duration;
use uuid::Uuid;

use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_model_api::BehaviourTypesContainer;
use reactive_graph_behaviour_model_api::ComponentBehaviourTypeId;
use reactive_graph_behaviour_model_api::RelationBehaviourTypeId;
use reactive_graph_behaviour_service_api::RelationBehaviourManager;
use reactive_graph_behaviour_service_api::RelationComponentBehaviourManager;
use reactive_graph_graph::ComponentContainer;
use reactive_graph_graph::ComponentOrEntityTypeId;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinitionComponent;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeDefinitionProperty;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_api::ReactiveInstance;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_runtime_model::EventProperties::EVENT;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_api::TypeSystemEventSubscriber;
use reactive_graph_type_system_api::TypeSystemEventTypes;
use springtime_di::component_alias;

static HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED: u128 = 0x6ba7b9210e1513d350b300c04fe530c7;
static HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED: u128 = 0x6ba8b8119e1513ee59b300c04fe630c7;
static HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED: u128 = 0x6bb9b9232e1513d350b300c04fe530c7;
static HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED: u128 = 0x6ba8b8339e1535ee5bd300c0410630c7;

pub struct OutboundInstances(DashMap<Uuid, DashSet<RelationInstanceId>>);

impl OutboundInstances {
    pub fn new() -> Self {
        OutboundInstances(DashMap::new())
    }

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

    pub fn get(&self, id: Uuid) -> Option<DashSet<RelationInstanceId>> {
        self.0.get(&id).map(|entry| entry.value().clone())
    }
}

impl Default for OutboundInstances {
    fn default() -> Self {
        Self::new()
    }
}

pub struct InboundInstances(DashMap<Uuid, DashSet<RelationInstanceId>>);

impl InboundInstances {
    pub fn new() -> Self {
        InboundInstances(DashMap::new())
    }

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

    pub fn get(&self, id: Uuid) -> Option<DashSet<RelationInstanceId>> {
        self.0.get(&id).map(|entry| entry.value().clone())
    }
}

impl Default for InboundInstances {
    fn default() -> Self {
        Self::new()
    }
}

fn create_running_state() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(true))
}

fn create_event_channels() -> EventChannels {
    let event_channels = EventChannels::new();
    event_channels.insert(HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED, crossbeam::channel::unbounded());
    event_channels.insert(HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED, crossbeam::channel::unbounded());
    event_channels.insert(HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED, crossbeam::channel::unbounded());
    event_channels.insert(HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED, crossbeam::channel::unbounded());
    event_channels
}

#[derive(Component)]
pub struct ReactiveRelationManagerImpl {
    reactive_instance_event_manager: Arc<dyn ReactiveInstanceEventManager + Send + Sync>,

    type_system_event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,

    relation_behaviour_manager: Arc<dyn RelationBehaviourManager + Send + Sync>,

    relation_component_behaviour_manager: Arc<dyn RelationComponentBehaviourManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    reactive_relation_instances: DashMap<RelationInstanceId, ReactiveRelation>, // ReactiveRelations,

    #[component(default = "OutboundInstances::new")]
    outbound_instances: OutboundInstances, // DashMap<Uuid, DashSet<RelationInstanceId>>, //

    #[component(default = "InboundInstances::new")]
    inbound_instances: InboundInstances, // DashMap<Uuid, DashSet<RelationInstanceId>>, //

    #[component(default = "create_running_state")]
    running: Arc<AtomicBool>, // RunningState,

    #[component(default = "create_event_channels")]
    event_channels: EventChannels,
}

#[async_trait]
#[component_alias]
impl ReactiveRelationManager for ReactiveRelationManagerImpl {
    fn has(&self, id: &RelationInstanceId) -> bool {
        self.reactive_relation_instances.contains_key(id)
    }

    fn get(&self, id: &RelationInstanceId) -> Option<ReactiveRelation> {
        self.reactive_relation_instances.get(id).map(|r| r.value().clone())
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<ReactiveRelation> {
        self.outbound_instances
            .get(outbound_entity_id)
            .and_then(|outbound_instances| outbound_instances.iter().map(|id| self.get(id.deref())).collect())
            .unwrap_or_default()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<ReactiveRelation> {
        self.inbound_instances
            .get(inbound_entity_id)
            .and_then(|inbound_instances| inbound_instances.iter().map(|id| self.get(id.deref())).collect())
            .unwrap_or_default()
    }

    fn get_all(&self) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances.iter().map(|r| r.value().clone()).collect()
    }

    fn get_by_type(&self, ty: &RelationTypeId) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances
            .iter()
            .filter(|r| &r.relation_type_id() == ty)
            .map(|r| r.value().clone())
            .collect()
    }

    fn get_by_component(&self, ty: &ComponentTypeId) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances
            .iter()
            .filter(|e| e.is_a(ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances
            .iter()
            .filter(|e| e.behaves_as(behaviour_ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<ReactiveRelation> {
        self.reactive_relation_instances
            .iter()
            .filter(|r| r.namespace() == namespace)
            .map(|r| r.value().clone())
            .collect()
    }

    fn count(&self) -> usize {
        self.reactive_relation_instances.len()
    }

    fn count_by_type(&self, ty: &RelationTypeId) -> usize {
        self.reactive_relation_instances.iter().filter(|r| &r.relation_type_id() == ty).count()
    }

    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize {
        self.reactive_relation_instances.iter().filter(|r| r.is_a(component_ty)).count()
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_relation_instances.iter().filter(|r| r.behaves_as(behaviour_ty)).count()
    }

    fn get_relation_instance_ids(&self) -> Vec<RelationInstanceId> {
        self.reactive_relation_instances.iter().map(|e| e.key().clone()).collect()
    }

    fn create_reactive_relation(&self, id: &RelationInstanceId, properties: PropertyInstances) -> Result<ReactiveRelation, ReactiveRelationCreationError> {
        let relation_instance = RelationInstance::builder()
            .outbound_id(id.outbound_id)
            .ty(id.ty.clone())
            .inbound_id(id.inbound_id)
            .properties(properties)
            .build();
        self.create_reactive_instance(relation_instance)
        // match self.relation_instance_manager.create(relation_instance_id, properties) {
        //     Ok(relation_instance_id) => match self.relation_instance_manager.get(&relation_instance_id) {
        //         Some(relation_instance) => self.create_reactive_instance(relation_instance),
        //         None => Err(ReactiveRelationCreationError::MissingInstance(relation_instance_id)),
        //     },
        //     Err(e) => Err(ReactiveRelationCreationError::RelationInstanceCreationError(e)),
        // }
    }

    fn create_reactive_instance(&self, reactive_relation_instance: RelationInstance) -> Result<ReactiveRelation, ReactiveRelationCreationError> {
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
                        return Err(ReactiveRelationCreationError::OutboundEntityDoesNotHaveComponent(outbound.id, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if &outbound.ty != entity_ty {
                        return Err(ReactiveRelationCreationError::OutboundEntityIsNotOfType(outbound.id, outbound.ty.clone(), entity_ty.clone()));
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
                        return Err(ReactiveRelationCreationError::InboundEntityDoesNotHaveComponent(inbound.id, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if &inbound.ty != entity_ty {
                        return Err(ReactiveRelationCreationError::InboundEntityIsNotOfType(inbound.id, inbound.ty.clone(), entity_ty.clone()));
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

    fn register_reactive_instance(&self, reactive_relation: ReactiveRelation) -> Result<ReactiveRelation, ReactiveRelationRegistrationError> {
        let id = reactive_relation.id();
        if self.reactive_relation_instances.contains_key(&id) {
            return Err(ReactiveRelationRegistrationError::RelationInstanceAlreadyExists(id.clone()));
        }
        // TODO: check if id already exists
        self.reactive_relation_instances.insert(id.clone(), reactive_relation.clone());
        self.outbound_instances.insert(&id);
        self.inbound_instances.insert(&id);
        // Apply all components that are predefined in the relation type
        let relation_ty = reactive_relation.relation_type_id();
        if let Some(components) = self.relation_type_manager.get(&relation_ty).map(|relation_type| relation_type.components) {
            components.iter().for_each(|component_ty| {
                reactive_relation.components.insert(component_ty.clone());
            });
        }
        // Add component behaviours
        self.relation_component_behaviour_manager.add_behaviours_to_relation(reactive_relation.clone());
        // Add relation behaviours
        self.relation_behaviour_manager.add_behaviours(reactive_relation.clone());
        self.reactive_instance_event_manager
            .emit_event(ReactiveInstanceEvent::RelationInstanceCreated(id));
        Ok(reactive_relation)

        // match self
        //     .relation_instance_manager
        //     .create_from_instance_if_not_exist(reactive_relation_instance.clone().into())
        // {
        //     Ok(relation_instance_id) => {
        //         self.reactive_relation_instances.insert(relation_instance_id.clone(), reactive_relation_instance.clone());
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
        //         self.event_manager.emit_event(SystemEvent::RelationInstanceCreated(relation_instance_id));
        //         Ok(reactive_relation_instance)
        //     }
        //     Err(e) => Err(ReactiveRelationRegistrationError::RelationInstanceCreationError(e)),
        // }
    }

    fn register_or_merge_reactive_instance(&self, relation_instance: ReactiveRelation) -> Result<ReactiveRelation, ReactiveRelationRegistrationError> {
        let id = relation_instance.id();
        match self.get(&id) {
            // No instance with the given relation instance id exists yet, try to register the given reactive instance
            None => self.register_reactive_instance(relation_instance),
            // Instance with the given relation instance id exists. Don't register but return the existing reactive instance instead of the given instance
            Some(reactive_relation_instance) => Ok(reactive_relation_instance),
        }
    }

    fn add_component(&self, id: &RelationInstanceId, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationComponentAddError> {
        let Some(component) = self.component_manager.get(component_ty) else {
            return Err(ReactiveRelationComponentAddError::ComponentNotRegistered(component_ty.clone()));
        };
        let Some(reactive_relation) = self.get(id) else {
            return Err(ReactiveRelationComponentAddError::MissingInstance(id.clone()));
        };
        if reactive_relation.is_a(component_ty) {
            return Err(ReactiveRelationComponentAddError::IsAlreadyA(component_ty.clone()));
        }
        // Add components with properties
        reactive_relation.add_component_with_properties(&component);
        // Add component behaviours
        self.relation_component_behaviour_manager
            .add_behaviours_to_relation_component(reactive_relation, component);
        Ok(())
    }

    fn remove_component(&self, id: &RelationInstanceId, component_ty: &ComponentTypeId) -> Result<(), ReactiveRelationComponentRemoveError> {
        let Some(reactive_relation) = self.get(id) else {
            return Err(ReactiveRelationComponentRemoveError::MissingInstance(id.clone()));
        };
        if !reactive_relation.is_a(component_ty) {
            return Err(ReactiveRelationComponentRemoveError::IsNotA(component_ty.clone()));
        }
        let Some(component) = self.component_manager.get(component_ty) else {
            return Err(ReactiveRelationComponentRemoveError::ComponentNotRegistered(component_ty.clone()));
        };
        // Remove component
        reactive_relation.remove_component(component_ty);
        //
        // We do not remove properties because we cannot ensure that the removal is intended
        // (At least yet)
        //
        // Remove component behaviours
        self.relation_component_behaviour_manager
            .remove_behaviours_from_relation_component(reactive_relation, component);
        Ok(())
    }

    fn add_property(
        &self,
        relation_instance_id: &RelationInstanceId,
        property_name: &str,
        mutability: Mutability,
        value: Value,
    ) -> Result<(), ReactiveRelationPropertyAddError> {
        match self.get(relation_instance_id) {
            Some(relation_instance) => {
                if relation_instance.has_property(property_name) {
                    return Err(ReactiveRelationPropertyAddError::PropertyAlreadyExists(property_name.to_string()));
                }
                relation_instance.add_property(property_name, mutability, value);
                Ok(())
            }
            None => Err(ReactiveRelationPropertyAddError::MissingInstance(relation_instance_id.clone())),
        }
    }

    fn remove_property(&self, relation_instance_id: &RelationInstanceId, property_name: &str) -> Result<(), ReactiveRelationPropertyRemoveError> {
        match self.get(relation_instance_id) {
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
            None => Err(ReactiveRelationPropertyRemoveError::MissingInstance(relation_instance_id.clone())),
        }
    }

    fn add_behaviour_to_all_relation_instances(&self, relation_behaviour_ty: &RelationBehaviourTypeId) {
        for relation_instance in self.reactive_relation_instances.iter() {
            if relation_instance.relation_type_id() == relation_behaviour_ty.relation_ty {
                self.relation_behaviour_manager
                    .add_behaviour(relation_instance.clone(), &relation_behaviour_ty.behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_all_relation_components(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        for relation_instance in self.reactive_relation_instances.iter() {
            if relation_instance.components.contains(&component_behaviour_ty.component_ty) {
                self.relation_component_behaviour_manager
                    .add_behaviour_to_relation_component(relation_instance.clone(), component_behaviour_ty);
            }
        }
    }

    // fn commit(&self, relation_instance_id: &RelationInstanceId) {
    //     if let Some(reactive_relation_instance) = self.get(relation_instance_id) {
    //         self.relation_instance_manager.commit(reactive_relation_instance.into());
    //     }
    // }

    fn delete(&self, id: &RelationInstanceId) -> bool {
        if self.has(id) {
            self.unregister_reactive_instance(id);
            self.reactive_instance_event_manager
                .emit_event(ReactiveInstanceEvent::RelationInstanceDeleted(id.clone()));
            true
        } else {
            false
        }
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
        self.outbound_instances.remove(id);
        self.inbound_instances.remove(id);
        self.reactive_relation_instances.remove(id);
    }

    fn handle_component_added_events(&self) {
        let component_manager = self.component_manager.clone();
        let relation_component_behaviour_manager = self.relation_component_behaviour_manager.clone();
        let reactive_relation_instances = self.reactive_relation_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED) {
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
        let reactive_relation_instances = self.reactive_relation_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED) {
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
        let reactive_relation_instances = self.reactive_relation_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED) {
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
        let reactive_relation_instances = self.reactive_relation_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED) {
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

impl TypeSystemEventSubscriber for ReactiveRelationManagerImpl {
    fn subscribe_type_system_event(&self, system_event_type: TypeSystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.type_system_event_manager.get_type_system_event_instance(system_event_type) {
            if let Some(sender) = self.event_channels.sender(&handle_id) {
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

    fn unsubscribe_type_system_event(&self, system_event_type: TypeSystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.type_system_event_manager.get_type_system_event_instance(system_event_type) {
            entity_instance.remove_observer(&EVENT.property_name(), handle_id);
        }
    }
}

#[async_trait]
impl Lifecycle for ReactiveRelationManagerImpl {
    async fn post_init(&self) {
        self.subscribe_type_system_event(TypeSystemEventTypes::RelationTypeComponentAdded, HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED);
        self.subscribe_type_system_event(TypeSystemEventTypes::RelationTypeComponentRemoved, HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED);
        self.subscribe_type_system_event(TypeSystemEventTypes::RelationTypePropertyAdded, HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED);
        self.subscribe_type_system_event(TypeSystemEventTypes::RelationTypePropertyRemoved, HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED);

        self.handle_component_added_events();
        self.handle_component_removed_events();
        self.handle_property_added_events();
        self.handle_property_removed_events();
    }

    async fn pre_shutdown(&self) {
        self.running.store(false, Ordering::Relaxed);

        self.unsubscribe_type_system_event(TypeSystemEventTypes::RelationTypePropertyRemoved, HANDLE_ID_RELATION_TYPE_PROPERTY_REMOVED);
        self.unsubscribe_type_system_event(TypeSystemEventTypes::RelationTypePropertyAdded, HANDLE_ID_RELATION_TYPE_PROPERTY_ADDED);
        self.unsubscribe_type_system_event(TypeSystemEventTypes::RelationTypeComponentRemoved, HANDLE_ID_RELATION_TYPE_COMPONENT_REMOVED);
        self.unsubscribe_type_system_event(TypeSystemEventTypes::RelationTypeComponentAdded, HANDLE_ID_RELATION_TYPE_COMPONENT_ADDED);
    }
}
