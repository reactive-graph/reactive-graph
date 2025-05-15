use anyhow::Result;
use anyhow::anyhow;
use reactive_graph_runtime_impl::RuntimeBuilder;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use workspace_root::get_workspace_root;

#[derive(Debug, Clone)]
pub enum SchemaTypes {
    ReactiveGraphSchema,
    ReactiveGraphPluginSchema,
    ReactiveGraphRuntimeSchema,
}

impl SchemaTypes {
    pub fn schema_path(&self) -> PathBuf {
        match self {
            SchemaTypes::ReactiveGraphSchema => Path::new("schema/graphql/reactive-graph-schema.graphql"),
            SchemaTypes::ReactiveGraphPluginSchema => Path::new("schema/graphql/reactive-graph-plugin-schema.graphql"),
            SchemaTypes::ReactiveGraphRuntimeSchema => Path::new("schema/graphql/reactive-graph-runtime-schema.graphql"),
        }
        .to_owned()
    }
}

pub fn generate_graphql_schema(schema_type: &SchemaTypes) -> String {
    let runtime = RuntimeBuilder::new().ignore_config_files().get();
    match schema_type {
        SchemaTypes::ReactiveGraphSchema => runtime.get_graphql_schema_manager().get_schema().sdl(),
        SchemaTypes::ReactiveGraphPluginSchema => runtime.get_plugin_schema_manager().get_schema().sdl(),
        SchemaTypes::ReactiveGraphRuntimeSchema => runtime.get_runtime_schema_manager().get_schema().sdl(),
    }
}

pub fn write_graphql_schema(schema_type: SchemaTypes) -> Result<()> {
    let schema_path = get_workspace_root().join(schema_type.schema_path());
    if !schema_path.exists() {
        eprintln!("Schema path doesn't exist: {:?}", schema_path.display());
        exit(1);
    }
    write(schema_path.clone(), generate_graphql_schema(&schema_type))
        .map_err(|_| anyhow!("Failed to write GraphQL schema {:?} to {:?}", schema_type, schema_path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::SchemaTypes::ReactiveGraphPluginSchema;
    use super::SchemaTypes::ReactiveGraphRuntimeSchema;
    use super::SchemaTypes::ReactiveGraphSchema;
    use super::generate_graphql_schema;

    #[test]
    fn test_generate_schema() {
        assert!(generate_graphql_schema(&ReactiveGraphSchema).len() > 0);
        assert!(generate_graphql_schema(&ReactiveGraphPluginSchema).len() > 0);
        assert!(generate_graphql_schema(&ReactiveGraphRuntimeSchema).len() > 0);
    }
}
