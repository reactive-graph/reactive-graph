use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::trace;
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
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::implementation::COMPONENT_EVENT;
use crate::implementation::COMPONENT_LABELED;
use crate::implementation::NAMESPACE_CORE;
use crate::implementation::PROPERTY_EVENT;
use crate::implementation::PROPERTY_LABEL;
use crate::model::ComponentTypeId;
use crate::model::DataType;
use crate::model::Extension;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::TypeDefinitionGetter;
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
        let labeled_ty = ComponentTypeId::new_from_type(NAMESPACE_CORE, COMPONENT_LABELED);
        let _ = self.register(
            ComponentBuilder::new(labeled_ty)
                .description("The label is an hierarchical path with static segments, named parameters and catch-all parameters.")
                .property(PROPERTY_LABEL, DataType::String)
                .build(),
        );
        let event_ty = ComponentTypeId::new_from_type(NAMESPACE_CORE, COMPONENT_EVENT);
        let _ = self.register(
            ComponentBuilder::new(event_ty)
                .description("This components spawns events.")
                .output_property(PROPERTY_EVENT, DataType::Any)
                .build(),
        );
    }
}

#[async_trait]
#[provides]
impl ComponentManager for ComponentManagerImpl {
    fn register(&self, component: crate::model::Component) -> Result<crate::model::Component, ComponentRegistrationError> {
        if self.has(&component.ty) {
            return Err(ComponentRegistrationError::ComponentAlreadyExists(component.ty.clone()));
        }
        self.components.0.write().unwrap().push(component.clone());
        debug!("Registered component {}", component.type_definition().to_string());
        self.event_manager.emit_event(SystemEvent::ComponentCreated(component.ty.clone()));
        Ok(component)
    }

    // Returns a copy
    fn get_all(&self) -> Vec<crate::model::Component> {
        self.components.0.read().unwrap().to_vec()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<crate::model::Component> {
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|component| component.namespace() == namespace)
            .cloned()
            .collect()
    }

    fn has(&self, ty: &ComponentTypeId) -> bool {
        self.components.0.read().unwrap().iter().any(|component| &component.ty == ty)
    }

    fn has_by_type(&self, namespace: &str, name: &str) -> bool {
        self.has(&ComponentTypeId::new_from_type(namespace, name))
    }

    fn get(&self, ty: &ComponentTypeId) -> Option<crate::model::Component> {
        self.components.0.read().unwrap().iter().find(|component| &component.ty == ty).cloned()
    }

    fn get_by_type(&self, namespace: &str, name: &str) -> Option<crate::model::Component> {
        self.get(&ComponentTypeId::new_from_type(namespace, name))
    }

    fn find(&self, search: &str) -> Vec<crate::model::Component> {
        let matcher = WildMatch::new(search);
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|component| matcher.matches(component.type_name().as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.components.0.read().unwrap().len()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.components
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|component| component.ty.namespace() == namespace)
            .count()
    }

    fn create(
        &self,
        ty: &ComponentTypeId,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<crate::model::Component, ComponentCreationError> {
        let component = crate::model::Component::new(ty.clone(), description, properties.to_vec(), extensions.to_vec());
        self.register(component).map_err(ComponentCreationError::RegistrationError)
    }

    fn replace(&self, ty: &ComponentTypeId, r_component: crate::model::Component) {
        let mut guard = self.components.0.write().unwrap();
        for mut component in guard.iter_mut() {
            if &component.ty == ty {
                component.description = r_component.description.clone();
                component.properties = r_component.properties.clone();
                component.extensions = r_component.extensions.clone();
                // TODO: Notify about changed component -> This effects reactive instances which contains the component -> Add/remove property instances
            }
        }
    }

    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<(), ComponentPropertyError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                if component.has_property(property.name.clone()) {
                    return Err(ComponentPropertyError::PropertyAlreadyExists);
                }
                component.properties.push(property.clone());
                // TODO: more specific system event
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(ty.clone()));
            }
        }
        Ok(())
    }

    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                component.properties.retain(|property| property.name != property_name);
                // TODO: more specific system event
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(ty.clone()));
            }
        }
    }

    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) -> Result<(), ComponentExtensionError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                if component.has_extension(extension.name.clone()) {
                    return Err(ComponentExtensionError::ExtensionAlreadyExists);
                }
                component.extensions.push(extension.clone());
                // TODO: more specific system event
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(ty.clone()));
            }
        }
        Ok(())
    }

    fn remove_extension(&self, ty: &ComponentTypeId, extension_name: &str) {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                component.extensions.retain(|extension| extension.name != extension_name);
                // TODO: more specific system event
                self.event_manager.emit_event(SystemEvent::ComponentUpdated(ty.clone()));
            }
        }
    }

    fn delete(&self, ty: &ComponentTypeId) {
        self.components.0.write().unwrap().retain(|component| &component.ty != ty);
        self.event_manager.emit_event(SystemEvent::ComponentDeleted(ty.clone()));
    }

    fn import(&self, path: &str) -> Result<crate::model::Component, ComponentImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let component: crate::model::Component = serde_json::from_reader(reader)?;
        self.register(component).map_err(ComponentImportError::RegistrationError)
    }

    fn export(&self, ty: &ComponentTypeId, path: &str) {
        if let Some(component) = self.get(ty) {
            match File::create(path) {
                Ok(file) => {
                    if let Err(error) = serde_json::to_writer_pretty(&file, &component) {
                        error!("Failed to export component {} to {}: {}", component.type_definition().to_string(), path, error);
                    }
                }
                Err(error) => error!("Failed to export component {} to {}: {}", component.type_definition().to_string(), path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, component_provider: Arc<dyn ComponentProvider>) {
        for component in component_provider.get_components() {
            trace!("Registering component: {}", component.type_definition().to_string());
            let _ = self.register(component);
        }
    }

    fn get_component_categories(&self) -> Vec<String> {
        self.get_all()
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