use anyhow::Result;
use reactive_graph_utils_schema_generator::SchemaTypes;
use reactive_graph_utils_schema_generator::write_graphql_schema;

fn main() -> Result<()> {
    write_graphql_schema(SchemaTypes::ReactiveGraphSchema)?;
    write_graphql_schema(SchemaTypes::ReactiveGraphPluginSchema)?;
    write_graphql_schema(SchemaTypes::ReactiveGraphRuntimeSchema)?;
    Ok(())
}
