use std::sync::Arc;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeAddComponentError;
use reactive_graph_graph::RelationTypeAddExtensionError;
use reactive_graph_graph::RelationTypeAddPropertyError;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::RelationTypeRemoveComponentError;
use reactive_graph_graph::RelationTypeRemoveExtensionError;
use reactive_graph_graph::RelationTypeRemovePropertyError;
use reactive_graph_graph::RelationTypeUpdateError;
use reactive_graph_graph::RelationTypeUpdateExtensionError;
use reactive_graph_graph::RelationTypeUpdatePropertyError;
use reactive_graph_graph::RelationTypes;
use reactive_graph_type_system_api::RelationTypeCreationError;

pub struct RelationTypeManagerDelegate {
    relation_type_manager: Arc<dyn reactive_graph_type_system_api::RelationTypeManager + Send + Sync>,
}

impl RelationTypeManagerDelegate {
    pub fn new(relation_type_manager: Arc<dyn reactive_graph_type_system_api::RelationTypeManager + Send + Sync>) -> Self {
        Self { relation_type_manager }
    }
}
impl reactive_graph_plugin_api::RelationTypeManager for RelationTypeManagerDelegate {
    fn get_all(&self) -> RelationTypes {
        self.relation_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &Namespace) -> RelationTypes {
        self.relation_type_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &RelationTypeId) -> bool {
        self.relation_type_manager.has(ty)
    }

    fn get(&self, ty: &RelationTypeId) -> Option<RelationType> {
        self.relation_type_manager.get(ty)
    }

    fn find(&self, search: &str) -> RelationTypes {
        self.relation_type_manager.find(search)
    }

    fn count(&self) -> usize {
        self.relation_type_manager.count()
    }

    fn count_by_namespace(&self, namespace: &Namespace) -> usize {
        self.relation_type_manager.count_by_namespace(namespace)
    }

    fn create(
        &self,
        outbound_type: &InboundOutboundType,
        ty: &RelationTypeId,
        inbound_type: &InboundOutboundType,
        description: &str,
        components: ComponentTypeIds,
        properties: PropertyTypes,
        extensions: Extensions,
    ) -> Result<RelationType, RelationTypeCreationError> {
        self.relation_type_manager
            .create_relation_type(outbound_type, ty, inbound_type, description, components, properties, extensions)
    }

    fn update_description(&self, ty: &RelationTypeId, description: &str) -> Result<RelationType, RelationTypeUpdateError> {
        self.relation_type_manager.update_description(ty, description)
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
