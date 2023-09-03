use std::sync::Arc;

use uuid::Uuid;
use inexor_rgf_core_model::{EntityInstances, Extensions, FlowTypeAddEntityInstanceError, FlowTypeAddExtensionError, FlowTypeAddVariableError, FlowTypeRemoveEntityInstanceError, FlowTypeRemoveExtensionError, FlowTypeRemoveVariableError, FlowTypes, FlowTypeUpdateEntityInstanceError, FlowTypeUpdateExtensionError, FlowTypeUpdateVariableError, PropertyTypes, RelationInstances, Variable};
use inexor_rgf_core_plugins::FlowTypeCreationError;

use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::model::PropertyType;
use crate::plugins::FlowTypeManager;

pub struct FlowTypeManagerImpl {
    flow_type_manager: Arc<dyn crate::api::FlowTypeManager>,
}

impl FlowTypeManagerImpl {
    pub fn new(flow_type_manager: Arc<dyn crate::api::FlowTypeManager>) -> Self {
        Self { flow_type_manager }
    }
}
impl FlowTypeManager for FlowTypeManagerImpl {
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
        self.flow_type_manager.create(ty, description, wrapper_entity_instance, entity_instances, relation_instances, variables, extensions).map_err(|_| FlowTypeCreationError {})
    }

    fn add_entity_instance(&self, flow_ty: &FlowTypeId, entity_instance: EntityInstance) -> Result<(), FlowTypeAddEntityInstanceError> {
        self.flow_type_manager.add_entity_instance(flow_ty, entity_instance)
    }

    fn update_entity_instance(&self, flow_ty: &FlowTypeId, id: Uuid, entity_instance: EntityInstance) -> Result<(Uuid, EntityInstance), FlowTypeUpdateEntityInstanceError> {
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
