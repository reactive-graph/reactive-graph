use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum IntrospectionQueryCommands {
    /// Get the GraphQL schema of the reactive graph.
    #[non_exhaustive]
    ReactiveGraph,
    /// Get the GraphQL schema of the dynamic graph.
    #[non_exhaustive]
    DynamicGraph,
    /// Get the GraphQL schema of the reactive graph runtime.
    #[non_exhaustive]
    ReactiveGraphRuntime,
    /// Get the GraphQL schema of the plugin system of reactive graph.
    #[non_exhaustive]
    ReactiveGraphPlugins,
}
