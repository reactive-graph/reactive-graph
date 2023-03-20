use std::collections::HashSet;
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
use crate::api::ComponentExtensionUpdateError;
use crate::api::ComponentImportError;
use crate::api::ComponentManager;
use crate::api::ComponentMergeError;
use crate::api::ComponentPropertyError;
use crate::api::ComponentPropertyUpdateError;
use crate::api::ComponentRegistrationError;
use crate::api::Lifecycle;
use crate::api::SystemEventManager;
use crate::builder::ComponentBuilder;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ComponentTypeId;
use crate::model::DataType;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::PropertyType;
use crate::model::PropertyTypeDefinition;
use crate::model::TypeDefinitionGetter;
use crate::model_runtime::EventProperties::EVENT;
use crate::model_runtime::LabeledProperties::LABEL;
use crate::model_runtime::COMPONENT_EVENT;
use crate::model_runtime::COMPONENT_LABELED;
use crate::model_runtime::EXTENSION_COMPONENT_CATEGORY;
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
            ComponentBuilder::new(COMPONENT_LABELED.clone())
                .description("The label is an hierarchical path with static segments, named parameters and catch-all parameters.")
                .property(LABEL.property_name(), DataType::String)
                .build(),
        );
        let _ = self.register(
            ComponentBuilder::new(COMPONENT_EVENT.clone())
                .description("This components spawns events.")
                .output_property(&EVENT.property_name(), DataType::Any)
                .build(),
        );
    }
}

#[async_trait]
#[provides]
impl ComponentManager for ComponentManagerImpl {
    fn register(&self, component: crate::model::Component) -> Result<crate::model::Component, ComponentRegistrationError> {
        if self.has(&component.ty) {
            return Err(ComponentRegistrationError::ComponentAlreadyExists(component.ty));
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

    fn get_namespaces(&self) -> HashSet<String> {
        self.components.0.read().unwrap().iter().map(|component| component.ty.namespace()).collect()
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
                component.ty = r_component.ty.clone();
                component.description = r_component.description.clone();
                component.properties = r_component.properties.clone();
                component.extensions = r_component.extensions.clone();
                // TODO: Notify about changed component -> This effects reactive instances which contains the component -> Add/remove property instances
            }
        }
    }

