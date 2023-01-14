use std::sync::Arc;

use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::plugins::RelationTypeCreationError;
use crate::plugins::RelationTypeImportError;
use crate::plugins::RelationTypeManager;

pub struct RelationTypeManagerImpl {
    relation_type_manager: Arc<dyn crate::api::RelationTypeManager>,
}

impl RelationTypeManagerImpl {
    pub fn new(relation_type_manager: Arc<dyn crate::api::RelationTypeManager>) -> Self {
        Self { relation_type_manager }
    }
}
impl RelationTypeManager for RelationTypeManagerImpl {
    fn get_all(&self) -> Vec<RelationType> {
        self.relation_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<RelationType> {
        self.relation_type_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &RelationTypeId) -> bool {
        self.relation_type_manager.has(ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.relation_type_manager.has_by_type(namespace, type_name)
    }

    fn get(&self, ty: &RelationTypeId) -> Option<RelationType> {
        self.relation_type_manager.get(ty)
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<RelationType> {
        self.relation_type_manager.get_by_type(namespace, type_name)
    }

    fn find(&self, search: &str) -> Vec<RelationType> {
        self.relation_type_manager.find(search)
    }

    fn count(&self) -> usize {
        self.relation_type_manager.count()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.relation_type_manager.count_by_namespace(namespace)
    }

    fn create(
        &self,
        outbound_type: &ComponentOrEntityTypeId,
        ty: &RelationTypeId,
        inbound_type: &ComponentOrEntityTypeId,
        description: &str,
        components: Vec<ComponentTypeId>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<RelationType, RelationTypeCreationError> {
        self.relation_type_manager
            .create(outbound_type, ty, inbound_type, description, components, properties, extensions)
            .map_err(|_| RelationTypeCreationError::Failed)
    }

    fn add_component(&self, ty: &RelationTypeId, component: &ComponentTypeId) {
        let _ = self.relation_type_manager.add_component(ty, component);
    }

    fn remove_component(&self, ty: &RelationTypeId, component: &ComponentTypeId) {
        self.relation_type_manager.remove_component(ty, component);
    }

    fn add_property(&self, ty: &RelationTypeId, property: PropertyType) {
        let _ = self.relation_type_manager.add_property(ty, property);
    }

    fn remove_property(&self, ty: &RelationTypeId, property_name: &str) {
        self.relation_type_manager.remove_property(ty, property_name)
    }

    fn add_extension(&self, ty: &RelationTypeId, extension: Extension) {
        let _ = self.relation_type_manager.add_extension(ty, extension);
    }

    fn remove_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId) {
        self.relation_type_manager.remove_extension(relation_ty, extension_ty)
    }

    fn delete(&self, ty: &RelationTypeId) {
        self.relation_type_manager.delete(ty)
    }

    fn validate(&self, ty: &RelationTypeId) -> bool {
        self.relation_type_manager.validate(ty)
    }

    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        self.relation_type_manager.import(path).map_err(|_| RelationTypeImportError::Failed)
    }

    fn export(&self, ty: &RelationTypeId, path: &str) {
        self.relation_type_manager.export(ty, path)
    }
}
