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

#[derive(Debug)]
pub enum FlowTypeManagerError {
    InitializationError,
}

#[derive(Debug)]
pub struct FlowTypeCreationError;

pub trait FlowTypeManager: Send + Sync {
    /// Returns all flow types.
    fn get_all(&self) -> FlowTypes;

    /// Returns all flow types.
    fn get_by_namespace(&self, namespace: &str) -> FlowTypes;

    /// Returns true, if a flow type with the given name exists.
    fn has(&self, ty: &FlowTypeId) -> bool;

    /// Returns true, if a flow type with the given name exists.
    fn has_by_type(&self, namespace: &str, name: &str) -> bool;

    /// Returns the flow type with the given name or empty.
    fn get(&self, ty: &FlowTypeId) -> Option<FlowType>;

    /// Returns the flow type with the given name or empty.
    fn get_by_type(&self, namespace: &str, name: &str) -> Option<FlowType>;

    /// Returns all flow types whose names matches the given search string.
    fn find_by_type_name(&self, search: &str) -> FlowTypes;

    /// Returns the count of flow types.
    fn count(&self) -> usize;

    /// Returns the count of flow types of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new flow type.
    #[allow(clippy::too_many_arguments)]
    fn create(
        &self,
        ty: &FlowTypeId,
        description: &str,
        wrapper_entity_instance: EntityInstance,
        entity_instances: EntityInstances,
        relation_instances: RelationInstances,
        variables: PropertyTypes,
        extensions: Extensions,
    ) -> Result<FlowType, FlowTypeCreationError>;

    /// Adds the given entity instance to the flow type with the given name.
    fn add_entity_instance(&self, ty: &FlowTypeId, entity_instance: EntityInstance) -> Result<(), FlowTypeAddEntityInstanceError>;

    /// Updates the entity instance with the given id of the flow type with the given name.
    fn update_entity_instance(
        &self,
        ty: &FlowTypeId,
        id: Uuid,
        entity_instance: EntityInstance,
    ) -> Result<(Uuid, EntityInstance), FlowTypeUpdateEntityInstanceError>;

    /// Removes the entity instance with the given id from the flow type with the given name.
    fn remove_entity_instance(&self, ty: &FlowTypeId, id: Uuid) -> Result<Option<(Uuid, EntityInstance)>, FlowTypeRemoveEntityInstanceError>;

    /// Adds the given extension to the given flow type.
    fn add_extension(&self, ty: &FlowTypeId, extension: Extension) -> Result<ExtensionTypeId, FlowTypeAddExtensionError>;

    /// Updates the extension with the given type of the given flow type.
    fn update_extension(&self, ty: &FlowTypeId, extension_ty: &ExtensionTypeId, extension: Extension) -> Result<Extension, FlowTypeUpdateExtensionError>;

    /// Removes the extension with the given type from the given flow type.
    fn remove_extension(&self, flow_ty: &FlowTypeId, extension_ty: &ExtensionTypeId) -> Result<Extension, FlowTypeRemoveExtensionError>;

    /// Adds the given variable to the given flow type.
    fn add_variable(&self, ty: &FlowTypeId, variable: PropertyType) -> Result<Variable, FlowTypeAddVariableError>;

    /// Updates the variable with the given name of the flow type with the given name.
    fn update_variable(&self, ty: &FlowTypeId, variable_name: &str, variable: PropertyType) -> Result<Variable, FlowTypeUpdateVariableError>;

    /// Removes the variable with the given name from the flow type with the given name.
    fn remove_variable(&self, ty: &FlowTypeId, variable_name: &str) -> Result<Variable, FlowTypeRemoveVariableError>;

    /// Deletes the flow type with the given name.
    fn delete(&self, ty: &FlowTypeId) -> Option<FlowType>;

    /// Validates the flow type with the given name.
    /// Tests that all entity types and relation types exists and are valid.
    fn validate(&self, ty: &FlowTypeId) -> bool;
}
