use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use crate::builder::ComponentBuilder;
use crate::di::{component, provides, wrapper, Component, Wrc};
use crate::model::DataType;
use async_trait::async_trait;
use log::{debug, error};
use wildmatch::WildMatch;

use crate::api::SystemEventManager;
use crate::api::Lifecycle;
use crate::api::{ComponentManager, SystemEvent};
use crate::model::PropertyType;
use crate::plugins::ComponentProvider;

#[wrapper]
pub struct ComponentsStorage(RwLock<Vec<crate::model::Component>>);

#[provides]
fn create_components_storage() -> ComponentsStorage {
    ComponentsStorage(RwLock::new(Vec::new()))
}

#[component]
pub struct ComponentManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    components: ComponentsStorage,
}

impl ComponentManagerImpl {
    pub(crate) fn create_base_components(&self) {
        self.register(
            ComponentBuilder::new("labeled")
                .description("The label is an hierarchical path with static segments, named parameters and catch-all parameters.")
                .property("label", DataType::String)
                .build(),
        );
        self.register(
            ComponentBuilder::new("event")
                .description("This components spawns events.")
                .output_property("event", DataType::Any)
                .build(),
        );
    }
}

#[async_trait]
#[provides]
impl ComponentManager for ComponentManagerImpl {
    fn register(&self, component: crate::model::Component) {
        if !self.has(component.name.clone()) {
            let name = component.name.clone();
            debug!("Registered component {}", name.clone());
            self.components.0.write().unwrap().push(component);
            self.event_manager.emit_event(SystemEvent::ComponentCreated(name));
        }
    }

    // Returns a copy
    fn get_components(&self) -> Vec<crate::model::Component> {
        self.components.0.read().unwrap().to_vec()
    }

    fn has(&self, name: String) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: String) -> Option<crate::model::Component> {
        self.components.0.read().unwrap().iter().find(|component| component.name == name).cloned()
    }

    fn find(&self, search: String) -> Vec<crate::model::Component> {
        let matcher = WildMatch::new(search.as_str());
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|component| matcher.matches(component.name.as_str()))
            .cloned()
            .collect()
    }

    fn create(&self, name: String, properties: Vec<PropertyType>) {
        self.register(crate::model::Component::new(name, properties.to_vec()));
    }

    fn delete(&self, name: String) {
        let event = SystemEvent::ComponentDeleted(name.clone());
        self.components.0.write().unwrap().retain(|component| component.name != name);
        self.event_manager.emit_event(event);
    }

    fn import(&self, path: String) {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Ok(component) = serde_json::from_reader(reader) {
                self.register(component);
            }
        }
    }

    fn export(&self, name: String, path: String) {
        if let Some(component) = self.get(name.clone()) {
            match File::create(path.clone()) {
                Ok(file) => {
                    if let Err(error) = serde_json::to_writer_pretty(&file, &component) {
                        error!("Failed to export component {} to {}: {}", name, path, error);
                    }
                }
                Err(error) => error!("Failed to export component {} to {}: {}", name, path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, component_provider: Arc<dyn ComponentProvider>) {
        for component in component_provider.get_components() {
            debug!("Registering component: {}", component.name);
            self.register(component);
        }
    }
}

impl Lifecycle for ComponentManagerImpl {
    fn init(&self) {
        self.create_base_components();
    }

    fn post_init(&self) {}

    fn pre_shutdown(&self) {}

    fn shutdown(&self) {
        // TODO: remove?
        self.components.0.write().unwrap().clear();
    }
}
