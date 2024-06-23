use reactive_graph_type_system_api::ComponentCreationError;
use std::sync::Arc;

use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentAddExtensionError;
use reactive_graph_graph::ComponentAddPropertyError;
use reactive_graph_graph::ComponentRemoveExtensionError;
use reactive_graph_graph::ComponentRemovePropertyError;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentUpdateExtensionError;
use reactive_graph_graph::ComponentUpdatePropertyError;
use reactive_graph_graph::Components;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;

pub struct ComponentManagerDelegate {
    component_manager: Arc<dyn reactive_graph_type_system_api::ComponentManager + Send + Sync>,
}

impl ComponentManagerDelegate {
    pub fn new(component_manager: Arc<dyn reactive_graph_type_system_api::ComponentManager + Send + Sync>) -> Self {
        Self { component_manager }
    }
}
impl reactive_graph_plugin_api::ComponentManager for ComponentManagerDelegate {
    fn get_all(&self) -> Components {
        self.component_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> Components {
        self.component_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &ComponentTypeId) -> bool {
        self.component_manager.has(ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.component_manager.has_by_type(namespace, type_name)
    }

    fn get(&self, ty: &ComponentTypeId) -> Option<Component> {
        self.component_manager.get(ty)
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<Component> {
        self.component_manager.get_by_type(namespace, type_name)
    }

    fn find_by_type_name(&self, search: &str) -> Components {
        self.component_manager.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.component_manager.count()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.component_manager.count_by_namespace(namespace)
    }

    fn create(&self, ty: &ComponentTypeId, description: &str, properties: PropertyTypes, extensions: Extensions) -> Result<Component, ComponentCreationError> {
        self.component_manager.create_component(ty, description, properties, extensions)
    }

    fn replace(&self, ty: &ComponentTypeId, component: Component) {
        self.component_manager.replace(ty, component)
    }

    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType) -> Result<PropertyType, ComponentAddPropertyError> {
        self.component_manager.add_property(ty, property)
    }

    fn update_property(&self, ty: &ComponentTypeId, property_name: &str, property: PropertyType) -> Result<PropertyType, ComponentUpdatePropertyError> {
        self.component_manager.update_property(ty, property_name, property)
    }

    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str) -> Result<PropertyType, ComponentRemovePropertyError> {
        self.component_manager.remove_property(ty, property_name)
    }

    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension) -> Result<ExtensionTypeId, ComponentAddExtensionError> {
        self.component_manager.add_extension(ty, extension)
    }

    fn update_extension(
        &self,
        component_ty: &ComponentTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, ComponentUpdateExtensionError> {
        self.component_manager.update_extension(component_ty, extension_ty, extension)
    }

    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, ComponentRemoveExtensionError> {
        self.component_manager.remove_extension(component_ty, extension_ty)
    }

    fn delete(&self, ty: &ComponentTypeId) -> bool {
        self.component_manager.delete(ty)
    }
}
