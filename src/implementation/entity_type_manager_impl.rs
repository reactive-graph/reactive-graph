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
use crate::api::EntityTypeComponentError;
use crate::api::EntityTypeExtensionError;
use crate::api::EntityTypeImportError;
use crate::api::EntityTypeManager;
use crate::api::EntityTypePropertyError;
use crate::api::Lifecycle;
use crate::api::SystemEventManager;
use crate::builder::EntityTypeBuilder;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::fully_qualified_identifier;
use crate::model::EntityType;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::model::TypeContainer;
use crate::model::NAMESPACE_ENTITY_TYPE;
use crate::plugins::EntityTypeProvider;
use crate::plugins::SystemEvent;

#[wrapper]
pub struct EntityTypesStorage(RwLock<Vec<EntityType>>);

#[provides]
fn create_entity_types_storage() -> EntityTypesStorage {
    EntityTypesStorage(RwLock::new(Vec::new()))
}

#[component]
pub struct EntityTypeManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    component_manager: Wrc<dyn ComponentManager>,

    entity_types: EntityTypesStorage,
}

impl EntityTypeManagerImpl {
    pub(crate) fn create_base_entity_types(&self) {
        self.register(
            EntityTypeBuilder::new("core", "system_event")
                .description("Events of the type system")
                .component("labeled")
                .component("event")
                .build(),
        );
        self.register(
            EntityTypeBuilder::new("flow", "generic_flow")
                .description("Generic flow without inputs and outputs")
                .component("labeled")
                .build(),
        );
    }
}

#[async_trait]
#[provides]
impl EntityTypeManager for EntityTypeManagerImpl {
    fn register(&self, mut entity_type: EntityType) -> EntityType {
        let type_name = entity_type.name.clone();
        // Construct the type
        entity_type.t = fully_qualified_identifier(&entity_type.namespace, &type_name, &NAMESPACE_ENTITY_TYPE);
        for component_name in entity_type.components.iter() {
            match self.component_manager.get(component_name) {
                Some(component) => {
                    // TODO: what if multiple components have the same property?
                    entity_type.properties.append(&mut component.clone().properties)
                }
                None => warn!("Entity type {} not fully initialized: No component named {}", &type_name, &component_name),
            }
        }
        self.entity_types.0.write().unwrap().push(entity_type.clone());
        debug!("Registered entity type {}", &type_name);
        self.event_manager.emit_event(SystemEvent::EntityTypeCreated(type_name));
        entity_type
    }

    fn get_entity_types(&self) -> Vec<EntityType> {
        self.entity_types.0.read().unwrap().to_vec()
    }

