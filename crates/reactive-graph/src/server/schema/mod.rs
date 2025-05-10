use crate::server::schema::commands::SchemaCommands;
use reactive_graph_runtime_impl::RuntimeBuilder;
use std::process::exit;

pub mod args;
pub mod commands;

pub async fn print_graphql_schema_and_exit(commands: &SchemaCommands) {
    let runtime = RuntimeBuilder::new().ignore_config_files().get();
    let sdl = match commands {
        SchemaCommands::ReactiveGraphSchema => runtime.get_graphql_schema_manager().get_schema().sdl(),
        SchemaCommands::DynamicGraphSchema => runtime
            .get_dynamic_graph_schema_manager()
            .get_dynamic_schema()
            .await
            .map(|schema| schema.sdl())
            .unwrap_or_default(),
        SchemaCommands::ReactiveGraphPluginSchema => runtime.get_plugin_schema_manager().get_schema().sdl(),
        SchemaCommands::ReactiveGraphRuntimeSchema => runtime.get_runtime_schema_manager().get_schema().sdl(),
    };
    println!("{}", sdl);
    exit(0);
}
