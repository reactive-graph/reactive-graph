use itertools::Itertools;
use reactive_graph_client as client;

use client::InexorRgfClient;
use client::InexorRgfClientError;
use client::InexorRgfClientExecutionError;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::system::plugin::PluginsTableContainer;

#[derive(Debug)]
enum SimpleClientError {
    InexorRgfClientError(InexorRgfClientError),
    InexorRgfClientExecutionError(InexorRgfClientExecutionError),
}

/// This simple example shows how to connect to a runtime and list all plugins.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), SimpleClientError> {
    // Connect to localhost:31415 (default port)
    // let client = ;
    let plugins: PluginsTableContainer = InexorRgfClient::new_default()
        // Handle connection error
        .map_err(SimpleClientError::InexorRgfClientError)?
        // Use reactive_graph_client to fetch the list of plugins
        .plugins()
        .get_all()
        .await
        // Handle execution error
        .map_err(SimpleClientError::InexorRgfClientExecutionError)?
        .into_iter()
        // Sort alphabetically
        .sorted()
        // Convert into table model and collect into table container
        .into();
    // Print as table
    println!("{}", plugins.table());
    Ok(())
}
