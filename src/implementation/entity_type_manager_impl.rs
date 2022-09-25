use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use crate::builder::EntityTypeBuilder;
use crate::di::{component, provides, wrapper, Component, Wrc};
use async_trait::async_trait;
use indradb::Identifier;
use log::debug;
use log::error;
use log::warn;
use wildmatch::WildMatch;

use crate::api::ComponentManager;
use crate::api::EntityTypeImportError;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::SystemEvent;
use crate::api::SystemEventManager;
use crate::model::EntityType;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::plugins::EntityTypeProvider;

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
            EntityTypeBuilder::new("generic_flow")
                .namespace("flow")
                .description("Generic flow without inputs and outputs")
                .component("labeled")
                .build(),
        );
        self.register(
            EntityTypeBuilder::new("system_event")
                .namespace("events")
                .description("Events of the type system")
                .component("labeled")
                .component("event")
                .build(),
        );
    }
}

#[async_trait]
#[provides]
impl EntityTypeManager for EntityTypeManagerImpl {
    fn register(&self, mut entity_type: EntityType) -> EntityType {
        // Construct the type
        entity_type.t = Identifier::new(entity_type.name.clone()).unwrap();
        for component_name in entity_type.components.iter() {
            match self.component_manager.get(&component_name) {
                Some(component) => entity_type.properties.append(&mut component.clone().properties),
                None => warn!("Entity type {} not fully initialized: No component named {}", entity_type.name.clone(), component_name),
            }
        }
        self.entity_types.0.write().unwrap().push(entity_type.clone());
        debug!("Registered entity type {}", entity_type.name);
        self.event_manager.emit_event(SystemEvent::EntityTypeCreated(entity_type.name.clone()));
        entity_type
    }

    fn get_entity_types(&self) -> Vec<EntityType> {
        self.entity_types.0.read().unwrap().to_vec()
    }

    fn has(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: &str) -> Option<EntityType> {
        self.entity_types
            .0
            .read()
            .unwrap()
            .iter()
            .find(|entity_type| &entity_type.name == name)
            .cloned()
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

    fn create(&self, name: String, namespace: String, components: Vec<String>, properties: Vec<PropertyType>, extensions: Vec<Extension>) {
        self.register(EntityType::new(name, namespace, String::new(), components.to_vec(), properties.to_vec(), extensions.to_vec()));
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
        if let Some(entity_type) = self.get(&name) {
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
                    .find(|extension| extension.name == "entity_type_category".to_string())
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
