use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::warn;
use wildmatch::WildMatch;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::RelationTypeComponentError;
use crate::api::RelationTypeExtensionError;
use crate::api::RelationTypeImportError;
use crate::api::RelationTypeManager;
use crate::api::RelationTypePropertyError;
use crate::api::SystemEvent;
use crate::api::SystemEventManager;
use crate::di::*;
use crate::model::fully_qualified_identifier;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::model::TypeContainer;
use crate::model::NAMESPACE_RELATION_TYPE;
use crate::plugins::RelationTypeProvider;

#[wrapper]
pub struct RelationTypes(RwLock<std::vec::Vec<RelationType>>);

#[provides]
fn create_relation_type_storage() -> RelationTypes {
    RelationTypes(RwLock::new(std::vec::Vec::new()))
}

#[component]
pub struct RelationTypeManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_types: RelationTypes,
}

#[async_trait]
#[provides]
impl RelationTypeManager for RelationTypeManagerImpl {
    fn register(&self, mut relation_type: RelationType) {
        let type_name = relation_type.type_name.clone();
        // Construct the type
        relation_type.t = fully_qualified_identifier(&relation_type.namespace, &type_name, &NAMESPACE_RELATION_TYPE);
        if relation_type.outbound_type != "*"
            && !self.entity_type_manager.has(&relation_type.outbound_type)
            && !self.component_manager.has(&relation_type.outbound_type)
        {
            warn!(
                "Relation type {} not initialized: Outbound entity type or component does not exist {}",
                &type_name,
                relation_type.outbound_type.clone()
            );
            // TODO: Result
            return;
        }
        if relation_type.inbound_type != "*"
            && !self.entity_type_manager.has(&relation_type.inbound_type)
            && !self.component_manager.has(&relation_type.outbound_type)
        {
            warn!(
                "Relation type {} not initialized: Inbound entity type or component does not exist {}",
                &type_name,
                relation_type.inbound_type.clone()
            );
            // TODO: Result
            return;
        }
        for component_name in relation_type.components.iter() {
            match self.component_manager.get(component_name) {
                Some(component) => relation_type.properties.append(&mut component.properties.to_vec()),
                None => warn!(
                    "Relation type {} not fully initialized: No component named {}",
                    relation_type.type_name.clone(),
                    component_name
                ),
            }
        }

        let event = SystemEvent::RelationTypeCreated(type_name.clone());
        self.relation_types.0.write().unwrap().push(relation_type);
        debug!("Registered relation type {}", &type_name);
        self.event_manager.emit_event(event);
        // TODO: Result
    }

    fn get_relation_types(&self) -> Vec<RelationType> {
        self.relation_types.0.read().unwrap().to_vec()
    }

