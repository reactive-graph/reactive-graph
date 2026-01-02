use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::ComponentAddExtensionError;
use reactive_graph_graph::ComponentAddPropertyError;
use reactive_graph_graph::ComponentMergeError;
use reactive_graph_graph::ComponentRemoveExtensionError;
use reactive_graph_graph::ComponentRemovePropertyError;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::ComponentUpdateError;
use reactive_graph_graph::ComponentUpdateExtensionError;
use reactive_graph_graph::ComponentUpdatePropertyError;
use reactive_graph_graph::Components;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::Namespaces;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeContainer;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentCreationError;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::ComponentRegistrationError;
use reactive_graph_type_system_api::NamespacedTypeManager;
use reactive_graph_type_system_api::TypeSystemEvent;
use reactive_graph_type_system_api::TypeSystemEventManager;

#[derive(Component)]
pub struct ComponentManagerImpl {
    event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    namespaced_type_manager: Arc<dyn NamespacedTypeManager + Send + Sync>,

    #[component(default = "Components::new")]
    components: Components,
}

#[async_trait]
#[component_alias]
impl ComponentManager for ComponentManagerImpl {
    fn register(&self, component: reactive_graph_graph::Component) -> Result<reactive_graph_graph::Component, ComponentRegistrationError> {
        let ty = component.ty.clone();
        self.namespaced_type_manager.register(ty.namespaced_type())?;
        if self.components.contains_key(&ty) {
            return Err(ComponentRegistrationError::ComponentAlreadyExists(ty));
        }
        self.components.push(component.clone());
        debug!("Registered component {ty}");
        self.event_manager.emit_event(TypeSystemEvent::ComponentCreated(ty));
        Ok(component)
    }

    // Returns a copy
    fn get_all(&self) -> Components {
        self.components.clone()
    }

    fn get_type_ids(&self) -> ComponentTypeIds {
        self.components.type_ids()
    }

    fn get_namespaces(&self) -> Namespaces {
        self.components.namespaces()
    }

    fn get_by_namespace(&self, namespace: &Namespace) -> Components {
        self.components.get_by_namespace(namespace)
    }

    fn get_types_by_namespace(&self, namespace: &Namespace) -> ComponentTypeIds {
        self.components.get_types_by_namespace(namespace)
    }

    fn has(&self, ty: &ComponentTypeId) -> bool {
        self.components.contains_key(ty)
    }

    fn has_all(&self, tys: &ComponentTypeIds) -> bool {
        tys.iter().all(|ty| self.components.contains_key(&ty))
    }

    fn get(&self, ty: &ComponentTypeId) -> Option<reactive_graph_graph::Component> {
        self.components.get(ty)
    }

    fn get_by_types(&self, tys: ComponentTypeIds) -> Components {
        self.components.get_by_types(tys)
    }

    fn find(&self, search: &str) -> Components {
        self.components.find(search)
    }

    fn count(&self) -> usize {
        self.components.len()
    }

    fn count_by_namespace(&self, namespace: &Namespace) -> usize {
        self.components.count_by_namespace(namespace)
    }

    fn create_component(
        &self,
        ty: &ComponentTypeId,
        description: &str,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<reactive_graph_graph::Component, ComponentCreationError> {
        let component = reactive_graph_graph::Component::new(ty.clone(), description, properties.to_vec(), extensions.to_vec());
        self.register(component).map_err(ComponentCreationError::RegistrationError)
    }

    fn replace(&self, ty: &ComponentTypeId, r_component: reactive_graph_graph::Component) {
        for mut component in self.components.iter_mut() {
            if &component.ty == ty {
                if self.namespaced_type_manager.replace(ty.as_ref(), r_component.namespaced_type()).is_ok() {
                    component.ty = r_component.ty.clone();
                    component.description = r_component.description.clone();
                    component.properties = r_component.properties.clone();
                    component.extensions = r_component.extensions.clone();
                    // TODO: Notify about changed component -> This effects reactive instances which contains the component -> Add/remove property instances
                }
            }
        }
    }

    fn update_description(&self, ty: &ComponentTypeId, description: &str) -> Result<reactive_graph_graph::Component, ComponentUpdateError> {
        if !self.has(ty) {
            return Err(ComponentUpdateError::ComponentDoesNotExist(ty.clone()));
        }
        for mut component in self.components.iter_mut() {
            if &component.ty == ty {
                component.description = description.to_string();
                // TODO: Notify about changed component
            }
        }
        self.get(ty).ok_or(ComponentUpdateError::ComponentDoesNotExist(ty.clone()))
    }

    fn merge(&self, component_to_merge: reactive_graph_graph::Component) -> Result<reactive_graph_graph::Component, ComponentMergeError> {
        self.components.merge(component_to_merge).inspect(|_component| {
            // TODO: Notify about changed component -> This effects reactive instances which contains the component -> Add/remove property instances
        })
    }

    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<PropertyType, ComponentAddPropertyError> {
        let Some(component) = self.components.get_mut(ty) else {
            return Err(ComponentAddPropertyError::ComponentDoesNotExist(ty.clone()));
        };
        component
            .add_property(property)
            .map_err(ComponentAddPropertyError::AddPropertyError)
            .inspect(|property_type| {
                self.event_manager
                    .emit_event(TypeSystemEvent::ComponentPropertyAdded(ty.clone(), property_type.name.clone()))
            })
    }

    fn update_property(&self, ty: &ComponentTypeId, property_name: &str, property: PropertyType) -> Result<PropertyType, ComponentUpdatePropertyError> {
        let Some(component) = self.components.get_mut(ty) else {
            return Err(ComponentUpdatePropertyError::ComponentDoesNotExist(ty.clone()));
        };
        component
            .update_property(property_name, property)
            .map_err(ComponentUpdatePropertyError::UpdatePropertyError)
            .inspect(|property| {
                if property.name != property_name {
                    self.event_manager
                        .emit_event(TypeSystemEvent::ComponentPropertyRenamed(ty.clone(), property_name.to_string(), property.name.clone()))
                }
                self.event_manager
                    .emit_event(TypeSystemEvent::ComponentPropertyUpdated(ty.clone(), property.name.clone()))
            })
    }

    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) -> Result<PropertyType, ComponentRemovePropertyError> {
        let Some(component) = self.components.get_mut(ty) else {
            return Err(ComponentRemovePropertyError::ComponentDoesNotExist(ty.clone()));
        };
        component
            .remove_property(property_name)
            .map_err(ComponentRemovePropertyError::RemovePropertyError)
            .inspect(|property_type| {
                self.event_manager
                    .emit_event(TypeSystemEvent::ComponentPropertyRemoved(ty.clone(), property_type.name.clone()))
            })
    }

