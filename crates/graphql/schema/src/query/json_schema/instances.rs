use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_graph::EntityInstance;
use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::RelationInstance;
use schemars::schema_for;
use serde_json::Value;

#[derive(Default)]
pub struct JsonSchemaInstanceSystem;

/// Get the JSON schema of the instance system (entity instances, relation instances or flow instances).
#[Object]
impl JsonSchemaInstanceSystem {
    /// Returns the JSON schema for entity instances.
    async fn entities(&self, _context: &Context<'_>) -> Result<Value> {
        let schema = schema_for!(EntityInstance);
        Ok(schema.to_value())
    }

    /// Returns the JSON schema for relation instances.
    async fn relations(&self, _context: &Context<'_>) -> Result<Value> {
        let schema = schema_for!(RelationInstance);
        Ok(schema.to_value())
    }

    /// Returns the JSON schema for flow instances.
    async fn flows(&self, _context: &Context<'_>) -> Result<Value> {
        let schema = schema_for!(FlowInstance);
        Ok(schema.to_value())
    }
}
