use reactive_graph_client as client;

use client::InexorRgfClient;
use client::InexorRgfClientError;
use client::InexorRgfClientExecutionError;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::types::component::ComponentTableContainer;

#[derive(Debug)]
enum SimpleClientError {
    InexorRgfClientError(InexorRgfClientError),
    InexorRgfClientExecutionError(InexorRgfClientExecutionError),
}

/// This simple example shows how to connect to a runtime and list all components.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), SimpleClientError> {
    // Connect to localhost:31415 (default port)
    let client = InexorRgfClient::new_default().map_err(SimpleClientError::InexorRgfClientError)?;
    // Use reactive_graph_client to fetch the list of components
    let components = client
        .types()
        .components()
        .get_all_components()
        .await
        .map_err(SimpleClientError::InexorRgfClientExecutionError)?
        .unwrap_or_default();
    // Convert client model into table model
    let components = ComponentTableContainer::from(components);
    // Generate table
    let table = components.table();
    // Print table
    println!("{}", table);
    Ok(())
}
