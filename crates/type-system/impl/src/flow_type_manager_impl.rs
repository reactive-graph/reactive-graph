use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use log::warn;
use springtime_di::component_alias;
use springtime_di::Component;
use uuid::Uuid;

use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::EntityInstances;
use reactive_graph_graph::Extension;
use reactive_graph_graph::ExtensionTypeId;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeAddEntityInstanceError;
use reactive_graph_graph::FlowTypeAddExtensionError;
use reactive_graph_graph::FlowTypeAddRelationInstanceError;
use reactive_graph_graph::FlowTypeAddVariableError;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::FlowTypeIds;
use reactive_graph_graph::FlowTypeRemoveEntityInstanceError;
use reactive_graph_graph::FlowTypeRemoveExtensionError;
use reactive_graph_graph::FlowTypeRemoveRelationInstanceError;
use reactive_graph_graph::FlowTypeRemoveVariableError;
use reactive_graph_graph::FlowTypeUpdateEntityInstanceError;
use reactive_graph_graph::FlowTypeUpdateExtensionError;
use reactive_graph_graph::FlowTypeUpdateRelationInstanceError;
use reactive_graph_graph::FlowTypeUpdateVariableError;
use reactive_graph_graph::FlowTypes;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeEntityInstanceContainer;
use reactive_graph_graph::NamespacedTypeExtensionContainer;
use reactive_graph_graph::NamespacedTypeRelationInstanceContainer;
use reactive_graph_graph::NamespacedTypeVariablesContainer;
use reactive_graph_graph::Namespaces;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstances;
use reactive_graph_graph::Variable;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeCreationError;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::FlowTypeRegistrationError;
use reactive_graph_type_system_api::RelationTypeManager;
use reactive_graph_type_system_api::TypeSystemEvent;
use reactive_graph_type_system_api::TypeSystemEventManager;

#[derive(Component)]
pub struct FlowTypeManagerImpl {
    event_manager: Arc<dyn TypeSystemEventManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    #[component(default = "FlowTypes::new")]
    flow_types: FlowTypes, // FlowTypesStorage,
}

#[async_trait]
#[component_alias]
impl FlowTypeManager for FlowTypeManagerImpl {
    fn register(&self, flow_type: FlowType) -> Result<FlowType, FlowTypeRegistrationError> {
        let flow_ty = flow_type.ty.clone();
        if self.has(&flow_ty) {
            return Err(FlowTypeRegistrationError::FlowTypeAlreadyExists(flow_ty));
        }
        // Check that the entity types of every declared entity instance exists
        for entity_ty in flow_type.uses_entity_types() {
            if !self.entity_type_manager.has(&entity_ty) {
                warn!("Flow type {flow_ty} not fully initialized: No entity type {entity_ty}");
            }
        }
        // Check that the relation type of every declared relation instance exists
        for relation_ty in flow_type.uses_relation_types() {
            if !self.relation_type_manager.has(&relation_ty) {
                warn!("Flow type {flow_ty} not fully initialized: No relation type named {relation_ty}");
            }
        }
        // TODO: Check that entity instances referenced by a relation instance exists
        // TODO: Check that relation instances outbound entity has correct entity_type
        // TODO: Check that relation instances inbound entity has correct entity_type
        self.flow_types.push(flow_type.clone());
        debug!("Registered flow type {flow_ty}");
        self.event_manager.emit_event(TypeSystemEvent::FlowTypeCreated(flow_ty));
        Ok(flow_type)
    }

    fn get_all(&self) -> FlowTypes {
        self.flow_types.clone()
    }

    fn get_type_ids(&self) -> FlowTypeIds {
        self.flow_types.type_ids()
    }

    fn get_namespaces(&self) -> Namespaces {
        self.flow_types.namespaces()
    }

    fn get_by_namespace(&self, namespace: &str) -> FlowTypes {
        self.flow_types.get_by_namespace(namespace)
    }

    fn get_types_by_namespace(&self, namespace: &str) -> FlowTypeIds {
        self.flow_types.get_types_by_namespace(namespace)
    }

    fn has(&self, ty: &FlowTypeId) -> bool {
        self.flow_types.contains_key(ty)
    }

    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool {
        self.has(&FlowTypeId::new_from_type(namespace, type_name))
    }

    fn get(&self, ty: &FlowTypeId) -> Option<FlowType> {
        self.flow_types.get(ty).map(|entity_type| entity_type.value().clone())
    }

    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<FlowType> {
        self.get(&FlowTypeId::new_from_type(namespace, type_name))
    }

    fn find_by_type_name(&self, search: &str) -> FlowTypes {
        self.flow_types.find_by_type_name(search)
    }

    fn count(&self) -> usize {
        self.flow_types.len()
    }

    fn count_by_namespace(&self, namespace: &str) -> usize {
        self.flow_types.count_by_namespace(namespace)
    }

