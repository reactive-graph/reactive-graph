use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_instance_system_json_schema::schema_entity_instances;
use reactive_graph_instance_system_json_schema::schema_flow_instances;
use reactive_graph_instance_system_json_schema::schema_relation_instances;
use serde_json::Value;

#[derive(Default)]
pub struct JsonSchemaInstanceSystem;

/// Get the JSON schema of the instance system (entity instances, relation instances or flow instances).
#[Object]
impl JsonSchemaInstanceSystem {
    /// Returns the JSON schema for entity instances.
    async fn entities(&self, _context: &Context<'_>) -> Result<Value> {
        Ok(schema_entity_instances().to_value())
    }

    /// Returns the JSON schema for relation instances.
    async fn relations(&self, _context: &Context<'_>) -> Result<Value> {
        Ok(schema_relation_instances().to_value())
    }

    /// Returns the JSON schema for flow instances.
    async fn flows(&self, _context: &Context<'_>) -> Result<Value> {
        Ok(schema_flow_instances().to_value())
    }
}
