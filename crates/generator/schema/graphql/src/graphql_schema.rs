use anyhow::anyhow;
use async_graphql::dynamic::SchemaError;
use reactive_graph_runtime_impl::RuntimeBuilder;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use thiserror::Error;
use workspace_root::get_workspace_root;

#[derive(Debug, Error)]
pub enum SchemaGenerationError {
    #[error("Failed to generate Dynamic Graph Schema: {0}")]
    DynamicSchemaError(#[from] SchemaError),
}

#[derive(Debug, Clone)]
pub enum GraphQLSchemaTypes {
    DynamicGraphSchema,
    ReactiveGraphSchema,
    ReactiveGraphPluginSchema,
    ReactiveGraphRuntimeSchema,
}

impl GraphQLSchemaTypes {
    pub fn schema_path(&self) -> PathBuf {
        match self {
            GraphQLSchemaTypes::DynamicGraphSchema => Path::new("schema/graphql/dynamic-graph-schema.graphql"),
            GraphQLSchemaTypes::ReactiveGraphSchema => Path::new("schema/graphql/reactive-graph-schema.graphql"),
            GraphQLSchemaTypes::ReactiveGraphPluginSchema => Path::new("schema/graphql/reactive-graph-plugin-schema.graphql"),
            GraphQLSchemaTypes::ReactiveGraphRuntimeSchema => Path::new("schema/graphql/reactive-graph-runtime-schema.graphql"),
        }
        .to_owned()
    }
}

pub fn generate_graphql_schema(schema_type: &GraphQLSchemaTypes) -> Result<String, SchemaGenerationError> {
    let runtime = RuntimeBuilder::new().ignore_config_files().get();
    let sdl = match schema_type {
        GraphQLSchemaTypes::DynamicGraphSchema => runtime.get_dynamic_graph_schema_builder().build_dynamic_schema()?.sdl(),
        GraphQLSchemaTypes::ReactiveGraphSchema => runtime.get_graphql_schema_manager().get_schema().sdl(),
        GraphQLSchemaTypes::ReactiveGraphPluginSchema => runtime.get_plugin_schema_manager().get_schema().sdl(),
        GraphQLSchemaTypes::ReactiveGraphRuntimeSchema => runtime.get_runtime_schema_manager().get_schema().sdl(),
    };
    Ok(sdl)
}

pub fn write_graphql_schema(schema_type: GraphQLSchemaTypes) -> anyhow::Result<()> {
    let schema_path = get_workspace_root().join(schema_type.schema_path());
    if !schema_path.exists() {
        eprintln!("Schema path doesn't exist: {:?}", schema_path.display());
        exit(1);
    }
    let sdl = generate_graphql_schema(&schema_type).map_err(|e| anyhow!("Failed to generate GraphQL schema {:?}:  {:?}", schema_type, e))?;
    write(schema_path.clone(), sdl).map_err(|_| anyhow!("Failed to write GraphQL schema {:?} to {:?}", schema_type, schema_path))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::GraphQLSchemaTypes::ReactiveGraphPluginSchema;
    use super::GraphQLSchemaTypes::ReactiveGraphRuntimeSchema;
    use super::GraphQLSchemaTypes::ReactiveGraphSchema;
    use super::generate_graphql_schema;

    #[test]
    fn test_generate_graphql_schemas() {
        assert!(
            generate_graphql_schema(&ReactiveGraphSchema)
                .expect("Failed to generate Reactive Graph GraphQL Schema")
                .len()
                > 0
        );
        assert!(
            generate_graphql_schema(&ReactiveGraphPluginSchema)
                .expect("Failed to generate Reactive Graph Plugin GraphQL Schema")
                .len()
                > 0
        );
        assert!(
            generate_graphql_schema(&ReactiveGraphRuntimeSchema)
                .expect("Failed to generate Reactive Graph Runtime GraphQL Schema")
                .len()
                > 0
        );
    }
}
