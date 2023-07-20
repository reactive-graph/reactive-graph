use std::sync::Arc;

use crate::model::ComponentTypeId;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;
use crate::plugins::EntityTypeCreationError;
use crate::plugins::EntityTypeImportError;
use crate::plugins::EntityTypeManager;

pub struct EntityTypeManagerImpl {
    entity_type_manager: Arc<dyn crate::api::EntityTypeManager>,
}

impl EntityTypeManagerImpl {
    pub fn new(entity_type_manager: Arc<dyn crate::api::EntityTypeManager>) -> Self {
        Self { entity_type_manager }
    }
}
impl EntityTypeManager for EntityTypeManagerImpl {
    fn get_all(&self) -> Vec<EntityType> {
        self.entity_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<EntityType> {
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

    fn find(&self, search: &str) -> Vec<EntityType> {
        self.entity_type_manager.find(search)
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
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<EntityType, EntityTypeCreationError> {
        self.entity_type_manager
            .create(ty, description, components, properties, extensions)
            .map_err(|_| EntityTypeCreationError::Failed)
    }

    fn add_component(&self, ty: &EntityTypeId, component: &ComponentTypeId) {
        let _ = self.entity_type_manager.add_component(ty, component);
    }

    fn remove_component(&self, ty: &EntityTypeId, component: &ComponentTypeId) {
        self.entity_type_manager.remove_component(ty, component);
    }

    fn add_property(&self, ty: &EntityTypeId, property: PropertyType) {
        let _ = self.entity_type_manager.add_property(ty, property);
    }

    fn remove_property(&self, ty: &EntityTypeId, property_name: &str) {
        self.entity_type_manager.remove_property(ty, property_name)
    }

    fn add_extension(&self, ty: &EntityTypeId, extension: Extension) {
        let _ = self.entity_type_manager.add_extension(ty, extension);
    }

    fn remove_extension(&self, ty: &EntityTypeId, extension_ty: &ExtensionTypeId) {
        self.entity_type_manager.remove_extension(ty, extension_ty)
    }

    fn delete(&self, ty: &EntityTypeId) -> bool {
        self.entity_type_manager.delete(ty)
    }

    fn validate(&self, ty: &EntityTypeId) -> bool {
        self.entity_type_manager.validate(ty)
    }

    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        self.entity_type_manager.import(path).map_err(|_| EntityTypeImportError::Failed)
    }

    fn export(&self, ty: &EntityTypeId, path: &str) {
        self.entity_type_manager.export(ty, path)
    }
}
