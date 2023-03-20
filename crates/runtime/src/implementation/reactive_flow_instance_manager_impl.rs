use std::collections::BTreeMap;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use dashmap::DashMap;
use indradb::EdgeKey;
use log::debug;
use log::error;
use log::trace;
use path_tree::PathTree;
use serde_json::Value;
use uuid::Uuid;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::FlowInstanceManager;
use crate::api::FlowTypeManager;
use crate::api::Lifecycle;
use crate::api::ReactiveEntityInstanceManager;
use crate::api::ReactiveFlowInstanceCreationError;
use crate::api::ReactiveFlowInstanceImportError;
use crate::api::ReactiveFlowInstanceManager;
use crate::api::ReactiveRelationInstanceManager;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::builder::FlowInstanceBuilder;
use crate::di::*;
use crate::model::EntityInstance;
use crate::model::ExtensionContainer;
use crate::model::FlowInstance;
use crate::model::FlowTypeId;
use crate::model::Mutability::Mutable;
use crate::model::MutablePropertyInstanceSetter;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyInstanceGetter;
use crate::model::PropertyInstanceSetter;
use crate::model::PropertyTypeDefinition;
use crate::model::ReactiveEntityInstance;
use crate::model::ReactiveFlowInstance;
use crate::model::ReactivePropertyContainer;
use crate::model::ReactiveRelationInstance;
use crate::model::RelationInstance;
use crate::model::TypeDefinitionGetter;
use crate::model_flow::EXTENSION_FLOW_RESOLVE_EXISTING_INSTANCE;
use crate::model_flow::EXTENSION_FLOW_UUID_TYPE_EXTENSION;
use crate::model_flow::EXTENSION_FLOW_UUID_TYPE_VARIABLE;
use crate::model_runtime::LabeledProperties::LABEL;
use crate::plugins::FlowInstanceProvider;
use crate::plugins::SystemEvent;

#[wrapper]
pub struct ReactiveFlowInstancesStorage(RwLock<BTreeMap<Uuid, Arc<ReactiveFlowInstance>>>);

#[provides]
fn create_reactive_flow_instances_storage() -> ReactiveFlowInstancesStorage {
    ReactiveFlowInstancesStorage(RwLock::new(BTreeMap::new()))
}

#[wrapper]
pub struct FlowInstanceProviders(DashMap<Uuid, Arc<dyn FlowInstanceProvider>>);

#[provides]
fn create_flow_instance_providers() -> FlowInstanceProviders {
    FlowInstanceProviders(DashMap::new())
}

#[wrapper]
pub struct LabelPathTree(RwLock<PathTree<Uuid>>);

#[provides]
fn create_label_path_tree() -> LabelPathTree {
    LabelPathTree(RwLock::new(PathTree::<Uuid>::new()))
}

#[component]
pub struct ReactiveFlowInstanceManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    flow_type_manager: Wrc<dyn FlowTypeManager>,

    flow_instance_manager: Wrc<dyn FlowInstanceManager>,

    reactive_entity_instance_manager: Wrc<dyn ReactiveEntityInstanceManager>,

    reactive_relation_instance_manager: Wrc<dyn ReactiveRelationInstanceManager>,

    reactive_flow_instances: ReactiveFlowInstancesStorage,

    flow_instance_providers: FlowInstanceProviders,

    label_path_tree: LabelPathTree,
}