    fn add_extension(&self, component_ty: &ComponentTypeId, extension: Extension) -> Result<ExtensionTypeId, ComponentAddExtensionError> {
        let Some(component) = self.components.get_mut(component_ty) else {
            return Err(ComponentAddExtensionError::ComponentDoesNotExist(component_ty.clone()));
        };
        component
            .add_extension(extension)
            .map_err(ComponentAddExtensionError::AddExtensionError)
            .inspect(|extension_ty| {
                self.event_manager
                    .emit_event(TypeSystemEvent::ComponentExtensionAdded(component_ty.clone(), extension_ty.clone()))
            })
    }

    fn update_extension(
        &self,
        component_ty: &ComponentTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, ComponentUpdateExtensionError> {
        let Some(component) = self.components.get_mut(component_ty) else {
            return Err(ComponentUpdateExtensionError::ComponentDoesNotExist(component_ty.clone()));
        };
        component
            .update_extension(extension_ty, extension)
            .map_err(ComponentUpdateExtensionError::UpdateExtensionError)
            .inspect(|extension| {
                if extension_ty != &extension.ty {
                    self.event_manager
                        .emit_event(TypeSystemEvent::ComponentExtensionRenamed(component_ty.clone(), extension_ty.clone(), extension.ty.clone()))
                }
                self.event_manager
                    .emit_event(TypeSystemEvent::ComponentExtensionUpdated(component_ty.clone(), extension.ty.clone()))
            })
    }

    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, ComponentRemoveExtensionError> {
        let Some(component) = self.components.get_mut(component_ty) else {
            return Err(ComponentRemoveExtensionError::ComponentDoesNotExist(component_ty.clone()));
        };
        component
            .remove_extension(extension_ty)
            .map_err(ComponentRemoveExtensionError::RemoveExtensionError)
            .inspect(|extension| {
                self.event_manager
                    .emit_event(TypeSystemEvent::ComponentExtensionRemoved(component_ty.clone(), extension.ty.clone()))
            })
    }

    fn delete(&self, ty: &ComponentTypeId) -> bool {
        self.namespaced_type_manager.delete(ty.as_ref());
        self.components
            .remove(ty)
            .inspect(|(ty, _)| self.event_manager.emit_event(TypeSystemEvent::ComponentDeleted(ty.clone())))
            .is_some()
    }
}

#[async_trait]
impl Lifecycle for ComponentManagerImpl {
    async fn shutdown(&self) {
        self.components.clear()
    }
}

#[cfg(test)]
mod test {
    use crate::TypeSystemSystemImpl;
    use reactive_graph_graph::Component;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::Components;
    use reactive_graph_graph::Extensions;
    use reactive_graph_graph::NamespacedTypeContainer;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RandomNamespacedTypeId;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_type_system_api::TypeSystemSystem;
    use reactive_graph_utils_test::r_string;

    #[test]
    fn test_register_component() {
        reactive_graph_utils_test::init_logger();

        let component_ty = ComponentTypeId::random_type_id().unwrap();
        // let extension_ty = ComponentTypeId::default_test();

        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let component_manager = type_system.get_component_manager();
        let description = r_string();
        let properties = PropertyTypes::random_types(1..5).unwrap();
        let extensions = Extensions::random_types(1..3).unwrap();
        let component = Component::new(&component_ty, &description, properties, extensions);
        assert!(component_manager.register(component.clone()).is_ok());
        assert!(component_manager.has(&component_ty));

        let component_2 = component_manager.get(&component_ty).expect("Could not get component");
        assert_eq!(component.namespace(), component_2.namespace());
        assert_eq!(component.properties.len(), component_2.properties.len());
        assert_eq!(component.properties.names(), component_2.properties.names());
        assert_eq!(component.extensions.len(), component_2.extensions.len());
        assert_eq!(component.extensions.type_ids(), component_2.extensions.type_ids());
    }

    #[test]
    fn test_get_all_and_has_components() {
        reactive_graph_utils_test::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let component_manager = type_system.get_component_manager();
        let non_registered_components = Components::random_types(1..3).unwrap();
        for non_registered_component in non_registered_components.iter() {
            component_manager
                .register(non_registered_component.clone())
                .expect("Failed to register component");
        }

        let components = component_manager.get_all();
        for component in components.iter() {
            assert!(component_manager.has(&component.ty));
        }
        let non_registered_component_ty = ComponentTypeId::random_type_id().unwrap();
        assert!(!component_manager.has(&non_registered_component_ty));
    }
}
