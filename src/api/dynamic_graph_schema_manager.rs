use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaError;
use async_trait::async_trait;

use crate::api::Lifecycle;

#[async_trait]
pub trait DynamicGraph: Send + Sync + Lifecycle {
    /// Returns true, if the type system has been modified.
    fn is_type_system_modified(&self) -> bool;

    /// Returns the SDL of the Dynamic Graph schema.
    fn create_dynamic_schema(&self) -> Result<Schema, SchemaError>;

    /// Regenerates the Dynamic Graph schema.
    fn regenerate_dynamic_schema(&self);

    /// Regenerates the Dynamic Graph schema if and only if the type system has been modified.
    fn regenerate_dynamic_schema_if_modified(&self);
}
