use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::model::PropertyType;
use crate::model::RelationInstance;

#[derive(Debug)]
pub enum FlowTypeManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum FlowTypeCreationError {
    Failed,
}

pub trait FlowTypeManager: Send + Sync {
    /// Returns all flow types.
    fn get_all(&self) -> Vec<FlowType>;

    /// Returns all flow types.
    fn get_by_namespace(&self, namespace: &str) -> Vec<FlowType>;

    /// Returns true, if a flow type with the given name exists.
    fn has(&self, ty: &FlowTypeId) -> bool;

    /// Returns true, if a flow type with the given name exists.
    fn has_by_type(&self, namespace: &str, name: &str) -> bool;

    /// Returns the flow type with the given name or empty.
    fn get(&self, ty: &FlowTypeId) -> Option<FlowType>;

    /// Returns the flow type with the given name or empty.
    fn get_by_type(&self, namespace: &str, name: &str) -> Option<FlowType>;

    /// Returns all flow types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<FlowType>;

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
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    );

    /// Adds the given entity instance to the flow type with the given name.
    fn add_entity_instance(&self, ty: &FlowTypeId, entity_instance: EntityInstance);

    /// Updates the entity instance with the given id of the flow type with the given name.
    fn update_entity_instance(&self, ty: &FlowTypeId, id: Uuid, entity_instance: EntityInstance);

    /// Removes the entity instance with the given id from the flow type with the given name.
    fn remove_entity_instance(&self, ty: &FlowTypeId, id: Uuid);

    /// Adds the given extension to the flow type with the given name.
    fn add_extension(&self, ty: &FlowTypeId, extension: Extension);

    /// Updates the extension with the given name of the flow type with the given name.
    fn update_extension(&self, ty: &FlowTypeId, extension_name: &str, extension: Extension);

    /// Removes the extension with the given name from the flow type with the given name.
    fn remove_extension(&self, ty: &FlowTypeId, extension_name: &str);

    /// Adds the given variable to the flow type with the given name.
    fn add_variable(&self, ty: &FlowTypeId, variable: PropertyType);

    /// Updates the variable with the given name of the flow type with the given name.
    fn update_variable(&self, ty: &FlowTypeId, variable_name: &str, variable: PropertyType);

    /// Removes the variable with the given name from the flow type with the given name.
    fn remove_variable(&self, ty: &FlowTypeId, variable_name: &str);

    /// Deletes the flow type with the given name.
    fn delete(&self, ty: &FlowTypeId);

    /// Validates the flow type with the given name.
    /// Tests that all entity types and relation types exists and are valid.
    fn validate(&self, ty: &FlowTypeId) -> bool;

    /// Imports an flow type from a JSON file file located at the given path.
    fn import(&self, path: &str);

    /// Exports the flow type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &FlowTypeId, path: &str);
}
