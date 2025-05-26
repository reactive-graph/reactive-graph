use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use reactive_graph_type_system_json_schema;
use reactive_graph_type_system_json_schema::schema_components;
use reactive_graph_type_system_json_schema::schema_entity_types;
use reactive_graph_type_system_json_schema::schema_flow_types;
use reactive_graph_type_system_json_schema::schema_relation_types;
use serde_json::Value;

#[derive(Default)]
pub struct JsonSchemaTypeSystem;

/// Get the JSON schema of the type system (components, entity types, relation types or flow types).
#[Object]
impl JsonSchemaTypeSystem {
    /// Returns the JSON schema for components.
    async fn components(&self, _context: &Context<'_>) -> Result<Value> {
        Ok(schema_components().to_value())
    }

    /// Returns the JSON schema for entity types.
    async fn entities(&self, _context: &Context<'_>) -> Result<Value> {
        Ok(schema_entity_types().to_value())
    }

    /// Returns the JSON schema for relation types.
    async fn relations(&self, _context: &Context<'_>) -> Result<Value> {
        Ok(schema_relation_types().to_value())
    }

    /// Returns the JSON schema for flow types.
    async fn flows(&self, _context: &Context<'_>) -> Result<Value> {
        Ok(schema_flow_types().to_value())
    }
}
