use crate::server::graphql_schema::commands::GraphqlSchemaCommands;
use reactive_graph_runtime_impl::RuntimeBuilder;
use std::process::exit;

pub mod args;
pub mod commands;

pub async fn print_graphql_schema_and_exit(commands: &GraphqlSchemaCommands) {
    let runtime = RuntimeBuilder::new().ignore_config_files().get();
    let sdl = match commands {
        GraphqlSchemaCommands::ReactiveGraphSchema => runtime.get_graphql_schema_manager().get_schema().sdl(),
        GraphqlSchemaCommands::DynamicGraphSchema => runtime
            .get_dynamic_graph_schema_manager()
            .get_dynamic_schema()
            .await
            .map(|schema| schema.sdl())
            .unwrap_or_default(),
        GraphqlSchemaCommands::ReactiveGraphPluginSchema => runtime.get_plugin_schema_manager().get_schema().sdl(),
        GraphqlSchemaCommands::ReactiveGraphRuntimeSchema => runtime.get_runtime_schema_manager().get_schema().sdl(),
    };
    println!("{}", sdl);
    exit(0);
}
