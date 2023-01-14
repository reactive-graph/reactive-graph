use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::trace;
use log::warn;
use serde_json::json;
use wildmatch::WildMatch;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::RelationTypeComponentError;
use crate::api::RelationTypeCreationError;
use crate::api::RelationTypeExtensionError;
use crate::api::RelationTypeImportError;
use crate::api::RelationTypeManager;
use crate::api::RelationTypePropertyError;
use crate::api::RelationTypeRegistrationError;
use crate::api::SystemEventManager;
use crate::core_model::EXTENSION_DIVERGENT;
use crate::di::*;
use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::ExtensionContainer;
use crate::model::ExtensionTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::model::TypeContainer;
use crate::model::TypeDefinitionGetter;
use crate::plugins::RelationTypeProvider;
use crate::plugins::SystemEvent;

#[wrapper]
pub struct RelationTypes(RwLock<Vec<RelationType>>);

#[provides]
fn create_relation_type_storage() -> RelationTypes {
    RelationTypes(RwLock::new(Vec::new()))
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
    fn register(&self, mut relation_type: RelationType) -> Result<RelationType, RelationTypeRegistrationError> {
        let relation_ty = relation_type.ty.clone();
        if self.has(&relation_ty) {
            return Err(RelationTypeRegistrationError::RelationTypeAlreadyExists(relation_ty));
        }
        // Check if outbound type exists
        if relation_type.outbound_type.type_name() != "*" {
            match &relation_type.outbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !self.component_manager.has(component_ty) {
                        warn!("Relation type {} not registered: Outbound component {} does not exist", &relation_ty, component_ty);
                        return Err(RelationTypeRegistrationError::OutboundComponentDoesNotExist(relation_ty, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if !self.entity_type_manager.has(entity_ty) {
                        warn!("Relation type {} not registered: Outbound entity type {} does not exist", &relation_ty, entity_ty);
                        return Err(RelationTypeRegistrationError::OutboundEntityTypeDoesNotExist(relation_ty, entity_ty.clone()));
                    }
                }
            }
        }
        // Check if inbound type exists
        if relation_type.inbound_type.type_name() != "*" {
            match &relation_type.inbound_type {
                ComponentOrEntityTypeId::Component(component_ty) => {
                    if !self.component_manager.has(component_ty) {
                        warn!("Relation type {} not registered: Inbound component {} does not exist", &relation_ty, component_ty);
                        return Err(RelationTypeRegistrationError::InboundComponentDoesNotExist(relation_ty, component_ty.clone()));
                    }
                }
                ComponentOrEntityTypeId::EntityType(entity_ty) => {
                    if !self.entity_type_manager.has(entity_ty) {
                        warn!("Relation type {} not registered: Inbound entity type {} does not exist", &relation_ty, entity_ty);
                        return Err(RelationTypeRegistrationError::InboundEntityTypeDoesNotExist(relation_ty, entity_ty.clone()));
                    }
                }
            }
        }
        // Apply components
        let mut divergent = Vec::new();
        for component_ty in relation_type.components.iter() {
            let mut is_divergent = false;
            match self.component_manager.get(component_ty) {
                Some(component) => {
                    // TODO: what if multiple components have the same property?
                    for property_type in component.properties {
                        // Own property wins
                        if !relation_type.has_own_property(&property_type.name) {
                            relation_type.properties.push(property_type.clone());
                        } else {
                            // Check for divergent data type
                            if let Some(relation_type_property_type) = relation_type.get_own_property(&property_type.name) {
                                if property_type.data_type != relation_type_property_type.data_type {
                                    is_divergent = true;
                                    warn!(
                                        "{}__{} has divergent data type {} to {}__{} which has data type {}",
                                        &relation_type.ty,
                                        &relation_type_property_type.name,
                                        &relation_type_property_type.data_type,
                                        &component_ty,
                                        &property_type.name,
                                        &property_type.data_type
                                    );
                                }
                            }
                            // TODO: merge description (if no own description)
                            // TODO: merge extensions (for each: if own does not have the extension, add it)
                        }
                    }
                    // relation_type.properties.append(&mut component.properties.to_vec())
                }
                None => {
                    is_divergent = true;
                    warn!("Relation type {} not fully initialized: Missing component {}", &relation_ty, component_ty)
                }
            }
            if is_divergent {
                divergent.push(component_ty.to_string());
            }
        }
        relation_type
            .extensions
            .push(Extension::new(&EXTENSION_DIVERGENT.clone(), String::new(), json!(divergent)));
        self.relation_types.0.write().unwrap().push(relation_type.clone());
        debug!("Registered relation type {}", &relation_ty);
        self.event_manager.emit_event(SystemEvent::RelationTypeCreated(relation_ty));
        Ok(relation_type)
    }

    fn get_all(&self) -> Vec<RelationType> {
        self.relation_types.0.read().unwrap().to_vec()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<RelationType> {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|relation_type| relation_type.namespace() == namespace)
            .cloned()
            .collect()
    }

    fn get_by_having_component(&self, component_ty: &ComponentTypeId) -> Vec<RelationType> {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|relation_type| relation_type.components.contains(component_ty))
            .cloned()
            .collect()
    }

    fn get_outbound_relation_types(&self, outbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> Vec<RelationType> {
        if wildcard && outbound_ty.type_name() == "*" {
            return self.get_all();
        }
        self.get_all()
            .into_iter()
            .filter(|relation_type| (wildcard && &relation_type.outbound_type.type_name() == "*") || outbound_ty == &relation_type.outbound_type)
            .collect()
    }

    fn get_inbound_relation_types(&self, inbound_ty: &ComponentOrEntityTypeId, wildcard: bool) -> Vec<RelationType> {
        if wildcard && inbound_ty.type_name() == "*" {
            return self.get_all();
        }
        self.get_all()
            .into_iter()
            .filter(|relation_type| (wildcard && &relation_type.inbound_type.type_name() == "*") || inbound_ty == &relation_type.inbound_type)
            .collect()
    }

    fn has(&self, ty: &RelationTypeId) -> bool {
        self.relation_types.0.read().unwrap().iter().any(|relation_type| &relation_type.ty == ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.has(&RelationTypeId::new_from_type(namespace, type_name))
    }

    fn get(&self, ty: &RelationTypeId) -> Option<RelationType> {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .find(|relation_type| &relation_type.ty == ty)
            .cloned()
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<RelationType> {
        self.get(&RelationTypeId::new_from_type(namespace, type_name))
    }

    fn find(&self, search: &str) -> Vec<RelationType> {
        let matcher = WildMatch::new(search);
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|relation_type| matcher.matches(relation_type.type_name().as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.relation_types.0.read().unwrap().len()
    }

    /// Returns the count of relation types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|relation_type| relation_type.namespace() == namespace)
            .count()
    }

    fn create(
        &self,
        outbound_type: &ComponentOrEntityTypeId,
        ty: &RelationTypeId,
        inbound_type: &ComponentOrEntityTypeId,
        description: &str,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<RelationType, RelationTypeCreationError> {
        self.register(RelationType::new(
            outbound_type,
            ty,
            inbound_type,
            description,
            components.to_vec(),
            properties.to_vec(),
            extensions.to_vec(),
        ))
        .map_err(RelationTypeCreationError::RegistrationError)
    }

    fn add_component(&self, ty: &RelationTypeId, component_ty: &ComponentTypeId) -> Result<(), RelationTypeComponentError> {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if &relation_type.ty == ty {
                if relation_type.is_a(component_ty) {
                    return Err(RelationTypeComponentError::ComponentAlreadyAssigned);
                }
                if !self.component_manager.has(component_ty) {
                    return Err(RelationTypeComponentError::ComponentDoesNotExist);
                }
                relation_type.components.push(component_ty.clone());
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeComponentAdded(relation_type.ty.clone(), component_ty.clone()));
            }
        }
        Ok(())
    }

    fn remove_component(&self, ty: &RelationTypeId, component_ty: &ComponentTypeId) {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if &relation_type.ty == ty {
                relation_type.components.retain(|c| c != component_ty);
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeComponentRemoved(relation_type.ty.clone(), component_ty.clone()));
            }
        }
    }

    fn add_property(&self, ty: &RelationTypeId, property: PropertyType) -> Result<(), RelationTypePropertyError> {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if &relation_type.ty == ty {
                if relation_type.has_own_property(property.name.clone()) {
                    return Err(RelationTypePropertyError::PropertyAlreadyExists);
                }
                relation_type.properties.push(property.clone());
                self.event_manager
                    .emit_event(SystemEvent::RelationTypePropertyAdded(relation_type.ty.clone(), property.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_property(&self, ty: &RelationTypeId, property_name: &str) {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if &relation_type.ty == ty {
                relation_type.properties.retain(|property| property.name != property_name);
                self.event_manager
                    .emit_event(SystemEvent::RelationTypePropertyRemoved(relation_type.ty.clone(), property_name.to_string()));
            }
        }
    }

    fn add_extension(&self, ty: &RelationTypeId, extension: Extension) -> Result<(), RelationTypeExtensionError> {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if &relation_type.ty == ty {
                let extension_ty = extension.ty.clone();
                if relation_type.has_own_extension(&extension_ty) {
                    return Err(RelationTypeExtensionError::ExtensionAlreadyExists(extension_ty));
                }
                relation_type.extensions.push(extension.clone());
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeExtensionAdded(relation_type.ty.clone(), extension_ty));
            }
        }
        Ok(())
    }

    // TODO: update extension

    fn remove_extension(&self, ty: &RelationTypeId, extension_ty: &ExtensionTypeId) {
        let mut guard = self.relation_types.0.write().unwrap();
        for relation_type in guard.iter_mut() {
            if &relation_type.ty == ty {
                relation_type.extensions.retain(|extension| &extension.ty != extension_ty);
                self.event_manager
                    .emit_event(SystemEvent::RelationTypeExtensionRemoved(relation_type.ty.clone(), extension_ty.clone()));
            }
        }
    }

    fn delete(&self, ty: &RelationTypeId) {
        self.relation_types.0.write().unwrap().retain(|relation_type| &relation_type.ty != ty);
        self.event_manager.emit_event(SystemEvent::RelationTypeDeleted(ty.clone()));
    }

    fn validate(&self, ty: &RelationTypeId) -> bool {
        if let Some(relation_type) = self.get(ty) {
            return relation_type.components.iter().all(|component_ty| self.component_manager.has(component_ty))
                && match &relation_type.outbound_type {
                    ComponentOrEntityTypeId::EntityType(entity_ty) => self.entity_type_manager.validate(entity_ty),
                    ComponentOrEntityTypeId::Component(component_ty) => self.component_manager.has(component_ty),
                }
                && match &relation_type.inbound_type {
                    ComponentOrEntityTypeId::EntityType(entity_ty) => self.entity_type_manager.validate(entity_ty),
                    ComponentOrEntityTypeId::Component(component_ty) => self.component_manager.has(component_ty),
                };
        }
        false
    }

    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_type: RelationType = serde_json::from_reader(reader)?;
        self.register(relation_type).map_err(RelationTypeImportError::RegistrationError)
    }

    fn export(&self, ty: &RelationTypeId, path: &str) {
        if let Some(relation_type) = self.get(ty) {
            match File::create(path) {
                Ok(file) => {
                    if let Err(error) = serde_json::to_writer_pretty(&file, &relation_type) {
                        error!("Failed to export relation type {} to {}: {}", ty.type_definition().to_string(), path, error);
                    }
                }
                Err(error) => error!("Failed to export relation type {} to {}: {}", ty.type_definition().to_string(), path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>) {
        for relation_type in relation_type_provider.get_relation_types() {
            trace!("Registering relation type: {}", relation_type.type_definition().to_string());
            let _ = self.register(relation_type);
        }
    }
}

impl Lifecycle for RelationTypeManagerImpl {
    fn shutdown(&self) {
        // TODO: remove?
        self.relation_types.0.write().unwrap().clear();
    }
}
