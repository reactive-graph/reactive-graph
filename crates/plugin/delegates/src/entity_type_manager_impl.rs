use std::sync::Arc;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeAddComponentError;
use reactive_graph_graph::EntityTypeAddExtensionError;
use reactive_graph_graph::EntityTypeAddPropertyError;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::EntityTypeRemoveComponentError;
use reactive_graph_graph::EntityTypeRemoveExtensionError;
use reactive_graph_graph::EntityTypeRemovePropertyError;
use reactive_graph_graph::EntityTypeUpdateError;
use reactive_graph_graph::EntityTypes;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_type_system_api::EntityTypeCreationError;

pub struct EntityTypeManagerDelegate {
    entity_type_manager: Arc<dyn reactive_graph_type_system_api::EntityTypeManager + Send + Sync>,
}

impl EntityTypeManagerDelegate {
    pub fn new(entity_type_manager: Arc<dyn reactive_graph_type_system_api::EntityTypeManager + Send + Sync>) -> Self {
        Self { entity_type_manager }
    }
}
impl reactive_graph_plugin_api::EntityTypeManager for EntityTypeManagerDelegate {
    fn get_all(&self) -> EntityTypes {
        self.entity_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> EntityTypes {
        self.entity_type_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &EntityTypeId) -> bool {
        self.entity_type_manager.has(ty)
    }

    fn has_by_type(&self, namespace: &str, name: &str) -> bool {
        self.entity_type_manager.has_by_type(namespace, name)
    }

    fn get(&self, ty: &EntityTypeId) -> Option<EntityType> {
        self.entity_type_manager.get(ty)
    }

    fn get_by_type(&self, namespace: &str, name: &str) -> Option<EntityType> {
        self.entity_type_manager.get_by_type(namespace, name)
    }

    fn find_by_type_name(&self, search: &str) -> EntityTypes {
        self.entity_type_manager.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.entity_type_manager.count()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.entity_type_manager.count_by_namespace(namespace)
    }

    fn create(
        &self,
        ty: &EntityTypeId,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<EntityType, EntityTypeCreationError> {
        self.entity_type_manager.create_entity_type(ty, description, components, properties, extensions)
    }

    fn update_description(&self, ty: &EntityTypeId, description: &str) -> Result<EntityType, EntityTypeUpdateError> {
        self.entity_type_manager.update_description(ty, description)
    }

    fn add_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<(), EntityTypeAddComponentError> {
        self.entity_type_manager.add_component(entity_ty, component_ty)
    }

    fn remove_component(&self, entity_ty: &EntityTypeId, component_ty: &ComponentTypeId) -> Result<ComponentTypeId, EntityTypeRemoveComponentError> {
        self.entity_type_manager.remove_component(entity_ty, component_ty)
    }

    fn add_property(&self, ty: &EntityTypeId, property: PropertyType) -> Result<PropertyType, EntityTypeAddPropertyError> {
        self.entity_type_manager.add_property(ty, property)
    }

    fn remove_property(&self, ty: &EntityTypeId, property_name: &str) -> Result<PropertyType, EntityTypeRemovePropertyError> {
        self.entity_type_manager.remove_property(ty, property_name)
    }

    fn add_extension(&self, ty: &EntityTypeId, extension: Extension) -> Result<ExtensionTypeId, EntityTypeAddExtensionError> {
        self.entity_type_manager.add_extension(ty, extension)
    }

    fn remove_extension(&self, ty: &EntityTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, EntityTypeRemoveExtensionError> {
        self.entity_type_manager.remove_extension(ty, extension_ty)
    }

    fn delete(&self, ty: &EntityTypeId) -> Option<EntityType> {
        self.entity_type_manager.delete(ty)
    }

    fn validate(&self, ty: &EntityTypeId) -> bool {
        self.entity_type_manager.validate(ty)
    }

    // fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
    //     self.entity_type_manager.import(path).map_err(|_| EntityTypeImportError {})
    // }
    //
    // fn export(&self, ty: &EntityTypeId, path: &str) {
    //     self.entity_type_manager.export(ty, path)
    // }
}