    fn get_relation_types_by_namespace(&self, namespace: &str) -> Vec<RelationType> {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|relation_type| relation_type.namespace == namespace)
            .cloned()
            .collect()
    }

    fn get_outbound_relation_types(&self, entity_type_name: &str, wildcard: bool) -> Vec<RelationType> {
        if wildcard && entity_type_name == "*" {
            return self.get_relation_types();
        }
        self.get_relation_types()
            .into_iter()
            .filter(|relation_type| (wildcard && &relation_type.outbound_type == "*") || entity_type_name.starts_with(&relation_type.outbound_type))
            .collect()
    }

    fn get_inbound_relation_types(&self, entity_type_name: &str, wildcard: bool) -> Vec<RelationType> {
        if wildcard && entity_type_name == "*" {
            return self.get_relation_types();
        }
        self.get_relation_types()
            .into_iter()
            .filter(|relation_type| (wildcard && &relation_type.inbound_type == "*") || entity_type_name.starts_with(&relation_type.inbound_type))
            .collect()
    }

    fn has(&self, type_name: &str) -> bool {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .any(|relation_type| relation_type.type_name == type_name)
    }

    fn has_starts_with(&self, type_name: &str) -> bool {
        self.get_starts_with(type_name).is_some()
    }

    fn get(&self, type_name: &str) -> Option<RelationType> {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .find(|relation_type| relation_type.type_name == type_name)
            .cloned()
    }

    fn get_starts_with(&self, type_name_starts_with: &str) -> Option<RelationType> {
        // Exact match has higher priority
        match self.get(type_name_starts_with) {
            Some(relation_type) => Some(relation_type),
            None => {
                // Fuzzy match has lower priority
                self.relation_types
                    .0
                    .read()
                    .unwrap()
                    .iter()
                    .find(|relation_type| type_name_starts_with.starts_with(relation_type.type_name.as_str()))
                    .map(|relation_type| {
                        let mut relation_type = relation_type.clone();
                        relation_type.instance_type_name = type_name_starts_with.to_string();
                        relation_type
                    })
            }
        }
    }

    fn find(&self, search: &str) -> Vec<RelationType> {
        let matcher = WildMatch::new(search);
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|relation_type| matcher.matches(relation_type.type_name.as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.relation_types.0.read().unwrap().len()
    }

    fn create(
        &self,
        namespace: &str,
        outbound_type: &str,
        type_name: &str,
        inbound_type: &str,
        description: &str,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.register(RelationType::new(
            namespace,
            outbound_type,
            type_name,
            inbound_type,
            description,
            components.to_vec(),
            properties.to_vec(),
            extensions.to_vec(),
        ));
    }

    fn add_component(&self, name: &str, component_name: &str) -> Result<(), RelationTypeComponentError> {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if relation_type.type_name == name {
                if relation_type.is_a(component_name) {
                    return Err(RelationTypeComponentError::ComponentAlreadyAssigned);
                }
                if !self.component_manager.has(component_name) {
                    return Err(RelationTypeComponentError::ComponentDoesNotExist);
                }
                relation_type.components.push(component_name.to_string());
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeComponentAdded(name.to_string(), component_name.to_string()));
            }
        }
        Ok(())
    }

    fn remove_component(&self, name: &str, component_name: &str) {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if relation_type.type_name == name {
                relation_type.components.retain(|c_name| c_name != component_name);
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeComponentRemoved(name.to_string(), component_name.to_string()));
            }
        }
    }

    fn add_property(&self, type_name: &str, property: PropertyType) -> Result<(), RelationTypePropertyError> {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if relation_type.type_name == type_name {
                if relation_type.has_own_property(property.name.clone()) {
                    return Err(RelationTypePropertyError::PropertyAlreadyExists);
                }
                relation_type.properties.push(property.clone());
                self.event_manager
                    .emit_event(SystemEvent::RelationTypePropertyAdded(type_name.to_string(), property.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_property(&self, type_name: &str, property_name: &str) {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if relation_type.type_name == type_name {
                relation_type.properties.retain(|property| property.name != property_name);
                self.event_manager
                    .emit_event(SystemEvent::RelationTypePropertyRemoved(type_name.to_string(), property_name.to_string()));
            }
        }
    }

    fn add_extension(&self, type_name: &str, extension: Extension) -> Result<(), RelationTypeExtensionError> {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if relation_type.type_name == type_name {
                if relation_type.has_own_extension(extension.name.clone()) {
                    return Err(RelationTypeExtensionError::ExtensionAlreadyExists);
                }
                relation_type.extensions.push(extension.clone());
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeExtensionAdded(type_name.to_string(), extension.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_extension(&self, type_name: &str, extension_name: &str) {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if relation_type.type_name == type_name {
                relation_type.extensions.retain(|extension| extension.name != extension_name);
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeExtensionRemoved(type_name.to_string(), extension_name.to_string()));
            }
        }
    }

    fn delete(&self, type_name: &str) {
        let event = SystemEvent::RelationTypeDeleted(type_name.to_string());
        self.relation_types
            .0
            .write()
            .unwrap()
            .retain(|relation_type| relation_type.type_name != type_name);
        self.event_manager.emit_event(event);
    }

    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_type: RelationType = serde_json::from_reader(reader)?;
        self.register(relation_type.clone());
        Ok(relation_type)
    }

    fn export(&self, type_name: &str, path: &str) {
        if let Some(relation_type) = self.get(type_name) {
            match File::create(path) {
                Ok(file) => {
                    if let Err(error) = serde_json::to_writer_pretty(&file, &relation_type) {
                        error!("Failed to export relation type {} to {}: {}", type_name, path, error);
                    }
                }
                Err(error) => error!("Failed to export relation type {} to {}: {}", type_name, path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>) {
        for relation_type in relation_type_provider.get_relation_types() {
            debug!("Registering relation type: {}", relation_type.type_name);
            self.register(relation_type);
        }
    }
}

impl Lifecycle for RelationTypeManagerImpl {
    fn shutdown(&self) {
        // TODO: remove?
        self.relation_types.0.write().unwrap().clear();
    }
}
