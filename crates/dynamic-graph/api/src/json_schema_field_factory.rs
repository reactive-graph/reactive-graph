use async_graphql::dynamic::Field;
use async_trait::async_trait;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

pub const FIELD_JSON_SCHEMA_APPENDIX: &str = "JsonSchema";

pub const FIELD_NAME_JSON_SCHEMA: &str = "_json_schema";

#[injectable]
#[async_trait]
pub trait JsonSchemaFieldFactory: Send + Sync + Lifecycle {
    fn get_json_schema_field(&self, field_name: &str, type_definition: &TypeDefinition) -> Option<Field>;
}