impl ReactiveFlowInstanceManagerImpl {
    fn get_entity_instance_id_by_extension(&self, entity_instance: &EntityInstance, variables: &HashMap<String, Value>) -> Uuid {
        // Resolve an existing entity instance: Do not replace the uuid
        if entity_instance.has_own_extension(&EXTENSION_FLOW_RESOLVE_EXISTING_INSTANCE.clone()) {
            return entity_instance.id;
        }
        // Parse the UUID from the variable with the name specified by the extension value.
        if let Some(id) = entity_instance
            .get_own_extension(&EXTENSION_FLOW_UUID_TYPE_VARIABLE.clone())
            .and_then(|extension| extension.extension.as_str().map(|s| s.to_string()))
            .and_then(|variable_name| variables.get(&variable_name))
            .and_then(|variable_value| variable_value.as_str())
            .and_then(|variable_value| Uuid::parse_str(variable_value).ok())
        {
            return id;
        }
        // Parse the UUID from the extension value.
        if let Some(id) = entity_instance
            .get_own_extension(&EXTENSION_FLOW_UUID_TYPE_EXTENSION.clone())
            .and_then(|extension| extension.extension.as_str().map(|s| s.to_string()))
            .and_then(|extension_value| Uuid::parse_str(extension_value.as_str()).ok())
        {
            return id;
        }
        // Default: Generate a random UUID
        Uuid::new_v4()
    }
}

