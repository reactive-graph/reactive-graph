use std::sync::Arc;

use async_trait::async_trait;

use crate::api::Lifecycle;
use crate::model::{Extension, PropertyType, RelationType};
use crate::plugins::RelationTypeProvider;

#[derive(Debug)]
pub struct RelationTypeImportError;

#[async_trait]
pub trait RelationTypeManager: Send + Sync + Lifecycle {
    // TODO: Result
    fn register(&self, entity_type: RelationType);
    fn get_relation_types(&self) -> Vec<RelationType>;

    fn has(&self, type_name: String) -> bool;
    fn has_starts_with(&self, type_name: String) -> bool;
    fn get(&self, type_name: String) -> Option<RelationType>;
    fn get_starts_with(&self, type_name_starts_with: String) -> Option<RelationType>;

    // TODO: Result
    fn create(
        &self,
        outbound_type: String,
        type_name: String,
        inbound_type: String,
        components: Vec<String>,
        behaviours: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    );
    fn delete(&self, type_name: String);

    fn import(&self, path: String) -> Result<RelationType, RelationTypeImportError>;
    fn export(&self, type_name: String, path: String);

    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>);
}
