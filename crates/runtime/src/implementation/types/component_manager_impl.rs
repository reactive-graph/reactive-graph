use async_trait::async_trait;
use log::debug;

use crate::api::ComponentManager;
use crate::api::Lifecycle;
use crate::api::SystemEventManager;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ComponentAddExtensionError;
use crate::model::ComponentAddPropertyError;
use crate::model::ComponentMergeError;
use crate::model::ComponentRemoveExtensionError;
use crate::model::ComponentRemovePropertyError;
use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::ComponentUpdateExtensionError;
use crate::model::ComponentUpdatePropertyError;
use crate::model::Components;
use crate::model::Extension;
use crate::model::ExtensionContainer;
use crate::model::ExtensionTypeId;
use crate::model::NamespacedTypeContainer;
use crate::model::Namespaces;
use crate::model::PropertyType;
use crate::model::PropertyTypeContainer;
use crate::plugins::SystemEvent;
use crate::rt_api::ComponentCreationError;
use crate::rt_api::ComponentRegistrationError;

#[wrapper]
pub struct ComponentsStorage(Components);

#[provides]
fn create_components_storage() -> ComponentsStorage {
    ComponentsStorage(Components::new())
}

#[component]
pub struct ComponentManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    components: ComponentsStorage,
}

#[async_trait]
#[provides]
impl ComponentManager for ComponentManagerImpl {
    fn register(&self, component: crate::model::Component) -> Result<crate::model::Component, ComponentRegistrationError> {
        let ty = component.ty.clone();
        if self.components.contains_key(&ty) {
            return Err(ComponentRegistrationError::ComponentAlreadyExists(ty));
        }
        self.components.push(component.clone());
        debug!("Registered component {ty}");
        self.event_manager.emit_event(SystemEvent::ComponentCreated(ty));
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

    fn get_by_namespace(&self, namespace: &str) -> Components {
        self.components.get_by_namespace(namespace)
    }

    fn get_types_by_namespace(&self, namespace: &str) -> ComponentTypeIds {
        self.components.get_types_by_namespace(namespace)
    }

    fn has(&self, ty: &ComponentTypeId) -> bool {
        self.components.contains_key(ty)
    }

    fn has_by_type(&self, namespace: &str, name: &str) -> bool {
        self.components.contains_key(&ComponentTypeId::new_from_type(namespace, name))
    }

    fn get(&self, ty: &ComponentTypeId) -> Option<crate::model::Component> {
        self.components.get(ty)
    }

    fn get_by_type(&self, namespace: &str, name: &str) -> Option<crate::model::Component> {
        self.components.get(ComponentTypeId::new_from_type(namespace, name))
    }

    fn get_by_types(&self, tys: ComponentTypeIds) -> Components {
        self.components.get_by_types(tys)
    }

    fn find_by_type_name(&self, search: &str) -> Components {
        self.components.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.components.len()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.components.count_by_namespace(namespace)
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
        for mut component in self.components.iter_mut() {
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
                    .emit_event(SystemEvent::ComponentPropertyAdded(ty.clone(), property_type.name.clone()))
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
                        .emit_event(SystemEvent::ComponentPropertyRenamed(ty.clone(), property_name.to_string(), property.name.clone()))
                }
                self.event_manager
                    .emit_event(SystemEvent::ComponentPropertyUpdated(ty.clone(), property.name.clone()))
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
                    .emit_event(SystemEvent::ComponentPropertyRemoved(ty.clone(), property_type.name.clone()))
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
                    .emit_event(SystemEvent::ComponentExtensionAdded(component_ty.clone(), extension_ty.clone()))
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
                        .emit_event(SystemEvent::ComponentExtensionRenamed(component_ty.clone(), extension_ty.clone(), extension.ty.clone()))
                }
                self.event_manager
                    .emit_event(SystemEvent::ComponentExtensionUpdated(component_ty.clone(), extension.ty.clone()))
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
                    .emit_event(SystemEvent::ComponentExtensionRemoved(component_ty.clone(), extension.ty.clone()))
            })
    }

    fn delete(&self, ty: &ComponentTypeId) -> bool {
        self.components
            .remove(ty)
            .inspect(|(ty, _)| self.event_manager.emit_event(SystemEvent::ComponentDeleted(ty.clone())))
            .is_some()
    }

    // fn register_provider(&self, component_provider: Arc<dyn ComponentProvider>) {
    //     for component in component_provider.get_components().iter() {
    //         trace!("Registering component: {}", component.key());
    //         if self.register(component.clone()).is_err() {
    //             trace!("Merging component: {}", component.key());
    //             let _ = self.merge(component.clone());
    //         }
    //     }
    // }
}

#[async_trait]
impl Lifecycle for ComponentManagerImpl {
    async fn shutdown(&self) {
        self.components.clear()
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use std::process::Termination;
    use test::Bencher;

    use default_test::DefaultTest;
    use serde_json::json;

    use crate::get_runtime;
    use crate::model::Component;
    use crate::model::ComponentTypeId;
    use crate::model::Extension;
    use crate::model::ExtensionContainer;
    use crate::model::ExtensionTypeId;
    use crate::model::NamespacedTypeGetter;
    use crate::model::PropertyType;
    use crate::model::PropertyTypeContainer;
    use crate::test_utils::r_string;

    #[test]
    fn test_register_component() {
        let runtime = get_runtime();
        let component_manager = runtime.get_component_manager();
        let namespace = r_string();
        let component_name = r_string();
        let description = r_string();
        let property_name = r_string();
        let extension_name = r_string();
        let component_ty = ComponentTypeId::new_from_type(namespace.clone(), component_name.clone());
        let extension_ty = ExtensionTypeId::new_from_type(&namespace, &extension_name);
        let component = Component::new(
            &component_ty,
            &description,
            vec![PropertyType::string(&property_name)],
            vec![Extension::new(extension_ty.clone(), "", json!(""))],
        );
        assert!(component_manager.register(component).is_ok());
        assert!(component_manager.has(&component_ty));

        let component = component_manager.get(&component_ty).unwrap();
        assert_eq!(namespace, component.namespace());
        assert_eq!(component_name, component.type_name());
        assert!(component.has_own_property(property_name.clone()));
        assert!(!component.has_own_property(r_string()));
        assert!(component.has_own_extension(&extension_ty));
    }

    #[test]
    fn test_get_components() {
        let runtime = get_runtime();
        let component_manager = runtime.get_component_manager();
        let components = component_manager.get_all();
        for component in components.iter() {
            assert!(component_manager.has(&component.ty));
        }
        assert!(!component_manager.has(&ComponentTypeId::new_from_type(r_string(), r_string())));
    }

    #[bench]
    fn creation_benchmark(bencher: &mut Bencher) -> impl Termination {
        let runtime = get_runtime();
        let component_manager = runtime.get_component_manager();
        let component = Component::default_test();
        let ty = component.ty.clone();
        bencher.iter(move || {
            let _ = component_manager.register(component.clone());
            component_manager.delete(&ty);
        })
    }
}
