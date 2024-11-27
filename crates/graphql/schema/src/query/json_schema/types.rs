use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_graph::Component;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::RelationType;
use schemars::schema_for;
use serde_json::Value;

#[derive(Default)]
pub struct JsonSchemaTypeSystem;

/// Get the JSON schema of the type system (components, entity types, relation types or flow types).
#[Object]
impl JsonSchemaTypeSystem {
    /// Returns the JSON schema for components.
    async fn components(&self, _context: &Context<'_>) -> Result<Value> {
        let schema = schema_for!(Component);
        Ok(schema.to_value())
    }

    /// Returns the JSON schema for entity types.
    async fn entities(&self, _context: &Context<'_>) -> Result<Value> {
        let schema = schema_for!(EntityType);
        Ok(schema.to_value())
    }

    /// Returns the JSON schema for relation types.
    async fn relations(&self, _context: &Context<'_>) -> Result<Value> {
        let schema = schema_for!(RelationType);
        Ok(schema.to_value())
    }

    /// Returns the JSON schema for flow types.
    async fn flows(&self, _context: &Context<'_>) -> Result<Value> {
        let schema = schema_for!(FlowType);
        Ok(schema.to_value())
    }
}
