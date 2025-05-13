use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum GraphqlSchemaCommands {
    /// Prints the GraphQL schema of the reactive graph.
    ReactiveGraphSchema,
    /// Prints the GraphQL schema of the dynamic graph.
    DynamicGraphSchema,
    /// Prints the GraphQL schema of the plugin system of the reactive graph.
    ReactiveGraphPluginSchema,
    /// Prints the GraphQL schema of the runtime of the reactive graph.
    ReactiveGraphRuntimeSchema,
}