    fn merge(&self, component_to_merge: crate::model::Component) -> Result<crate::model::Component, ComponentMergeError> {
        let ty = component_to_merge.ty;
        if !self.has(&ty) {
            return Err(ComponentMergeError::ComponentDoesNotExist(ty));
        }
        let mut guard = self.components.0.write().unwrap();
        for mut component in guard.iter_mut() {
            if component.ty == ty {
                component.description = component_to_merge.description.clone();
                for property_to_merge in component_to_merge.properties.into_iter() {
                    if !component.has_property(&property_to_merge.name) {
                        component.properties.push(property_to_merge);
                    } else {
                        for existing_property in component.properties.iter_mut() {
                            if existing_property.name == property_to_merge.name {
                                existing_property.description = property_to_merge.description.clone();
                                existing_property.data_type = property_to_merge.data_type;
                                existing_property.socket_type = property_to_merge.socket_type;
                                existing_property.mutability = property_to_merge.mutability;
                                for property_extension_to_merge in property_to_merge.extensions.iter() {
                                    if !existing_property.has_extension(&property_extension_to_merge.ty) {
                                        existing_property.extensions.push(property_extension_to_merge.clone());
                                    } else {
                                        for existing_property_extension in existing_property.extensions.iter_mut() {
                                            if existing_property_extension.ty == property_extension_to_merge.ty {
                                                existing_property_extension.description = property_extension_to_merge.description.clone();
                                                existing_property_extension.extension = property_extension_to_merge.extension.clone();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                for extension_to_merge in component_to_merge.extensions.into_iter() {
                    if !component.has_extension(&extension_to_merge.ty) {
                        component.extensions.push(extension_to_merge);
                    } else {
                        for existing_extension in component.extensions.iter_mut() {
                            if existing_extension.ty == extension_to_merge.ty {
                                existing_extension.description = extension_to_merge.description.clone();
                                existing_extension.extension = extension_to_merge.extension.clone();
                            }
                        }
                    }
                }
                // TODO: Notify about changed component -> This effects reactive instances which contains the component -> Add/remove property instances
                return Ok(component.clone());
            }
        }
        Err(ComponentMergeError::ComponentDoesNotExist(ty))
    }

    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<(), ComponentPropertyError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                if component.has_property(property.name.clone()) {
                    return Err(ComponentPropertyError::PropertyAlreadyExists);
                }
                component.properties.push(property.clone());
                self.event_manager
                    .emit_event(SystemEvent::ComponentPropertyAdded(ty.clone(), property.name.clone()));
            }
        }
        Ok(())
    }

    fn update_property(&self, ty: &ComponentTypeId, property_name: &str, property: PropertyType) -> Result<(), ComponentPropertyUpdateError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                if !component.has_property(property_name) {
                    return Err(ComponentPropertyUpdateError::PropertyDoesNotExist);
                }
                component.properties.retain(|property| property.name != property_name);
                component.properties.push(property.clone());
                // TODO:
                // self.event_manager
                //     .emit_event(SystemEvent::ComponentPropertyUpdated(ty.clone(), property.name.clone()));
            }
        }
        Ok(())
    }

    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                component.properties.retain(|property| property.name != property_name);
                self.event_manager
                    .emit_event(SystemEvent::ComponentPropertyRemoved(ty.clone(), property_name.to_string()));
            }
        }
    }

    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) -> Result<(), ComponentExtensionError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                if component.has_extension(&extension.ty) {
                    return Err(ComponentExtensionError::ExtensionAlreadyExists);
                }
                component.extensions.push(extension.clone());
                self.event_manager
                    .emit_event(SystemEvent::ComponentExtensionAdded(ty.clone(), extension.ty.clone()));
            }
        }
        Ok(())
    }

    fn update_extension(
        &self,
        component_ty: &ComponentTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<(), ComponentExtensionUpdateError> {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == component_ty {
                if !component.has_extension(extension_ty) {
                    return Err(ComponentExtensionUpdateError::ExtensionDoesNotExist);
                }
                component.extensions.retain(|extension| &extension.ty != extension_ty);
                component.extensions.push(extension.clone());
                // self.event_manager
                //     .emit_event(SystemEvent::ComponentExtensionRemoved(ty.clone(), extension_ty.clone()));
            }
        }
        Ok(())
    }

    fn remove_extension(&self, ty: &ComponentTypeId, extension_ty: &ExtensionTypeId) {
        let mut guard = self.components.0.write().unwrap();
        for component in guard.iter_mut() {
            if &component.ty == ty {
                component.extensions.retain(|extension| &extension.ty != extension_ty);
                self.event_manager
                    .emit_event(SystemEvent::ComponentExtensionRemoved(ty.clone(), extension_ty.clone()));
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
            if let Err(_) = self.register(component.clone()) {
                let _ = self.merge(component);
            }
        }
    }

    // TODO: move to own service ComponentCategoryManager
    fn get_component_categories(&self) -> Vec<String> {
        self.get_all()
            .iter()
            .filter_map(|component| {
                component
                    .extensions
                    .iter()
                    .find(|extension| extension.ty == EXTENSION_COMPONENT_CATEGORY.clone())
                    .and_then(|extension| extension.extension.as_str().map(str::to_string))
            })
            .collect()
    }
}

#[async_trait]
impl Lifecycle for ComponentManagerImpl {
    async fn init(&self) {
        self.create_base_components();
    }

    async fn shutdown(&self) {
        // TODO: remove?
        self.components.0.write().unwrap().clear();
    }
}
