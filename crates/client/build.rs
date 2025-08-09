use anyhow::Result;
use reactive_graph_generator_schema_graphql::GraphQLSchemaTypes;
use reactive_graph_generator_schema_graphql::write_graphql_schema;

use reactive_graph_generator_schema_json::JsonSchemaTypes;
use reactive_graph_generator_schema_json::write_json_schema;

fn main() -> Result<()> {
    write_graphql_schema(GraphQLSchemaTypes::DynamicGraphSchema)?;
    write_graphql_schema(GraphQLSchemaTypes::ReactiveGraphSchema)?;
    write_graphql_schema(GraphQLSchemaTypes::ReactiveGraphPluginSchema)?;
    write_graphql_schema(GraphQLSchemaTypes::ReactiveGraphRuntimeSchema)?;
    write_json_schema(JsonSchemaTypes::Component)?;
    write_json_schema(JsonSchemaTypes::EntityType)?;
    write_json_schema(JsonSchemaTypes::RelationType)?;
    write_json_schema(JsonSchemaTypes::FlowType)?;
    write_json_schema(JsonSchemaTypes::EntityInstance)?;
    write_json_schema(JsonSchemaTypes::RelationInstance)?;
    write_json_schema(JsonSchemaTypes::FlowInstance)?;
    Ok(())
}
