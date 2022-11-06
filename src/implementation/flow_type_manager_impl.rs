use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::warn;
use uuid::Uuid;
use wildmatch::WildMatch;

use crate::api::flow_type_manager::FlowTypeImportError;
use crate::api::flow_type_manager::FlowTypeManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::RelationTypeManager;
use crate::api::SystemEventManager;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::RelationInstance;
use crate::model::TypeDefinitionGetter;
use crate::plugins::FlowTypeProvider;
use crate::plugins::SystemEvent;

#[wrapper]
pub struct FlowTypesStorage(RwLock<Vec<FlowType>>);

#[provides]
fn create_flow_types_storage() -> FlowTypesStorage {
    FlowTypesStorage(RwLock::new(Vec::new()))
}

#[component]
pub struct FlowTypeManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    flow_types: FlowTypesStorage,
}

impl FlowTypeManagerImpl {}

#[async_trait]
#[provides]
impl FlowTypeManager for FlowTypeManagerImpl {
    fn register(&self, flow_type: FlowType) -> FlowType {
        // Check that the entity types of every declared entity instance exists
        for entity_ty in flow_type.uses_entity_types() {
            if !self.entity_type_manager.has(&entity_ty) {
                warn!(
                    "Flow type {} not fully initialized: No entity type named {}",
                    flow_type.type_definition().to_string(),
                    entity_ty.type_definition().to_string()
                );
            }
        }
        // Check that the relation type of every declared relation instance exists
        for relation_ty in flow_type.uses_relation_types() {
            if !self.relation_type_manager.has(&relation_ty) {
                warn!(
                    "Flow type {} not fully initialized: No relation type named {}",
                    flow_type.type_definition().to_string(),
                    relation_ty.type_definition().to_string()
                );
            }
        }
        // TODO: Check that entity instances referenced by a relation instance exists
        // TODO: Check that relation instances outbound entity has correct entity_type
        // TODO: Check that relation instances inbound entity has correct entity_type
        self.flow_types.0.write().unwrap().push(flow_type.clone());
        debug!("Registered flow type {}", &flow_type.ty);
        self.event_manager.emit_event(SystemEvent::FlowTypeCreated(flow_type.ty.clone()));
        flow_type
    }

    fn get_all(&self) -> Vec<FlowType> {
        self.flow_types.0.read().unwrap().to_vec()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<FlowType> {
        self.flow_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|flow_type| flow_type.namespace() == namespace)
            .cloned()
            .collect()
    }

    fn has(&self, ty: &FlowTypeId) -> bool {
        self.flow_types.0.read().unwrap().iter().any(|flow_type| &flow_type.ty == ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.has(&FlowTypeId::new_from_type(namespace, type_name))
    }

    fn get(&self, ty: &FlowTypeId) -> Option<FlowType> {
        self.flow_types.0.read().unwrap().iter().find(|flow_type| &flow_type.ty == ty).cloned()
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<FlowType> {
        self.get(&FlowTypeId::new_from_type(namespace, type_name))
    }

    fn find(&self, search: &str) -> Vec<FlowType> {
        let matcher = WildMatch::new(search);
        self.flow_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|flow_type| matcher.matches(flow_type.type_name().as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.flow_types.0.read().unwrap().len()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.flow_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|flow_type| flow_type.namespace() == namespace)
            .count()
    }

    fn create(
        &self,
        ty: &FlowTypeId,
        description: &str,
        wrapper_entity_instance: EntityInstance,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.register(FlowType::new(
            ty.clone(),
            description,
            wrapper_entity_instance,
            entity_instances.to_vec(),
            relation_instances.to_vec(),
            variables.to_vec(),
            extensions.to_vec(),
        ));
    }

    fn add_entity_instance(&self, ty: &FlowTypeId, entity_instance: EntityInstance) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.entity_instances.push(entity_instance);
        }
    }

    fn update_entity_instance(&self, ty: &FlowTypeId, id: Uuid, entity_instance: EntityInstance) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.entity_instances.retain(|e| e.id != id);
            flow_type.entity_instances.push(entity_instance);
        }
    }

    fn remove_entity_instance(&self, ty: &FlowTypeId, id: Uuid) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.entity_instances.retain(|e| e.id != id);
        }
    }

    fn add_extension(&self, ty: &FlowTypeId, extension: Extension) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.extensions.push(extension);
        }
    }

    fn update_extension(&self, ty: &FlowTypeId, extension_name: &str, extension: Extension) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.extensions.retain(|extension| extension.name == extension_name);
            flow_type.extensions.push(extension);
        }
    }

    fn remove_extension(&self, ty: &FlowTypeId, extension_name: &str) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.extensions.retain(|extension| extension.name == extension_name);
        }
    }

    fn add_variable(&self, ty: &FlowTypeId, variable: PropertyType) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.variables.push(variable);
        }
    }

    fn update_variable(&self, ty: &FlowTypeId, variable_name: &str, variable: PropertyType) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.variables.retain(|variable| variable.name == variable_name);
            flow_type.variables.push(variable);
        }
    }

    fn remove_variable(&self, ty: &FlowTypeId, variable_name: &str) {
        let mut guard = self.flow_types.0.write().unwrap();
        if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
            flow_type.variables.retain(|variable| variable.name == variable_name);
        }
    }

    fn delete(&self, ty: &FlowTypeId) {
        self.flow_types.0.write().unwrap().retain(|flow_type| &flow_type.ty != ty);
        self.event_manager.emit_event(SystemEvent::FlowTypeDeleted(ty.clone()));
    }

    fn validate(&self, ty: &FlowTypeId) -> bool {
        if let Some(flow_type) = self.get(ty) {
            return flow_type
                .entity_instances
                .iter()
                .all(|entity_instance| self.entity_type_manager.validate(&entity_instance.ty))
                && flow_type
                    .relation_instances
                    .iter()
                    .all(|relation_instance| self.relation_type_manager.validate(&relation_instance.relation_type_id()));
        }
        false
    }

    fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let flow_type: FlowType = serde_json::from_reader(reader)?;
        self.register(flow_type.clone());
        Ok(flow_type)
    }

    fn export(&self, ty: &FlowTypeId, path: &str) {
        if let Some(flow_type) = self.get(ty) {
            match File::create(path) {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &flow_type);
                    if result.is_err() {
                        error!("Failed to export flow type {} to {}: {}", ty.type_definition().to_string(), path, result.err().unwrap());
                    }
                }
                Err(error) => error!("Failed to export flow type {} to {}: {}", ty.type_definition().to_string(), path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, flow_type_provider: Arc<dyn FlowTypeProvider>) {
        for flow_type in flow_type_provider.get_flow_types() {
            self.register(flow_type);
        }
    }
}

impl Lifecycle for FlowTypeManagerImpl {
    fn shutdown(&self) {
        // TODO: remove?
        self.flow_types.0.write().unwrap().clear();
    }
}
