use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::FlowType;
use crate::model::PropertyType;
use crate::model::RelationInstance;
use crate::plugins::FlowTypeProvider;

#[derive(Debug)]
pub enum FlowTypeImportError {
    Io(std::io::Error),
    Deserialization(serde_json::Error),
}

impl From<std::io::Error> for FlowTypeImportError {
    fn from(e: std::io::Error) -> Self {
        FlowTypeImportError::Io(e)
    }
}

impl From<serde_json::Error> for FlowTypeImportError {
    fn from(e: serde_json::Error) -> Self {
        FlowTypeImportError::Deserialization(e)
    }
}

#[async_trait]
pub trait FlowTypeManager: Send + Sync + Lifecycle {
    fn register(&self, flow_type: FlowType) -> FlowType;

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
    fn create(
        &self,
        type_name: String,
        name: String,
        namespace: String,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    );

    /// Deletes the flow type with the given name.
    fn delete(&self, name: &str);

    /// Imports an flow type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError>;

    /// Exports the flow type with the given name to a JSON file located at the given path.
    fn export(&self, name: &str, path: &str);

    /// Registers an flow type provider.
    fn add_provider(&self, flow_type_provider: Arc<dyn FlowTypeProvider>);
}
