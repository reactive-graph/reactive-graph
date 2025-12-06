use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use dashmap::DashMap;
use log::error;
use path_tree::PathTree;
use serde_json::Value;
use springtime_di::Component;
use springtime_di::component_alias;
use tokio::time::sleep;
use uuid::Uuid;

use reactive_graph_behaviour_model_api::BehaviourTypeId;
use reactive_graph_behaviour_model_api::BehaviourTypesContainer;
use reactive_graph_behaviour_model_api::ComponentBehaviourTypeId;
use reactive_graph_behaviour_model_api::EntityBehaviourTypeId;
use reactive_graph_behaviour_service_api::EntityBehaviourManager;
use reactive_graph_behaviour_service_api::EntityComponentBehaviourManager;
use reactive_graph_graph::ComponentContainer;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::TypeDefinitionComponent;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeDefinitionProperty;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_model_core::reactive_graph::core::event::EventProperties;
use reactive_graph_model_core::reactive_graph::core::labeled::LabeledProperties::LABEL;
use reactive_graph_reactive_model_api::ReactivePropertyContainer;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityComponentAddError;
use reactive_graph_reactive_service_api::ReactiveEntityCreationError;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveEntityPropertyAddError;
use reactive_graph_reactive_service_api::ReactiveEntityPropertyRemoveError;
use reactive_graph_reactive_service_api::ReactiveEntityRegistrationError;
use reactive_graph_reactive_service_api::ReactiveInstanceEvent;
use reactive_graph_reactive_service_api::ReactiveInstanceEventManager;
use reactive_graph_reactive_service_api::event_channels::EventChannels;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::TypeSystemEventManager;
use reactive_graph_type_system_api::TypeSystemEventSubscriber;
use reactive_graph_type_system_api::TypeSystemEventTypes;

static HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED: u128 = 0x6ba7b8109e1513d350b300c04fe530c7;
static HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED: u128 = 0x6ba8b8119e1513d350b300c04fe630c7;
static HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED: u128 = 0x6ba7b8109e2613d350b300c04fe640c7;
static HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED: u128 = 0x7ca8b8119e1523d361b311c050e630c7;

fn create_label_path_tree() -> RwLock<PathTree<Uuid>> {
    RwLock::new(PathTree::<Uuid>::new())
}

fn create_running_state() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(true))
}

fn create_event_channels() -> EventChannels {
    let event_channels = EventChannels::new();
    event_channels.insert(HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED, crossbeam::channel::unbounded());
    event_channels.insert(HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED, crossbeam::channel::unbounded());
    event_channels.insert(HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED, crossbeam::channel::unbounded());
    event_channels.insert(HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED, crossbeam::channel::unbounded());
    event_channels
}

#[derive(Component)]
pub struct ReactiveEntityManagerImpl {
    reactive_instance_event_manager: Arc<dyn ReactiveInstanceEventManager + Send + Sync>,

    type_system_event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    entity_behaviour_manager: Arc<dyn EntityBehaviourManager + Send + Sync>,

    entity_component_behaviour_manager: Arc<dyn EntityComponentBehaviourManager + Send + Sync>,

    #[component(default = "DashMap::new")]
    reactive_entity_instances: DashMap<Uuid, ReactiveEntity>,

    #[component(default = "create_label_path_tree")]
    label_path_tree: RwLock<PathTree<Uuid>>,

    #[component(default = "create_running_state")]
    running: Arc<AtomicBool>,

    #[component(default = "create_event_channels")]
    event_channels: EventChannels,
    // TODO: Type Cache
}