    fn get_entity_types_by_namespace(&self, namespace: &str) -> Vec<EntityType> {
        self.entity_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|entity_type| entity_type.namespace == namespace)
            .cloned()
            .collect()
    }

    fn has(&self, name: &str) -> bool {
        self.entity_types.0.read().unwrap().iter().any(|entity_type| entity_type.name == name)
    }

    fn get(&self, name: &str) -> Option<EntityType> {
        self.entity_types.0.read().unwrap().iter().find(|entity_type| entity_type.name == name).cloned()
    }

    fn find(&self, search: &str) -> Vec<EntityType> {
        let matcher = WildMatch::new(search);
        self.entity_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|entity_type| matcher.matches(entity_type.name.as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.entity_types.0.read().unwrap().len()
    }

    fn create(&self, namespace: &str, name: &str, description: &str, components: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>) {
        self.register(EntityType::new(namespace, name, description, components.to_vec(), properties.to_vec(), extensions.to_vec()));
    }

    fn add_component(&self, name: &str, component_name: &str) -> Result<(), EntityTypeComponentError> {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if entity_type.name == name {
                if entity_type.is_a(component_name) {
                    return Err(EntityTypeComponentError::ComponentAlreadyAssigned);
                }
                match self.component_manager.get(component_name) {
                    Some(component) => {
                        entity_type.components.push(component_name.to_string());
                        // TODO: what if multiple components have the same property?
                        entity_type.properties.append(&mut component.clone().properties)
                    }
                    None => {
                        return Err(EntityTypeComponentError::ComponentDoesNotExist);
                    }
                }
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeComponentAdded(name.to_string(), component_name.to_string()));
            }
        }
        Ok(())
    }

    fn remove_component(&self, name: &str, component_name: &str) {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if entity_type.name == name {
                entity_type.components.retain(|c_name| c_name != component_name);
                // TODO: what if multiple components have the same property?
                if let Some(component) = self.component_manager.get(component_name) {
                    let properties_to_remove: Vec<String> = component.properties.iter().map(|property| property.name.clone()).collect();
                    entity_type.properties.retain(|property| !properties_to_remove.contains(&property.name));
                }
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeComponentRemoved(name.to_string(), component_name.to_string()));
            }
        }
    }

    fn add_property(&self, name: &str, property: PropertyType) -> Result<(), EntityTypePropertyError> {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if entity_type.name == name {
                if entity_type.has_own_property(property.name.clone()) {
                    return Err(EntityTypePropertyError::PropertyAlreadyExists);
                }
                entity_type.properties.push(property.clone());
                self.event_manager
                    .emit_event(SystemEvent::EntityTypePropertyAdded(name.to_string(), property.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_property(&self, name: &str, property_name: &str) {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if entity_type.name == name {
                entity_type.properties.retain(|property| property.name != property_name);
                self.event_manager
                    .emit_event(SystemEvent::EntityTypePropertyRemoved(name.to_string(), property_name.to_string()));
            }
        }
    }

    fn add_extension(&self, name: &str, extension: Extension) -> Result<(), EntityTypeExtensionError> {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if entity_type.name == name {
                if entity_type.has_own_extension(extension.name.clone()) {
                    return Err(EntityTypeExtensionError::ExtensionAlreadyExists);
                }
                entity_type.extensions.push(extension.clone());
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeExtensionAdded(name.to_string(), extension.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_extension(&self, name: &str, extension_name: &str) {
        let mut guard = self.entity_types.0.write().unwrap();
        for entity_type in guard.iter_mut() {
            if entity_type.name == name {
                entity_type.extensions.retain(|extension| extension.name != extension_name);
                self.event_manager
                    .emit_event(SystemEvent::EntityTypeExtensionRemoved(name.to_string(), extension_name.to_string()));
            }
        }
    }

    /// TODO: first delete the entity instance of this type, then delete the entity type itself.
    fn delete(&self, name: &str) {
        let event = SystemEvent::EntityTypeDeleted(name.to_string());
        self.entity_types.0.write().unwrap().retain(|entity_type| entity_type.name != name);
        self.event_manager.emit_event(event);
    }

    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let entity_type: EntityType = serde_json::from_reader(reader)?;
        self.register(entity_type.clone());
        Ok(entity_type)
    }

    fn export(&self, name: &str, path: &str) {
        if let Some(entity_type) = self.get(name) {
            match File::create(path) {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &entity_type);
                    if result.is_err() {
                        error!("Failed to export entity type {} to {}: {}", name, path, result.err().unwrap());
                    }
                }
                Err(error) => error!("Failed to export entity type {} to {}: {}", name, path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, entity_type_provider: Arc<dyn EntityTypeProvider>) {
        for entity_type in entity_type_provider.get_entity_types() {
            self.register(entity_type);
        }
    }

    fn get_entity_type_categories(&self) -> Vec<String> {
        self.get_entity_types()
            .iter()
            .filter_map(|entity_type| {
                entity_type
                    .extensions
                    .iter()
                    .find(|extension| extension.name == *"entity_type_category")
                    .map(|extension| extension.name.clone())
            })
            .collect()
    }
}

impl Lifecycle for EntityTypeManagerImpl {
    fn init(&self) {
        self.create_base_entity_types();
    }

    fn shutdown(&self) {
        // TODO: remove?
        self.entity_types.0.write().unwrap().clear();
    }
}