    fn create_flow_type(
        &self,
        ty: &FlowTypeId,
        description: &str,
        wrapper_entity_instance: EntityInstance,
        entity_instances: EntityInstances,
        relation_instances: RelationInstances,
        variables: PropertyTypes,
        extensions: Extensions,
    ) -> Result<FlowType, FlowTypeCreationError> {
        let flow_type = FlowType::builder()
            .ty(ty)
            .description(description)
            .wrapper_entity_instance(wrapper_entity_instance)
            .entity_instances(entity_instances)
            .relation_instances(relation_instances)
            .variables(variables)
            .extensions(extensions)
            .build();
        self.register(flow_type).map_err(FlowTypeCreationError::RegistrationError)
    }

    fn add_entity_instance(&self, ty: &FlowTypeId, entity_instance: EntityInstance) -> Result<(), FlowTypeAddEntityInstanceError> {
        self.flow_types.add_entity_instance(ty, entity_instance)
    }

    fn update_entity_instance(
        &self,
        ty: &FlowTypeId,
        id: Uuid,
        entity_instance: EntityInstance,
    ) -> Result<(Uuid, EntityInstance), FlowTypeUpdateEntityInstanceError> {
        self.flow_types.update_entity_instance(ty, id, entity_instance)
    }

    fn remove_entity_instance(&self, ty: &FlowTypeId, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, FlowTypeRemoveEntityInstanceError> {
        self.flow_types.remove_entity_instance(ty, id)
    }

    fn add_relation_instance(&self, flow_ty: &FlowTypeId, relation_instance: RelationInstance) -> Result<(), FlowTypeAddRelationInstanceError> {
        self.flow_types.add_relation_instance(flow_ty, relation_instance)
    }

    fn update_relation_instance(
        &self,
        flow_ty: &FlowTypeId,
        id: &RelationInstanceId,
        relation_instance: RelationInstance,
    ) -> Result<(RelationInstanceId, RelationInstance), FlowTypeUpdateRelationInstanceError> {
        self.flow_types.update_relation_instance(flow_ty, id, relation_instance)
    }

    fn remove_relation_instance(
        &self,
        flow_ty: &FlowTypeId,
        id: &RelationInstanceId,
    ) -> Result<Option<(RelationInstanceId, RelationInstance)>, FlowTypeRemoveRelationInstanceError> {
        self.flow_types.remove_relation_instance(flow_ty, id)
    }

    fn add_extension(&self, flow_ty: &FlowTypeId, extension: Extension) -> Result<ExtensionTypeId, FlowTypeAddExtensionError> {
        self.flow_types.add_extension(flow_ty, extension)
    }

    fn update_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId, extension: Extension) -> Result<Extension, FlowTypeUpdateExtensionError> {
        self.flow_types.update_extension(flow_ty, extension_ty, extension)
    }

    fn remove_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, FlowTypeRemoveExtensionError> {
        self.flow_types.remove_extension(flow_ty, extension_ty)
    }

    fn add_variable(&self, flow_ty: &FlowTypeId, variable: PropertyType) -> Result<Variable, FlowTypeAddVariableError> {
        self.flow_types.add_variable(flow_ty, variable)
        // let mut guard = self.flow_types.0.write().unwrap();
        // if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
        //     flow_type.variables.push(variable);
        // }
    }

    fn update_variable(&self, flow_ty: &FlowTypeId, variable_name: &str, variable: PropertyType) -> Result<Variable, FlowTypeUpdateVariableError> {
        self.flow_types.update_variable(flow_ty, variable_name, variable)

        // let mut guard = self.flow_types.0.write().unwrap();
        // if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
        //     flow_type.variables.retain(|variable| variable.name == variable_name);
        //     flow_type.variables.push(variable);
        // }
    }

    fn remove_variable(&self, flow_ty: &FlowTypeId, variable_name: &str) -> Result<Variable, FlowTypeRemoveVariableError> {
        self.flow_types.remove_variable(flow_ty, variable_name)
        // let mut guard = self.flow_types.0.write().unwrap();
        // if let Some(flow_type) = guard.iter_mut().find(|flow_type| &flow_type.ty == ty) {
        //     flow_type.variables.retain(|variable| variable.name == variable_name);
        // }
    }

    fn delete(&self, flow_ty: &FlowTypeId) -> Option<FlowType> {
        self.flow_types.remove(flow_ty).map(|(flow_ty, flow_type)| {
            self.event_manager.emit_event(TypeSystemEvent::FlowTypeDeleted(flow_ty.clone()));
            flow_type
        })
    }

    fn validate(&self, ty: &FlowTypeId) -> bool {
        if let Some(flow_type) = self.get(ty) {
            return flow_type
                .entity_instances
                .iter()
                .all(|entity_instance| self.entity_type_manager.validate(&entity_instance.ty))
                && flow_type
                    .relation_instances
                    .iter()
                    .all(|relation_instance| self.relation_type_manager.validate(&relation_instance.relation_type_id()));
        }
        false
    }
}

#[async_trait]
impl Lifecycle for FlowTypeManagerImpl {
    async fn shutdown(&self) {
        self.flow_types.clear();
    }
}
