use itertools::Itertools;
use reactive_graph_client as client;

use client::ReactiveGraphClient;
use client::ReactiveGraphClientError;
use client::ReactiveGraphClientExecutionError;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::system::plugin::PluginsTableContainer;

#[derive(Debug)]
enum ListPluginsError {
    ReactiveGraphClientError(ReactiveGraphClientError),
    ReactiveGraphClientExecutionError(ReactiveGraphClientExecutionError),
}

/// This simple example shows how to connect to a runtime and list all plugins.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), ListPluginsError> {
    // Connect to localhost:31415 (default port)
    // let client = ;
    let plugins: PluginsTableContainer = ReactiveGraphClient::new_default()
        // Handle connection error
        .map_err(ListPluginsError::ReactiveGraphClientError)?
        // Use reactive_graph_client to fetch the list of plugins
        .plugins()
        .get_all()
        .await
        // Handle execution error
        .map_err(ListPluginsError::ReactiveGraphClientExecutionError)?
        .into_iter()
        // Sort alphabetically
        .sorted()
        // Convert into table model and collect into table container
        .into();
    // Print as table
    println!("{}", plugins.table());
    Ok(())
}