#[async_trait]
#[component_alias]
impl ReactiveEntityManager for ReactiveEntityManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_entity_instances.contains_key(&id)
    }

    fn get(&self, id: Uuid) -> Option<ReactiveEntity> {
        self.reactive_entity_instances.get(&id).map(|entity_instance| entity_instance.value().clone())
    }

    fn get_by_label(&self, label: &str) -> Option<ReactiveEntity> {
        let reader = self.label_path_tree.read().unwrap();
        reader.find(label).and_then(|result| self.get(*result.0))
    }

    fn get_by_label_with_params(&self, label: &str) -> Option<(ReactiveEntity, HashMap<String, String>)> {
        let reader = self.label_path_tree.read().unwrap();
        reader.find(label).and_then(|result| match self.get(*result.0) {
            Some(instance) => {
                let params = result.1.params_iter().map(|param| (param.0.to_string(), param.1.to_string())).collect();
                Some((instance, params))
            }
            None => None,
        })
    }

    fn get_all(&self) -> Vec<ReactiveEntity> {
        self.reactive_entity_instances.iter().map(|e| e.value().clone()).collect()
    }

    fn get_by_type(&self, ty: &EntityTypeId) -> Vec<ReactiveEntity> {
        self.reactive_entity_instances
            .iter()
            .filter(|e| &e.ty == ty)
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_component(&self, ty: &ComponentTypeId) -> Vec<ReactiveEntity> {
        self.reactive_entity_instances
            .iter()
            .filter(|e| e.is_a(ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> Vec<ReactiveEntity> {
        self.reactive_entity_instances
            .iter()
            .filter(|e| e.behaves_as(behaviour_ty))
            .map(|e| e.value().clone())
            .collect()
    }

    fn get_by_namespace(&self, namespace: &Namespace) -> Vec<ReactiveEntity> {
        self.reactive_entity_instances
            .iter()
            .filter(|r| &r.path() == namespace)
            .map(|r| r.value().clone())
            .collect()
    }

    fn count(&self) -> usize {
        self.reactive_entity_instances.len()
    }

    fn count_by_type(&self, ty: &EntityTypeId) -> usize {
        self.reactive_entity_instances.iter().filter(|e| &e.ty == ty).count()
    }

    fn count_by_component(&self, component_ty: &ComponentTypeId) -> usize {
        self.reactive_entity_instances.iter().filter(|e| e.is_a(component_ty)).count()
    }

    fn count_by_behaviour(&self, behaviour_ty: &BehaviourTypeId) -> usize {
        self.reactive_entity_instances.iter().filter(|e| e.behaves_as(behaviour_ty)).count()
    }

    fn get_ids(&self) -> Vec<Uuid> {
        self.reactive_entity_instances.iter().map(|e| *e.key()).collect()
    }

    fn create_reactive_entity(&self, ty: &EntityTypeId, properties: PropertyInstances) -> Result<ReactiveEntity, ReactiveEntityCreationError> {
        let entity_instance = EntityInstance::builder().ty(ty.clone()).properties(properties).build();
        self.create_reactive_instance(entity_instance)
    }

    fn create_with_id(&self, ty: &EntityTypeId, id: Uuid, properties: PropertyInstances) -> Result<ReactiveEntity, ReactiveEntityCreationError> {
        if self.has(id) {
            return Err(ReactiveEntityCreationError::UuidTaken(id));
        }

        let entity_instance = EntityInstance::builder().ty(ty.clone()).id(id).properties(properties).build();
        self.create_reactive_instance(entity_instance)
    }

    fn create_reactive_instance(&self, entity_instance: EntityInstance) -> Result<ReactiveEntity, ReactiveEntityCreationError> {
        let reactive_entity = ReactiveEntity::from(entity_instance);

        // Initialize property mutability states
        if let Some(entity_type) = self.entity_type_manager.get(&reactive_entity.ty) {
            for component_ty in entity_type.components {
                if let Some(component) = self.component_manager.get(&component_ty) {
                    for property_type in component.properties.iter() {
                        if let Some(mut property) = reactive_entity.properties.get_mut(&property_type.name) {
                            property.set_mutability(property_type.mutability);
                        }
                    }
                }
            }
            for property_type in entity_type.properties.iter() {
                if let Some(mut property) = reactive_entity.properties.get_mut(&property_type.name) {
                    property.set_mutability(property_type.mutability);
                }
            }
        }

        self.register_reactive_instance(reactive_entity)
            .map_err(ReactiveEntityCreationError::ReactiveEntityRegistrationError)
    }

    fn register_reactive_instance(&self, reactive_entity: ReactiveEntity) -> Result<ReactiveEntity, ReactiveEntityRegistrationError> {
        if self.reactive_entity_instances.contains_key(&reactive_entity.id) {
            return Err(ReactiveEntityRegistrationError::UuidTaken(reactive_entity.id));
        }
        if !self.entity_type_manager.has(&reactive_entity.ty) {
            return Err(ReactiveEntityRegistrationError::UnknownEntityType(reactive_entity.ty.clone()));
        }
        self.reactive_entity_instances.insert(reactive_entity.id, reactive_entity.clone());
        // Apply all components that are predefined in the entity type
        if let Some(components) = self.entity_type_manager.get(&reactive_entity.ty).map(|entity_type| entity_type.components) {
            components.iter().for_each(|component_ty| {
                reactive_entity.components.insert(component_ty.clone());
            });
        }
        // Add component behaviours
        self.entity_component_behaviour_manager.add_behaviours_to_entity(reactive_entity.clone());
        // Add entity behaviours
        self.entity_behaviour_manager.add_behaviours(reactive_entity.clone());
        // Register label
        if let Some(value) = reactive_entity.get(LABEL.as_ref()).and_then(|v| v.as_str().map(|s| s.to_string())) {
            let mut writer = self.label_path_tree.write().unwrap();
            let _ = writer.insert(&value, reactive_entity.id);
        }
        self.reactive_instance_event_manager
            .emit_event(ReactiveInstanceEvent::EntityInstanceCreated(reactive_entity.id));
        Ok(reactive_entity)
        //
        // match self
        //     .entity_instance_manager
        //     .create_from_instance_if_not_exist(reactive_entity_instance.clone().into())
        // {
        //     Ok(_id) => {
        //         self.reactive_entity_instances
        //
        //             .insert(reactive_entity_instance.id, reactive_entity_instance.clone());
        //         // Apply all components that are predefined in the entity type
        //         if let Some(components) = self
        //             .entity_type_manager
        //             .get(&reactive_entity_instance.ty)
        //             .map(|entity_type| entity_type.components)
        //         {
        //             components.iter().for_each(|component_ty| {
        //                 reactive_entity_instance.components.insert(component_ty.clone());
        //             });
        //         }
        //         // Add component behaviours
        //         self.entity_component_behaviour_manager
        //             .add_behaviours_to_entity(reactive_entity_instance.clone());
        //         // Add entity behaviours
        //         self.entity_behaviour_manager.add_behaviours(reactive_entity_instance.clone());
        //         // Register label
        //         if let Some(value) = reactive_entity_instance
        //             .get(LABEL.property_name())
        //             .and_then(|v| v.as_str().map(|s| s.to_string()))
        //         {
        //             let mut writer = self.label_path_tree.write().unwrap();
        //             writer.insert(&value, reactive_entity_instance.id);
        //         }
        //         self.event_manager.emit_event(SystemEvent::EntityInstanceCreated(reactive_entity_instance.id));
        //         Ok(reactive_entity_instance)
        //     }
        //     Err(e) => Err(ReactiveEntityRegistrationError::EntityInstanceCreationError(e)),
        // }
    }

    fn register_or_merge_reactive_instance(&self, entity_instance: ReactiveEntity) -> Result<ReactiveEntity, ReactiveEntityRegistrationError> {
        match self.get(entity_instance.id) {
            // No instance with the given id exists: register as new instance and return it
            None => self.register_reactive_instance(entity_instance),
            // Instance with the given id exists. Don't register but return the existing reactive instance instead of the given instance
            Some(entity_instance) => Ok(entity_instance),
        }
    }

    fn add_component(&self, id: Uuid, component_ty: &ComponentTypeId) -> Result<(), ReactiveEntityComponentAddError> {
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
                    None => Err(ReactiveEntityComponentAddError::MissingInstance(id)),
                }
            }
            None => Err(ReactiveEntityComponentAddError::MissingComponent(component_ty.clone())),
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

    fn add_property(&self, id: Uuid, property_name: &str, mutability: Mutability, value: Value) -> Result<(), ReactiveEntityPropertyAddError> {
        match self.get(id) {
            Some(entity_instance) => {
                if entity_instance.has_property(property_name) {
                    return Err(ReactiveEntityPropertyAddError::PropertyAlreadyExists(property_name.to_string()));
                }
                entity_instance.add_property(property_name, mutability, value);
                Ok(())
            }
            None => Err(ReactiveEntityPropertyAddError::MissingInstance(id)),
        }
    }

    fn remove_property(&self, id: Uuid, property_name: &str) -> Result<(), ReactiveEntityPropertyRemoveError> {
        match self.get(id) {
            Some(entity_instance) => {
                if !entity_instance.has_property(property_name) {
                    return Err(ReactiveEntityPropertyRemoveError::MissingProperty(property_name.to_string()));
                }
                for component_ty in entity_instance.get_components() {
                    if let Some(component) = self.component_manager.get(&component_ty) {
                        if component.has_own_property(property_name) {
                            return Err(ReactiveEntityPropertyRemoveError::PropertyInUseByComponent(property_name.to_string(), component_ty.clone()));
                        }
                    }
                }
                entity_instance.remove_property(property_name);
                Ok(())
            }
            None => Err(ReactiveEntityPropertyRemoveError::MissingInstance(id)),
        }
    }

    fn add_behaviour_to_all_entity_instances(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {
        for entity_instance in self.reactive_entity_instances.iter() {
            if entity_instance.ty == entity_behaviour_ty.entity_ty {
                self.entity_behaviour_manager
                    .add_behaviour(entity_instance.clone(), &entity_behaviour_ty.behaviour_ty);
            }
        }
    }

    fn add_behaviour_to_all_entity_components(&self, component_behaviour_ty: &ComponentBehaviourTypeId) {
        for entity_instance in self.reactive_entity_instances.iter() {
            if entity_instance.components.contains(&component_behaviour_ty.component_ty) {
                self.entity_component_behaviour_manager
                    .add_behaviour_to_entity_component(entity_instance.clone(), component_behaviour_ty);
            }
        }
    }

    // fn commit(&self, id: Uuid) {
    //     if let Some(reactive_entity_instance) = self.get(id) {
    //         self.entity_instance_manager.commit(reactive_entity_instance.into());
    //     }
    // }

    // TODO: Important: Check if the entity is part of relations
    fn delete(&self, id: Uuid) -> bool {
        let mut result = false;
        if self.has(id) {
            // TODO: check for relations
            result = self.unregister_reactive_instance(id);
        }
        // TODO: remove label
        // self.entity_instance_manager.delete(id);
        self.reactive_instance_event_manager
            .emit_event(ReactiveInstanceEvent::EntityInstanceDeleted(id));
        result
    }

    // TODO: fn delete_and_delete_relations(&self, id: Uuid) {}

    fn unregister_reactive_instance(&self, id: Uuid) -> bool {
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
        self.reactive_entity_instances.remove(&id).is_some()
    }

    fn handle_component_added_events(&self) {
        let component_manager = self.component_manager.clone();
        let component_behaviour_manager = self.entity_component_behaviour_manager.clone();
        let reactive_entity_instances = self.reactive_entity_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED) {
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
        let reactive_entity_instances = self.reactive_entity_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED) {
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
        let reactive_entity_instances = self.reactive_entity_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED) {
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
        let reactive_entity_instances = self.reactive_entity_instances.clone();
        let running = self.running.clone();
        if let Some(receiver) = self.event_channels.receiver(&HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED) {
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

impl TypeSystemEventSubscriber for ReactiveEntityManagerImpl {
    fn subscribe_type_system_event(&self, event_type: TypeSystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.type_system_event_manager.get_type_system_event_instance(event_type) {
            if let Some(sender) = self.event_channels.sender(&handle_id) {
                entity_instance.observe_with_handle(
                    EventProperties::EVENT.as_ref(),
                    move |v| {
                        let _ = sender.send(v.clone());
                    },
                    handle_id,
                );
            }
        }
    }

    fn unsubscribe_type_system_event(&self, event_type: TypeSystemEventTypes, handle_id: u128) {
        if let Some(entity_instance) = self.type_system_event_manager.get_type_system_event_instance(event_type) {
            entity_instance.remove_observer(EventProperties::EVENT.as_ref(), handle_id);
        }
    }
}

#[async_trait]
impl Lifecycle for ReactiveEntityManagerImpl {
    async fn post_init(&self) {
        for type_system_event_instance in self.type_system_event_manager.get_type_system_event_instances() {
            if let Err(e) = self.register_reactive_instance(type_system_event_instance) {
                error!("Failed to register type system event instance: {e:?}");
                // TODO: Propagate this error
            }
        }

        for reactive_instance_event_instance in self.reactive_instance_event_manager.get_reactive_instance_event_instances() {
            if let Err(e) = self.register_reactive_instance(reactive_instance_event_instance) {
                error!("Failed to register reactive instance event instance: {e:?}");
                // TODO: Propagate this error
            }
        }

        self.subscribe_type_system_event(TypeSystemEventTypes::EntityTypeComponentAdded, HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED);
        self.subscribe_type_system_event(TypeSystemEventTypes::EntityTypeComponentRemoved, HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED);
        self.subscribe_type_system_event(TypeSystemEventTypes::EntityTypePropertyAdded, HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED);
        self.subscribe_type_system_event(TypeSystemEventTypes::EntityTypePropertyRemoved, HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED);

        self.handle_component_added_events();
        self.handle_component_removed_events();
        self.handle_property_added_events();
        self.handle_property_removed_events();
    }

    async fn pre_shutdown(&self) {
        self.running.store(false, Ordering::Relaxed);

        self.unsubscribe_type_system_event(TypeSystemEventTypes::EntityTypePropertyRemoved, HANDLE_ID_ENTITY_TYPE_PROPERTY_REMOVED);
        self.unsubscribe_type_system_event(TypeSystemEventTypes::EntityTypePropertyAdded, HANDLE_ID_ENTITY_TYPE_PROPERTY_ADDED);
        self.unsubscribe_type_system_event(TypeSystemEventTypes::EntityTypeComponentRemoved, HANDLE_ID_ENTITY_TYPE_COMPONENT_REMOVED);
        self.unsubscribe_type_system_event(TypeSystemEventTypes::EntityTypeComponentAdded, HANDLE_ID_ENTITY_TYPE_COMPONENT_ADDED);
        for event_instance in self.reactive_instance_event_manager.get_reactive_instance_event_instances() {
            self.unregister_reactive_instance(event_instance.id);
        }
        for event_instance in self.type_system_event_manager.get_type_system_event_instances() {
            self.unregister_reactive_instance(event_instance.id);
        }
    }
}

#[cfg(test)]
mod tests {
    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_behaviour_service_impl::BehaviourSystemImpl;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypeId;
    use reactive_graph_reactive_model_impl::ReactiveEntity;
    use reactive_graph_reactive_service_api::ReactiveSystem;
    use reactive_graph_utils_test::r_string;
    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_type_system_impl::TypeSystemSystemImpl;

    use crate::ReactiveSystemImpl;

    #[test]
    fn test_register_reactive_entity_instance() {
        reactive_graph_utils_test::init_logger();

        let reactive_system = reactive_graph_di::get_container::<ReactiveSystemImpl>();
        let type_system = reactive_system.type_system_system();

        let entity_type_manager = type_system.get_entity_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();

        let entity_type = EntityType::random_type().unwrap();
        let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();

        // Check that we cannot register an reactive entity with an entity type which doesn't exist
        assert!(reactive_entity_manager.register_reactive_instance(reactive_entity.clone()).is_err());
        // assert_eq!(ReactiveEntityRegistrationError::UnknownEntityType(entity_type.ty.clone()), result.unwrap_err(), "It shouldn't be allowed to register a reactive entity for a non-existent entity type!");
        assert!(!reactive_entity_manager.has(reactive_entity.id), "There shouldn't be a reactive entity with the id");

        // Register entity type
        let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");
        // Register the reactive entity
        let reactive_entity = reactive_entity_manager
            .register_reactive_instance(reactive_entity)
            .expect("Failed to register the reactive entity");
        // Register the reactive entity
        assert!(
            reactive_entity_manager.has(reactive_entity.id),
            "The reactive entity with the id should be known by the reactive_entity_manager!"
        );
        // Get the reactive entity by id
        let id = reactive_entity.id;
        let reactive_entity = reactive_entity_manager
            .get(reactive_entity.id)
            .expect("Failed to get the reactive entity by id!");
        assert_eq!(id, reactive_entity.id, "The id of the reactive entity doesn't match!");
        assert_eq!(entity_type.ty, reactive_entity.ty, "The entity type id doesn't match!");
    }

    #[test]
    fn test_unregister_reactive_entity_instance() {
        reactive_graph_utils_test::init_logger();

        let reactive_system = reactive_graph_di::get_container::<ReactiveSystemImpl>();
        let type_system = reactive_system.type_system_system();
        let entity_type_manager = type_system.get_entity_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();

        let entity_ty = EntityTypeId::random_type_id().unwrap();
        let entity_type = EntityType::builder()
            .ty(entity_ty)
            .properties(PropertyTypes::new_with_string_property(r_string()))
            .build();
        // Register entity type
        let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");

        let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
        let id = reactive_entity.id;

        // Register the reactive entity
        let _reactive_entity = reactive_entity_manager
            .register_reactive_instance(reactive_entity)
            .expect("Failed to register the reactive entity");

        assert!(reactive_entity_manager.has(id), "The reactive entity should be registered!");
        assert!(reactive_entity_manager.unregister_reactive_instance(id), "The reactive entity should have been unregistered!");
        assert!(!reactive_entity_manager.has(id), "The reactive entity shouldn't be registered anymore!");
    }

    #[test]
    fn test_not_register_twice_reactive_entity_instance() {
        reactive_graph_utils_test::init_logger();

        let reactive_system = reactive_graph_di::get_container::<ReactiveSystemImpl>();
        let type_system = reactive_system.type_system_system();
        let entity_type_manager = type_system.get_entity_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();

        let entity_ty = EntityTypeId::random_type_id().unwrap();
        let entity_type = EntityType::builder()
            .ty(entity_ty)
            .properties(PropertyTypes::new_with_string_property(r_string()))
            .build();

        let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();
        let id = reactive_entity.id;

        // Check that we cannot create an entity instance with a type which doesn't exist
        assert!(
            reactive_entity_manager.register_reactive_instance(reactive_entity.clone()).is_err(),
            "The reactive entity shouldn't have been registered because the entity type was not registered!"
        );

        assert!(!reactive_entity_manager.has(id), "There shouldn't be a reactive entity with id!");
        assert_eq!(reactive_entity_manager.count(), 0);

        // Register entity type
        let _entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");

        let reactive_entity = reactive_entity_manager
            .register_reactive_instance(reactive_entity)
            .expect("Failed to register the reactive entity!");

        assert!(reactive_entity_manager.has(id), "The reactive entity with id should be registered!");
        assert_eq!(reactive_entity_manager.count(), 1);

        assert!(
            reactive_entity_manager.register_reactive_instance(reactive_entity).is_err(),
            "The reactive entity was registered twice!"
        );

        assert!(reactive_entity_manager.has(id), "The reactive entity with id should be registered!");
        assert_eq!(reactive_entity_manager.count(), 1);
    }
}
