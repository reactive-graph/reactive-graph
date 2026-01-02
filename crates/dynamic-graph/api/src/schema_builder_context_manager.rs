use async_trait::async_trait;
use springtime_di::injectable;

use reactive_graph_lifecycle::Lifecycle;

use crate::SchemaBuilderContext;

#[injectable]
#[async_trait]
pub trait SchemaBuilderContextManager: Send + Sync + Lifecycle {
    /// Returns the context to build a new schema.
    fn get_schema_builder_context(&self) -> SchemaBuilderContext;
}