#[async_trait]
#[provides]
impl ReactiveFlowInstanceManager for ReactiveFlowInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_flow_instances.0.read().unwrap().contains_key(&id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlowInstance>> {
        let reader = self.reactive_flow_instances.0.read().unwrap();
        reader.get(&id).cloned()
    }

    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveFlowInstance>> {
        let reader = self.label_path_tree.0.read().unwrap();
        reader.find(label).and_then(|result| self.get(*result.0))
    }

    fn get_all(&self) -> Vec<Arc<ReactiveFlowInstance>> {
        let reader = self.reactive_flow_instances.0.read().unwrap();
        reader.values().cloned().collect()
    }

    fn count_flow_instances(&self) -> usize {
        self.reactive_flow_instances.0.read().unwrap().len()
    }

    fn create(&self, flow_instance: FlowInstance) -> Result<Arc<ReactiveFlowInstance>, ReactiveFlowInstanceCreationError> {
        let reactive_flow_instance = ReactiveFlowInstance::try_from(flow_instance);
        if reactive_flow_instance.is_err() {
            return Err(ReactiveFlowInstanceCreationError::ReactiveFlowInstanceConstructionError(
                reactive_flow_instance.err().unwrap(),
            ));
        }
        let reactive_flow_instance = reactive_flow_instance.unwrap();
        let reactive_flow_instance = Arc::new(reactive_flow_instance);
        self.register_flow_instance_and_reactive_instances(reactive_flow_instance.clone());
        Ok(reactive_flow_instance)
    }

    fn create_from_type(
        &self,
        ty: &FlowTypeId,
        variables: HashMap<String, Value>,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveFlowInstance>, ReactiveFlowInstanceCreationError> {
        let flow_type = self
            .flow_type_manager
            .get(ty)
            .ok_or_else(|| ReactiveFlowInstanceCreationError::FlowTypeDoesntExist(ty.clone()))?;
        for variable in flow_type.variables.iter() {
            if !variables.contains_key(variable.name.as_str()) {
                return Err(ReactiveFlowInstanceCreationError::MissingVariable(variable.name.clone()));
            }
        }
        let wrapper_entity_type = self
            .entity_type_manager
            .get(&flow_type.wrapper_entity_instance.ty)
            .ok_or_else(|| ReactiveFlowInstanceCreationError::EntityTypeDoesntExist(flow_type.wrapper_entity_instance.ty.clone()))?;
        let mut wrapper_entity_instance = flow_type.wrapper_entity_instance.clone();
        let wrapper_entity_instance_id = self.get_entity_instance_id_by_extension(&wrapper_entity_instance, &variables);
        let mut entity_instance_id_mapping: HashMap<Uuid, Uuid> = HashMap::new();
        entity_instance_id_mapping.insert(wrapper_entity_instance.id, wrapper_entity_instance_id);
        wrapper_entity_instance.id = wrapper_entity_instance_id;

        // Add properties from entity_type if not existing
        for property in wrapper_entity_type.properties.iter() {
            trace!("Adding property {} from entity type {}", &property.name, &wrapper_entity_type.type_definition().to_string());
            if !wrapper_entity_instance.properties.contains_key(&property.name) {
                wrapper_entity_instance
                    .properties
                    .insert(property.name.clone(), property.data_type.default_value());
            }
        }

        // Add properties from components if not existing
        for component_ty in wrapper_entity_type.components.iter() {
            if let Some(component) = self.component_manager.get(component_ty) {
                for property in component.properties {
                    trace!("Adding property {} from component {}", &property.name, &component_ty.type_definition().to_string());
                    if !wrapper_entity_instance.properties.contains_key(&property.name) {
                        //
                        // TODO: templating using the variables
                        //
                        wrapper_entity_instance
                            .properties
                            .insert(property.name.clone(), property.data_type.default_value());
                    }
                }
            }
        }

        for (property_name, property_value) in properties.iter() {
            trace!("Setting property {} with value {} from parameter", &property_name, property_value.clone());
            wrapper_entity_instance.set(property_name, property_value.clone());
        }

        let mut flow_instance_builder = FlowInstanceBuilder::new(wrapper_entity_instance);
        flow_instance_builder.name(flow_type.type_name()); // Default name for the flow instance is the flow type name.
        flow_instance_builder.description(flow_type.description.clone());
        for entity_instance in flow_type.entity_instances {
            let entity_type = self
                .entity_type_manager
                .get(&entity_instance.ty)
                .ok_or_else(|| ReactiveFlowInstanceCreationError::EntityTypeDoesntExist(entity_instance.ty.clone()))?;
            let entity_instance_id = self.get_entity_instance_id_by_extension(&entity_instance, &variables);
            entity_instance_id_mapping.insert(entity_instance.id, entity_instance_id);
            let mut entity_instance_copy = entity_instance.clone();
            entity_instance_copy.id = entity_instance_id;

            // Add properties from entity_type if not existing
            for property in entity_type.properties.iter() {
                trace!("Adding property {} from entity type {}", &property.name, &entity_type.type_definition().to_string());
                if !entity_instance_copy.properties.contains_key(&property.name) {
                    //
                    // TODO: templating using the variables
                    //
                    entity_instance_copy
                        .properties
                        .insert(property.name.clone(), property.data_type.default_value());
                }
            }

            // Add properties from components if not existing
            for component_ty in entity_type.components.iter() {
                if let Some(component) = self.component_manager.get(component_ty) {
                    for property in component.properties {
                        trace!("Adding property {} from component {}", &property.name, component_ty.type_definition().to_string());
                        if !entity_instance_copy.properties.contains_key(&property.name) {
                            entity_instance_copy
                                .properties
                                .insert(property.name.clone(), property.data_type.default_value());
                        }
                    }
                }
            }

            // TODO: templating using the variables
            flow_instance_builder.entity(entity_instance_copy);
        }
        for (uf, ut) in entity_instance_id_mapping.iter() {
            trace!("Mapping flow type entity instance id {uf} to actual entity instance id {ut}");
        }
        for relation_instance in flow_type.relation_instances {
            let relation_ty = relation_instance.relation_type_id();
            trace!("Relation instance type: {}", &relation_instance.ty);
            let relation_type = self
                .relation_type_manager
                .get(&relation_ty)
                .ok_or(ReactiveFlowInstanceCreationError::RelationTypeDoesntExist(relation_ty))?;
            let mut relation_instance_copy = relation_instance.clone();
            match entity_instance_id_mapping.get(&relation_instance.outbound_id) {
                Some(replaced_id) => relation_instance_copy.outbound_id = *replaced_id,
                None => return Err(ReactiveFlowInstanceCreationError::InvalidOutboundId(relation_instance.outbound_id)),
            }
            match entity_instance_id_mapping.get(&relation_instance.inbound_id) {
                Some(replaced_id) => relation_instance_copy.inbound_id = *replaced_id,
                None => return Err(ReactiveFlowInstanceCreationError::InvalidInboundId(relation_instance.inbound_id)),
            }

            // Add properties from relation type if not existing
            for property in relation_type.properties.iter() {
                if !relation_instance_copy.properties.contains_key(&property.name) {
                    //
                    // TODO: templating using the variables
                    //
                    relation_instance_copy
                        .properties
                        .insert(property.name.clone(), property.data_type.default_value());
                }
            }

            // Add properties from components if not existing
            for component_ty in relation_type.components.iter() {
                if let Some(component) = self.component_manager.get(component_ty) {
                    for property in component.properties {
                        if !relation_instance_copy.properties.contains_key(&property.name) {
                            //
                            // TODO: templating using the variables
                            //
                            relation_instance_copy
                                .properties
                                .insert(property.name.clone(), property.data_type.default_value());
                        }
                    }
                }
            }

            // TODO: templating using the variables
            flow_instance_builder.relation(relation_instance_copy);
        }
        let flow_instance = flow_instance_builder.build();
        trace!("{:?}", flow_instance);
        match ReactiveFlowInstance::try_from(flow_instance) {
            Ok(reactive_flow_instance) => {
                let reactive_flow_instance = Arc::new(reactive_flow_instance);
                self.register_flow_instance_and_reactive_instances(reactive_flow_instance.clone());

                // Set or create properties given with the flow type instantiation
                if let Some(wrapper_entity_instance) = reactive_flow_instance.get_wrapper_entity_instance() {
                    for (property_name, property_value) in properties.iter() {
                        if !wrapper_entity_instance.has_property(property_name) {
                            trace!("Adding parameter property {} with value {} from parameter", property_name, property_value.clone());
                            wrapper_entity_instance.add_property(property_name, Mutable, property_value.clone());
                        } else {
                            trace!("Set parameter property {} with value {} from parameter", property_name, property_value.clone());
                            wrapper_entity_instance.set(property_name, property_value.clone());
                        }
                    }
                }

                Ok(reactive_flow_instance)
            }
            Err(e) => Err(ReactiveFlowInstanceCreationError::ReactiveFlowInstanceConstructionError(e)),
        }
    }

    fn register_flow_instance_and_reactive_instances(&self, reactive_flow_instance: Arc<ReactiveFlowInstance>) {
        if !self.has(reactive_flow_instance.id) {
            {
                // Step 1: Register all entity instances (if not already registered by uuid)
                let mut entity_instances = reactive_flow_instance.entity_instances.write().unwrap();
                let mut replaced_entity_instances = HashMap::<Uuid, Arc<ReactiveEntityInstance>>::new();
                for (uuid, entity_instance) in entity_instances.iter() {
                    // if let Some(entity_type) = self.entity_type_manager.get(&entity_instance.type_name) {
                    //     for property in entity_type.properties.iter() {}
                    // }
                    match self
                        .reactive_entity_instance_manager
                        .register_or_merge_reactive_instance(entity_instance.clone())
                    {
                        Ok(entity_instance) => {
                            // Replace the entity instance with the actual registered instance instead
                            replaced_entity_instances.insert(*uuid, entity_instance);
                        }
                        Err(e) => {
                            // This happens when a entity instance doesn't exist and cannot be created
                            debug!("Failed to register entity instance {}: {:?}", uuid, e);
                        }
                    }
                    // let entity_instance = self
                    //     .reactive_entity_instance_manager
                    //     .register_or_merge_reactive_instance(entity_instance.clone());
                    // // Replace the entity instance with the actual registered instance instead
                    // replaced_entity_instances.insert(*uuid, entity_instance);
                }

                // Step 2: Replace the entity instances of the flow instance with the actual registered entity instances
                entity_instances.clear();
                for (uuid, entity_instance) in replaced_entity_instances.iter() {
                    entity_instances.insert(*uuid, entity_instance.clone());
                }

                // Step 3: Recreate the reactive relation instances
                // Because the entity instances might have been replaced by the actual registered entity instances
                let mut relation_instances = reactive_flow_instance.relation_instances.write().unwrap();
                let mut replaced_relation_instances = HashMap::<EdgeKey, Arc<ReactiveRelationInstance>>::new();
                for (edge_key, relation_instance) in relation_instances.iter() {
                    let inbound_id = relation_instance.inbound.id;
                    let outbound_id = relation_instance.outbound.id;

                    let recreated_relation_instance = Arc::new(ReactiveRelationInstance::new_from_instance(
                        entity_instances.get(&outbound_id).unwrap().clone(),
                        entity_instances.get(&inbound_id).unwrap().clone(),
                        RelationInstance::from(relation_instance.clone()),
                    ));
                    replaced_relation_instances.insert(edge_key.clone(), recreated_relation_instance);
                    // relation_instance.inbound = entity_instances.get(&inbound_id).unwrap().clone();
                    // relation_instance.outbound = entity_instances.get(&outbound_id).unwrap().clone();
                }

                // Step 4: Replace the relation instances of the flow instance with the recreated relation instances
                relation_instances.clear();
                for (edge_key, relation_instance) in replaced_relation_instances.iter() {
                    relation_instances.insert(edge_key.clone(), relation_instance.clone());
                }

                // Step 5: Register all (recreated) relation instances (if not already registered by edge_key)
                let mut replaced_relation_instances = HashMap::<EdgeKey, Arc<ReactiveRelationInstance>>::new();
                for (edge_key, relation_instance) in relation_instances.iter() {
                    match self
                        .reactive_relation_instance_manager
                        .register_or_merge_reactive_instance(relation_instance.clone())
                    {
                        Ok(relation_instance) => {
                            // Replace the relation instance with the actual registered instance
                            replaced_relation_instances.insert(edge_key.clone(), relation_instance);
                        }
                        Err(e) => {
                            // This happens when a relation instance doesn't exist and cannot be created
                            debug!("Failed to register relation instance {:?}: {:?}", edge_key, e);
                        }
                    }
                }

                // Step 6: Replace the relation instances of the flow instance with the actual registered relation instances
                relation_instances.clear();
                for (edge_key, relation_instance) in replaced_relation_instances.iter() {
                    relation_instances.insert(edge_key.clone(), relation_instance.clone());
                }
            } // Drop rwlock
            self.register_flow_instance(reactive_flow_instance);
        }
    }

    fn register_flow_instance(&self, reactive_flow_instance: Arc<ReactiveFlowInstance>) {
        if !self.reactive_entity_instance_manager.has(reactive_flow_instance.id) {
            if let Some(wrapper_entity_instance) = reactive_flow_instance.get_entity(reactive_flow_instance.id) {
                if let Err(e) = self.reactive_entity_instance_manager.register_reactive_instance(wrapper_entity_instance) {
                    error!("Failed to register wrapper entity instance of flow {}: {:?}", reactive_flow_instance.id, e);
                }
            }
        }
        self.reactive_flow_instances
            .0
            .write()
            .unwrap()
            .insert(reactive_flow_instance.id, reactive_flow_instance.clone());
        // Register label
        if let Some(value) = reactive_flow_instance.get(LABEL.property_name()) {
            if let Some(label) = value.as_str() {
                let mut writer = self.label_path_tree.0.write().unwrap();
                writer.insert(label, reactive_flow_instance.id);
            }
        }
        self.event_manager.emit_event(SystemEvent::FlowInstanceCreated(reactive_flow_instance.id))
    }

    // TODO: how to detect if the flow instance has removed an entity? => remove behaviour
    // TODO: how to detect if the flow instance has removed an relation? => remove behaviour
    fn commit(&self, id: Uuid) {
        if let Some(reactive_flow_instance) = self.get(id) {
            // Unregister removed relations
            for edge_key in reactive_flow_instance.relations_removed.read().unwrap().iter() {
                self.reactive_relation_instance_manager.unregister_reactive_instance(edge_key);
            }
            reactive_flow_instance.relations_removed.write().unwrap().clear();

            // Unregister removed entities
            for id in reactive_flow_instance.entities_removed.read().unwrap().iter() {
                self.reactive_entity_instance_manager.unregister_reactive_instance(*id);
            }
            reactive_flow_instance.entities_removed.write().unwrap().clear();

            // Register added entities
            for id in reactive_flow_instance.entities_added.read().unwrap().iter() {
                if let Some(entity_instance) = reactive_flow_instance.get_entity(*id) {
                    // TODO: How to handle reactive if registering an entity instance wasn't successful?
                    let _ = self.reactive_entity_instance_manager.register_reactive_instance(entity_instance.clone());
                }
            }
            reactive_flow_instance.entities_added.write().unwrap().clear();

            // Register added relations
            for edge_key in reactive_flow_instance.relations_added.read().unwrap().iter() {
                if let Some(relation_instance) = reactive_flow_instance.get_relation(edge_key) {
                    // TODO: How to handle reactive if registering a relation instance wasn't successful?
                    let _ = self.reactive_relation_instance_manager.register_reactive_instance(relation_instance.clone());
                }
            }
            reactive_flow_instance.relations_added.write().unwrap().clear();

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

            if let Ok(flow_instance) = FlowInstance::try_from(reactive_flow_instance) {
                self.flow_instance_manager.commit(flow_instance);
            }
        }
    }

    fn delete(&self, id: Uuid) {
        if self.has(id) {
            let reactive_flow_instance = self.get(id).unwrap();
            for (_, entity_instance) in reactive_flow_instance.entity_instances.read().unwrap().iter() {
                self.reactive_entity_instance_manager.unregister_reactive_instance(entity_instance.id);
            }
            for (_, relation_instance) in reactive_flow_instance.relation_instances.read().unwrap().iter() {
                self.reactive_relation_instance_manager
                    .unregister_reactive_instance(&relation_instance.get_key());
            }
            self.reactive_flow_instances.0.write().unwrap().remove(&id);
            // TODO: remove label
            self.event_manager.emit_event(SystemEvent::FlowInstanceDeleted(id))
        }
    }

    fn import(&self, path: &str) -> Result<Arc<ReactiveFlowInstance>, ReactiveFlowInstanceImportError> {
        if let Ok(flow_instance) = self.flow_instance_manager.import(path) {
            if let Ok(reactive_flow_instance) = self.create(flow_instance) {
                return Ok(reactive_flow_instance);
            }
        }
        Err(ReactiveFlowInstanceImportError)
    }

    fn export(&self, id: Uuid, path: &str) {
        if self.has(id) {
            self.commit(id);
            if let Ok(flow_instance) = FlowInstance::try_from(self.get(id).unwrap()) {
                self.flow_instance_manager.export(flow_instance, path);
            }
        }
    }

    fn add_provider(&self, id: Uuid, provider: Arc<dyn FlowInstanceProvider>) {
        self.flow_instance_providers.0.insert(id, provider);
    }

    fn remove_provider(&self, id: &Uuid) {
        self.flow_instance_providers.0.remove(id);
    }
}

#[async_trait]
impl Lifecycle for ReactiveFlowInstanceManagerImpl {
    async fn init(&self) {
        debug!("Importing provided flow instances");
        for flow_instance_provider in self.flow_instance_providers.0.iter() {
            for flow_instance in flow_instance_provider.get_flow_instances() {
                debug!("Creating provided flow instance {}", flow_instance.id);
                let reactive_flow_instance = self.create(flow_instance.clone());
                match reactive_flow_instance {
                    Ok(reactive_flow_instance) => {
                        let created_flow_instance: Result<FlowInstance, _> = reactive_flow_instance.try_into();
                        match created_flow_instance {
                            Ok(created_flow_instance) => {
                                let json = serde_json::to_string_pretty(&created_flow_instance).unwrap();
                                debug!("Successfully created reactive flow instance:\r\n{}", json);
                            }
                            Err(err) => {
                                debug!("Successfully created reactive flow instance {}, but failed to serialize: {:?}", flow_instance.id, err);
                            }
                        }
                    }
                    Err(err) => {
                        error!("Failed to create provided flow instance {}: {}", flow_instance.id, err);
                    }
                }
            }
        }
    }

    async fn shutdown(&self) {
        // self.reactive_flow_instances.0.write().unwrap().clear();
        // self.flow_instance_providers.0.write().unwrap().clear();
    }
}
