use std::sync::Arc;

use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::EntityType;
use crate::model::EntityTypeAddComponentError;
use crate::model::EntityTypeAddExtensionError;
use crate::model::EntityTypeAddPropertyError;
use crate::model::EntityTypeId;
use crate::model::EntityTypeRemoveComponentError;
use crate::model::EntityTypeRemoveExtensionError;
use crate::model::EntityTypeRemovePropertyError;
use crate::model::EntityTypes;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::Extensions;
use crate::model::PropertyType;
use crate::model::PropertyTypes;
use crate::plugins::EntityTypeManager;
use crate::rt_api::EntityTypeCreationError;

pub struct EntityTypeManagerImpl {
    entity_type_manager: Arc<dyn crate::api::EntityTypeManager>,
}

impl EntityTypeManagerImpl {
    pub fn new(entity_type_manager: Arc<dyn crate::api::EntityTypeManager>) -> Self {
        Self { entity_type_manager }
    }
}
impl EntityTypeManager for EntityTypeManagerImpl {
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
        self.entity_type_manager.create(ty, description, components, properties, extensions)
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
