use async_trait::async_trait;
use springtime_di::injectable;
use uuid::Uuid;

use crate::FlowTypeCreationError;
use crate::FlowTypeRegistrationError;
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
use reactive_graph_graph::FlowTypeUpdateError;
use reactive_graph_graph::FlowTypeUpdateExtensionError;
use reactive_graph_graph::FlowTypeUpdateRelationInstanceError;
use reactive_graph_graph::FlowTypeUpdateVariableError;
use reactive_graph_graph::FlowTypes;
use reactive_graph_graph::Namespaces;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstances;
use reactive_graph_graph::Variable;
use reactive_graph_lifecycle::Lifecycle;

#[injectable]
#[async_trait]
pub trait FlowTypeManager: Send + Sync + Lifecycle {
    fn register(&self, flow_type: FlowType) -> Result<FlowType, FlowTypeRegistrationError>;

    /// Returns all flow types.
    fn get_all(&self) -> FlowTypes;

    /// Returns the ids of all flow types.
    fn get_type_ids(&self) -> FlowTypeIds;

    /// Returns all defined namespaces.
    fn get_namespaces(&self) -> Namespaces;

    /// Returns all flow types.
    fn get_by_namespace(&self, namespace: &str) -> FlowTypes;

    fn get_types_by_namespace(&self, namespace: &str) -> FlowTypeIds;

    /// Returns true, if a flow type with the given name exists.
    fn has(&self, ty: &FlowTypeId) -> bool;

    /// Returns true, if a flow type with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, type_name: &str) -> bool;

    /// Returns the flow type with the given name or empty.
    fn get(&self, ty: &FlowTypeId) -> Option<FlowType>;

    /// Returns the flow type with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, type_name: &str) -> Option<FlowType>;

    /// Returns all flow types whose names matches the given search string.
    fn find_by_type_name(&self, search: &str) -> FlowTypes;

    /// Returns the count of flow types.
    fn count(&self) -> usize;

    /// Returns the count of flow types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new flow type.
    #[allow(clippy::too_many_arguments)]
    fn create_flow_type(
        &self,
        ty: &FlowTypeId,
        description: &str,
        wrapper_entity_instance: EntityInstance,
        entity_instances: EntityInstances,
        relation_instances: RelationInstances,
        variables: PropertyTypes,
        extensions: Extensions,
    ) -> Result<FlowType, FlowTypeCreationError>;

    /// Updates the description of the given flow type.
    fn update_description(&self, ty: &FlowTypeId, description: &str) -> Result<FlowType, FlowTypeUpdateError>;

    /// Adds the given entity instance to the given flow type.
    fn add_entity_instance(&self, ty: &FlowTypeId, entity_instance: EntityInstance) -> Result<(), FlowTypeAddEntityInstanceError>;

    /// Updates the entity instance with the given id of the given flow type.
    fn update_entity_instance(
        &self,
        ty: &FlowTypeId,
        id: Uuid,
        entity_instance: EntityInstance,
    ) -> Result<(Uuid, EntityInstance), FlowTypeUpdateEntityInstanceError>;

    /// Removes the entity instance with the given id from the given flow type.
    fn remove_entity_instance(&self, ty: &FlowTypeId, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, FlowTypeRemoveEntityInstanceError>;

    /// Adds the given relation instance to the given flow type.
    fn add_relation_instance(&self, ty: &FlowTypeId, relation_instance: RelationInstance) -> Result<(), FlowTypeAddRelationInstanceError>;

    /// Updates the relation instance with the given id of the given flow type.
    fn update_relation_instance(
        &self,
        ty: &FlowTypeId,
        id: &RelationInstanceId,
        relation_instance: RelationInstance,
    ) -> Result<(RelationInstanceId, RelationInstance), FlowTypeUpdateRelationInstanceError>;

    /// Removes the relation instance with the given id from the given flow type.
    fn remove_relation_instance(
        &self,
        ty: &FlowTypeId,
        id: &RelationInstanceId,
    ) -> Result<Option<(RelationInstanceId, RelationInstance)>, FlowTypeRemoveRelationInstanceError>;

    /// Adds the given extension to the given flow type.
    fn add_extension(&self, ty: &FlowTypeId, extension: Extension) -> Result<ExtensionTypeId, FlowTypeAddExtensionError>;

    /// Updates the extension with the given name of the given flow type.
    fn update_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId, extension: Extension) -> Result<Extension, FlowTypeUpdateExtensionError>;

    /// Removes the extension with the given name from the given flow type.
    fn remove_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, FlowTypeRemoveExtensionError>;

    /// Adds the given variable to the given flow type.
    fn add_variable(&self, ty: &FlowTypeId, variable: PropertyType) -> Result<Variable, FlowTypeAddVariableError>;

    /// Updates the variable with the given name of the given flow type.
    fn update_variable(&self, ty: &FlowTypeId, variable_name: &str, variable: PropertyType) -> Result<Variable, FlowTypeUpdateVariableError>;

    /// Removes the variable with the given name from the given flow type.
    fn remove_variable(&self, ty: &FlowTypeId, variable_name: &str) -> Result<Variable, FlowTypeRemoveVariableError>;

    /// Deletes the given flow type.
    fn delete(&self, ty: &FlowTypeId) -> Option<FlowType>;

    /// Validates the given flow type.
    /// Tests that all entity types and relation types exists and are valid.
    fn validate(&self, ty: &FlowTypeId) -> bool;
}
