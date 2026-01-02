use std::sync::Arc;

use uuid::Uuid;

use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityInstances;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeAddEntityInstanceError;
use reactive_graph_graph::FlowTypeAddExtensionError;
use reactive_graph_graph::FlowTypeAddVariableError;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::FlowTypeRemoveEntityInstanceError;
use reactive_graph_graph::FlowTypeRemoveExtensionError;
use reactive_graph_graph::FlowTypeRemoveVariableError;
use reactive_graph_graph::FlowTypeUpdateEntityInstanceError;
use reactive_graph_graph::FlowTypeUpdateError;
use reactive_graph_graph::FlowTypeUpdateExtensionError;
use reactive_graph_graph::FlowTypeUpdateVariableError;
use reactive_graph_graph::FlowTypes;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::RelationInstances;
use reactive_graph_graph::Variable;
use reactive_graph_type_system_api::FlowTypeCreationError;

pub struct FlowTypeManagerDelegate {
    flow_type_manager: Arc<dyn reactive_graph_type_system_api::FlowTypeManager + Send + Sync>,
}

impl FlowTypeManagerDelegate {
    pub fn new(flow_type_manager: Arc<dyn reactive_graph_type_system_api::FlowTypeManager + Send + Sync>) -> Self {
        Self { flow_type_manager }
    }
}
impl reactive_graph_plugin_api::FlowTypeManager for FlowTypeManagerDelegate {
    fn get_all(&self) -> FlowTypes {
        self.flow_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &Namespace) -> FlowTypes {
        self.flow_type_manager.get_by_namespace(namespace)
    }

    fn has(&self, ty: &FlowTypeId) -> bool {
        self.flow_type_manager.has(ty)
    }

    fn get(&self, ty: &FlowTypeId) -> Option<FlowType> {
        self.flow_type_manager.get(ty)
    }

    fn find(&self, search: &str) -> FlowTypes {
        self.flow_type_manager.find(search)
    }

    fn count(&self) -> usize {
        self.flow_type_manager.count()
    }

    fn count_by_namespace(&self, namespace: &Namespace) -> usize {
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

    fn update_description(&self, ty: &FlowTypeId, description: &str) -> Result<FlowType, FlowTypeUpdateError> {
        self.flow_type_manager.update_description(ty, description)
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
