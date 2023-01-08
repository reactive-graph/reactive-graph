use std::sync::Arc;

use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_trait::async_trait;

use crate::api::Lifecycle;

#[async_trait]
pub trait DynamicGraphSchemaManager: Send + Sync + Lifecycle {
    /// Returns true, if the type system has been modified.
    fn is_type_system_modified(&self) -> bool;

    /// Returns the SDL of the Dynamic Graph Schema.
    fn create_dynamic_schema(&self) -> Result<Schema, SchemaError>;

    /// Regenerates the Dynamic Graph Schema.
    fn regenerate_dynamic_schema(&self);

    /// Regenerates the Dynamic Graph Schema if and only if the type system has been modified.
    fn regenerate_dynamic_schema_if_modified(&self);

    /// Returns the Dynamic Graph Schema.
    fn get_dynamic_schema(&self) -> Option<Arc<Schema>>;
}
