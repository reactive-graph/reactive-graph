use std::sync::Arc;

use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::model::PropertyType;
use crate::model::RelationInstance;
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
    fn get_all(&self) -> Vec<FlowType> {
        self.flow_type_manager.get_all()
    }

    fn get_by_namespace(&self, namespace: &str) -> Vec<FlowType> {
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

    fn find(&self, search: &str) -> Vec<FlowType> {
        self.flow_type_manager.find(search)
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
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.flow_type_manager
            .create(ty, description, wrapper_entity_instance, entity_instances, relation_instances, variables, extensions);
    }

    fn add_entity_instance(&self, ty: &FlowTypeId, entity_instance: EntityInstance) {
        self.flow_type_manager.add_entity_instance(ty, entity_instance);
    }

    fn update_entity_instance(&self, ty: &FlowTypeId, id: Uuid, entity_instance: EntityInstance) {
        self.flow_type_manager.update_entity_instance(ty, id, entity_instance);
    }

    fn remove_entity_instance(&self, ty: &FlowTypeId, id: Uuid) {
        self.flow_type_manager.remove_entity_instance(ty, id);
    }

    fn add_extension(&self, ty: &FlowTypeId, extension: Extension) {
        self.flow_type_manager.add_extension(ty, extension);
    }

    fn update_extension(&self, ty: &FlowTypeId, extension: Extension) {
        self.flow_type_manager.update_extension(ty, extension);
    }

    fn remove_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId) {
        self.flow_type_manager.remove_extension(flow_ty, extension_ty);
    }

    fn add_variable(&self, ty: &FlowTypeId, variable: PropertyType) {
        self.flow_type_manager.add_variable(ty, variable);
    }

    fn update_variable(&self, ty: &FlowTypeId, variable_name: &str, variable: PropertyType) {
        self.flow_type_manager.update_variable(ty, variable_name, variable);
    }

    fn remove_variable(&self, ty: &FlowTypeId, variable_name: &str) {
        self.flow_type_manager.remove_variable(ty, variable_name);
    }

    fn delete(&self, ty: &FlowTypeId) -> bool {
        self.flow_type_manager.delete(ty)
    }

    fn validate(&self, ty: &FlowTypeId) -> bool {
        self.flow_type_manager.validate(ty)
    }

    fn import(&self, path: &str) {
        let _result = self.flow_type_manager.import(path);
    }

    fn export(&self, ty: &FlowTypeId, path: &str) {
        self.flow_type_manager.export(ty, path)
    }
}
