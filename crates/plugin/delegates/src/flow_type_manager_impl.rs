use std::sync::Arc;

use uuid::Uuid;

use inexor_rgf_graph::EntityInstance;
use inexor_rgf_graph::EntityInstances;
use inexor_rgf_graph::Extension;
use inexor_rgf_graph::ExtensionTypeId;
use inexor_rgf_graph::Extensions;
use inexor_rgf_graph::FlowType;
use inexor_rgf_graph::FlowTypeAddEntityInstanceError;
use inexor_rgf_graph::FlowTypeAddExtensionError;
use inexor_rgf_graph::FlowTypeAddVariableError;
use inexor_rgf_graph::FlowTypeId;
use inexor_rgf_graph::FlowTypeRemoveEntityInstanceError;
use inexor_rgf_graph::FlowTypeRemoveExtensionError;
use inexor_rgf_graph::FlowTypeRemoveVariableError;
use inexor_rgf_graph::FlowTypeUpdateEntityInstanceError;
use inexor_rgf_graph::FlowTypeUpdateExtensionError;
use inexor_rgf_graph::FlowTypeUpdateVariableError;
use inexor_rgf_graph::FlowTypes;
use inexor_rgf_graph::PropertyType;
use inexor_rgf_graph::PropertyTypes;
use inexor_rgf_graph::RelationInstances;
use inexor_rgf_graph::Variable;
use inexor_rgf_type_system_api::FlowTypeCreationError;

pub struct FlowTypeManagerDelegate {
    flow_type_manager: Arc<dyn inexor_rgf_type_system_api::FlowTypeManager + Send + Sync>,
}

impl FlowTypeManagerDelegate {
    pub fn new(flow_type_manager: Arc<dyn inexor_rgf_type_system_api::FlowTypeManager + Send + Sync>) -> Self {
        Self { flow_type_manager }
    }
}
impl inexor_rgf_plugin_api::FlowTypeManager for FlowTypeManagerDelegate {
    fn get_all(&self) -> FlowTypes {
        self.flow_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> FlowTypes {
        self.flow_type_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &FlowTypeId) -> bool {
        self.flow_type_manager.has(ty)
    }

    fn has_by_type(&self, namespace: &str, name: &str) -> bool {
        self.flow_type_manager.has_by_type(namespace, name)
    }

    fn get(&self, ty: &FlowTypeId) -> Option<FlowType> {
        self.flow_type_manager.get(ty)
    }

    fn get_by_type(&self, namespace: &str, name: &str) -> Option<FlowType> {
        self.flow_type_manager.get_by_type(namespace, name)
    }

    fn find_by_type_name(&self, search: &str) -> FlowTypes {
        self.flow_type_manager.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.flow_type_manager.count()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.flow_type_manager.count_by_namespace(namespace)
    }

    fn create(
        &self,
        ty: &FlowTypeId,
        description: &str,
        wrapper_entity_instance: EntityInstance,
        entity_instances: EntityInstances,
        relation_instances: RelationInstances,
        variables: PropertyTypes,
        extensions: Extensions,
    ) -> Result<FlowType, FlowTypeCreationError> {
        self.flow_type_manager
            .create_flow_type(ty, description, wrapper_entity_instance, entity_instances, relation_instances, variables, extensions)
    }

    fn add_entity_instance(&self, flow_ty: &FlowTypeId, entity_instance: EntityInstance) -> Result<(), FlowTypeAddEntityInstanceError> {
        self.flow_type_manager.add_entity_instance(flow_ty, entity_instance)
    }

    fn update_entity_instance(
        &self,
        flow_ty: &FlowTypeId,
        id: Uuid,
        entity_instance: EntityInstance,
    ) -> Result<(Uuid, EntityInstance), FlowTypeUpdateEntityInstanceError> {
        self.flow_type_manager.update_entity_instance(flow_ty, id, entity_instance)
    }

    fn remove_entity_instance(&self, flow_ty: &FlowTypeId, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, FlowTypeRemoveEntityInstanceError> {
        self.flow_type_manager.remove_entity_instance(flow_ty, id)
    }

    fn add_extension(&self, flow_ty: &FlowTypeId, extension: Extension) -> Result<ExtensionTypeId, FlowTypeAddExtensionError> {
        self.flow_type_manager.add_extension(flow_ty, extension)
    }

    fn update_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId, extension: Extension) -> Result<Extension, FlowTypeUpdateExtensionError> {
        self.flow_type_manager.update_extension(flow_ty, extension_ty, extension)
    }

    fn remove_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, FlowTypeRemoveExtensionError> {
        self.flow_type_manager.remove_extension(flow_ty, extension_ty)
    }

    fn add_variable(&self, ty: &FlowTypeId, variable: PropertyType) -> Result<Variable, FlowTypeAddVariableError> {
        self.flow_type_manager.add_variable(ty, variable)
    }

    fn update_variable(&self, ty: &FlowTypeId, variable_name: &str, variable: PropertyType) -> Result<Variable, FlowTypeUpdateVariableError> {
        self.flow_type_manager.update_variable(ty, variable_name, variable)
    }

    fn remove_variable(&self, ty: &FlowTypeId, variable_name: &str) -> Result<Variable, FlowTypeRemoveVariableError> {
        self.flow_type_manager.remove_variable(ty, variable_name)
    }

    fn delete(&self, ty: &FlowTypeId) -> Option<FlowType> {
        self.flow_type_manager.delete(ty)
    }

    fn validate(&self, ty: &FlowTypeId) -> bool {
        self.flow_type_manager.validate(ty)
    }

    // fn import(&self, path: &str) {
    //     let _result = self.flow_type_manager.import(path);
    // }
    //
    // fn export(&self, ty: &FlowTypeId, path: &str) {
    //     self.flow_type_manager.export(ty, path)
    // }
}
