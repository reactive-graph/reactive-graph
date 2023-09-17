use std::sync::Arc;

use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::ComponentTypeIds;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::Extensions;
use crate::model::PropertyType;
use crate::model::PropertyTypes;
use crate::model::RelationType;
use crate::model::RelationTypeAddComponentError;
use crate::model::RelationTypeAddExtensionError;
use crate::model::RelationTypeAddPropertyError;
use crate::model::RelationTypeId;
use crate::model::RelationTypeRemoveComponentError;
use crate::model::RelationTypeRemoveExtensionError;
use crate::model::RelationTypeRemovePropertyError;
use crate::model::RelationTypeUpdateExtensionError;
use crate::model::RelationTypeUpdatePropertyError;
use crate::model::RelationTypes;
use crate::plugins::RelationTypeCreationError;
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
    fn get_all(&self) -> RelationTypes {
        self.relation_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> RelationTypes {
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

    fn find_by_type_name(&self, search: &str) -> RelationTypes {
        self.relation_type_manager.find_by_type_name(search)
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
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<RelationType, RelationTypeCreationError> {
        self.relation_type_manager
            .create(outbound_type, ty, inbound_type, description, components, properties, extensions)
            .map_err(|_| RelationTypeCreationError {})
    }

    fn add_component(&self, ty: &RelationTypeId, component: &ComponentTypeId) -> Result<(), RelationTypeAddComponentError> {
        self.relation_type_manager.add_component(ty, component)
    }

    fn remove_component(&self, ty: &RelationTypeId, component: &ComponentTypeId) -> Result<ComponentTypeId, RelationTypeRemoveComponentError> {
        self.relation_type_manager.remove_component(ty, component)
    }

    fn add_property(&self, relation_ty: &RelationTypeId, property: PropertyType) -> Result<PropertyType, RelationTypeAddPropertyError> {
        self.relation_type_manager.add_property(relation_ty, property)
    }

    fn update_property(
        &self,
        relation_ty: &RelationTypeId,
        property_name: &str,
        property_type: PropertyType,
    ) -> Result<PropertyType, RelationTypeUpdatePropertyError> {
        self.relation_type_manager.update_property(relation_ty, property_name, property_type)
    }

    fn remove_property(&self, relation_ty: &RelationTypeId, property_name: &str) -> Result<PropertyType, RelationTypeRemovePropertyError> {
        self.relation_type_manager.remove_property(relation_ty, property_name)
    }

    fn add_extension(&self, relation_ty: &RelationTypeId, extension: Extension) -> Result<ExtensionTypeId, RelationTypeAddExtensionError> {
        self.relation_type_manager.add_extension(relation_ty, extension)
    }

    fn update_extension(
        &self,
        relation_ty: &RelationTypeId,
        extension_ty: &ExtensionTypeId,
        extension: Extension,
    ) -> Result<Extension, RelationTypeUpdateExtensionError> {
        self.relation_type_manager.update_extension(relation_ty, extension_ty, extension)
    }

    fn remove_extension(&self, relation_ty: &RelationTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, RelationTypeRemoveExtensionError> {
        self.relation_type_manager.remove_extension(relation_ty, extension_ty)
    }

    fn delete(&self, ty: &RelationTypeId) -> Option<RelationType> {
        self.relation_type_manager.delete(ty)
    }

    fn validate(&self, ty: &RelationTypeId) -> bool {
        self.relation_type_manager.validate(ty)
    }

    // fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
    //     self.relation_type_manager.import(path).map_err(|_| RelationTypeImportError {})
    // }
    //
    // fn export(&self, ty: &RelationTypeId, path: &str) {
    //     self.relation_type_manager.export(ty, path)
    // }
}
