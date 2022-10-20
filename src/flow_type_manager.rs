use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::FlowType;
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
    fn get_flow_types(&self) -> Vec<FlowType>;

    /// Returns true, if a flow type with the given name exists.
    fn has(&self, name: &str) -> bool;

    /// Returns the flow type with the given name or empty.
    fn get(&self, name: &str) -> Option<FlowType>;

    /// Returns all flow types whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<FlowType>;

    /// Returns the count of flow types.
    fn count(&self) -> usize;

    /// Creates a new flow type.
    #[allow(clippy::too_many_arguments)]
    fn create(
        &self,
        namespace: &str,
        name: &str,
        description: &str,
        wrapper_entity_instance: EntityInstance,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    );

    /// Adds the given entity instance to the flow type with the given name.
    fn add_entity_instance(&self, name: &str, entity_instance: EntityInstance);

    /// Updates the entity instance with the given id of the flow type with the given name.
    fn update_entity_instance(&self, name: &str, id: Uuid, entity_instance: EntityInstance);

    /// Removes the entity instance with the given id from the flow type with the given name.
    fn remove_entity_instance(&self, name: &str, id: Uuid);

    /// Adds the given extension to the flow type with the given name.
    fn add_extension(&self, name: &str, extension: Extension);

    /// Updates the extension with the given name of the flow type with the given name.
    fn update_extension(&self, flow_name: &str, extension_name: &str, extension: Extension);

    /// Removes the extension with the given name from the flow type with the given name.
    fn remove_extension(&self, flow_name: &str, extension_name: &str);

    /// Adds the given variable to the flow type with the given name.
    fn add_variable(&self, name: &str, variable: PropertyType);

    /// Updates the variable with the given name of the flow type with the given name.
    fn update_variable(&self, flow_name: &str, variable_name: &str, variable: PropertyType);

    /// Removes the variable with the given name from the flow type with the given name.
    fn remove_variable(&self, flow_name: &str, variable_name: &str);

    /// Deletes the flow type with the given name.
    fn delete(&self, name: &str);

    /// Validates the flow type with the given name.
    /// Tests that all entity types and relation types exists and are valid.
    fn validate(&self, name: &str) -> bool;

    /// Imports an flow type from a JSON file file located at the given path.
    fn import(&self, path: &str);

    /// Exports the flow type with the given name to a JSON file located at the given path.
    fn export(&self, name: &str, path: &str);
}
