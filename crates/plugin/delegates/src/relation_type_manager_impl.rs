use std::sync::Arc;

use inexor_rgf_graph::ComponentOrEntityTypeId;
use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::ComponentTypeIds;
use inexor_rgf_graph::Extension;
use inexor_rgf_graph::ExtensionTypeId;
use inexor_rgf_graph::Extensions;
use inexor_rgf_graph::PropertyType;
use inexor_rgf_graph::PropertyTypes;
use inexor_rgf_graph::RelationType;
use inexor_rgf_graph::RelationTypeAddComponentError;
use inexor_rgf_graph::RelationTypeAddExtensionError;
use inexor_rgf_graph::RelationTypeAddPropertyError;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_graph::RelationTypeRemoveComponentError;
use inexor_rgf_graph::RelationTypeRemoveExtensionError;
use inexor_rgf_graph::RelationTypeRemovePropertyError;
use inexor_rgf_graph::RelationTypeUpdateExtensionError;
use inexor_rgf_graph::RelationTypeUpdatePropertyError;
use inexor_rgf_graph::RelationTypes;
use inexor_rgf_type_system_api::RelationTypeCreationError;

pub struct RelationTypeManagerDelegate {
    relation_type_manager: Arc<dyn inexor_rgf_type_system_api::RelationTypeManager + Send + Sync>,
}

impl RelationTypeManagerDelegate {
    pub fn new(relation_type_manager: Arc<dyn inexor_rgf_type_system_api::RelationTypeManager + Send + Sync>) -> Self {
        Self { relation_type_manager }
    }
}
impl inexor_rgf_plugin_api::RelationTypeManager for RelationTypeManagerDelegate {
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
            .create_relation_type(outbound_type, ty, inbound_type, description, components, properties, extensions)
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
