use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use wildmatch::WildMatch;

use crate::api::ComponentCreationError;
use crate::api::ComponentExtensionError;
use crate::api::ComponentImportError;
use crate::api::ComponentManager;
use crate::api::ComponentPropertyError;
use crate::api::ComponentRegistrationError;
use crate::api::Lifecycle;
use crate::api::SystemEventManager;
use crate::builder::ComponentBuilder;
use crate::di::{component, provides, wrapper, Component, Wrc};
use crate::model::DataType;
use crate::model::Extension;
use crate::model::PropertyType;
use crate::plugins::ComponentProvider;
use crate::plugins::SystemEvent;

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
        let _ = self.register(
            ComponentBuilder::new("core", "labeled")
                .description("The label is an hierarchical path with static segments, named parameters and catch-all parameters.")
                .property("label", DataType::String)
                .build(),
        );
        let _ = self.register(
            ComponentBuilder::new("core", "event")
                .description("This components spawns events.")
                .output_property("event", DataType::Any)
                .build(),
        );
    }
}

#[async_trait]
#[provides]
impl ComponentManager for ComponentManagerImpl {
    fn register(&self, component: crate::model::Component) -> Result<crate::model::Component, ComponentRegistrationError> {
        if self.has_fully_qualified(&component.namespace, &component.name) {
            return Err(ComponentRegistrationError::ComponentAlreadyExists(component.namespace, component.name));
        }
        self.components.0.write().unwrap().push(component.clone());
        debug!("Registered component {}", component.fully_qualified_name());
        self.event_manager.emit_event(SystemEvent::ComponentCreated(component.name.clone()));
        Ok(component)
    }

    // Returns a copy
    fn get_components(&self) -> Vec<crate::model::Component> {
        self.components.0.read().unwrap().to_vec()
    }

    fn get_components_by_namespace(&self, namespace: &str) -> Vec<crate::model::Component> {
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|component| component.namespace == namespace)
            .cloned()
            .collect()
    }

    fn has(&self, name: &str) -> bool {
        self.components.0.read().unwrap().iter().any(|component| component.name == name)
    }

    fn has_fully_qualified(&self, namespace: &str, name: &str) -> bool {
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .any(|component| component.namespace == namespace && component.name == name)
    }

    fn get(&self, name: &str) -> Option<crate::model::Component> {
        self.components.0.read().unwrap().iter().find(|component| component.name == name).cloned()
    }

    fn get_fully_qualified(&self, namespace: &str, name: &str) -> Option<crate::model::Component> {
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .find(|component| component.namespace == namespace && component.name == name)
            .cloned()
    }

    fn find(&self, search: &str) -> Vec<crate::model::Component> {
        let matcher = WildMatch::new(search);
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|component| matcher.matches(component.name.as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.components.0.read().unwrap().len()
    }

    fn create(
        &self,
        namespace: &str,
        name: &str,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<crate::model::Component, ComponentCreationError> {
        self.register(crate::model::Component::new(namespace, name, description, properties.to_vec(), extensions.to_vec()))
            .map_err(ComponentCreationError::RegistrationError)
    }

    fn replace(&self, name: &str, r_component: crate::model::Component) {
        let mut guard = self.components.0.write().unwrap();
        for mut component in guard.iter_mut() {
            if component.name == name {
                component.description = r_component.description.clone();
                component.properties = r_component.properties.clone();
                component.extensions = r_component.extensions.clone();
                // TODO: Notify about changed component -> This effects reactive instances which contains the component -> Add/remove property instances
            }
        }
    }

    fn add_property(&self, name: &str, property: PropertyType) -> Result<(), ComponentPropertyError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if component.name == name {
                if component.has_property(property.name.clone()) {
                    return Err(ComponentPropertyError::PropertyAlreadyExists);
                }
                component.properties.push(property.clone());
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(name.to_string()));
            }
        }
        Ok(())
    }

    fn remove_property(&self, name: &str, property_name: &str) {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if component.name == name {
                component.properties.retain(|property| property.name != property_name);
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(name.to_string()));
            }
        }
    }

    fn add_extension(&self, name: &str, extension: Extension) -> Result<(), ComponentExtensionError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if component.name == name {
                if component.has_extension(extension.name.clone()) {
                    return Err(ComponentExtensionError::ExtensionAlreadyExists);
                }
                component.extensions.push(extension.clone());
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(name.to_string()));
            }
        }
        Ok(())
    }

    fn remove_extension(&self, name: &str, extension_name: &str) {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if component.name == name {
                component.extensions.retain(|extension| extension.name != extension_name);
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(name.to_string()));
            }
        }
    }

    fn delete(&self, name: &str) {
        let event = SystemEvent::ComponentDeleted(name.to_string());
        self.components.0.write().unwrap().retain(|component| component.name != name);
        self.event_manager.emit_event(event);
    }

    fn import(&self, path: &str) -> Result<crate::model::Component, ComponentImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let component: crate::model::Component = serde_json::from_reader(reader)?;
        self.register(component).map_err(ComponentImportError::RegistrationError)
    }

    fn export(&self, name: &str, path: &str) {
        if let Some(component) = self.get(name) {
            match File::create(path) {
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
            let _ = self.register(component);
        }
    }

    fn get_component_categories(&self) -> Vec<String> {
        self.get_components()
            .iter()
            .filter_map(|component| {
                component
                    .extensions
                    .iter()
                    .find(|extension| extension.name == *"component_category")
                    .map(|extension| extension.name.clone())
            })
            .collect()
    }
}

impl Lifecycle for ComponentManagerImpl {
    fn init(&self) {
        self.create_base_components();
    }

    fn shutdown(&self) {
        // TODO: remove?
        self.components.0.write().unwrap().clear();
    }
}
