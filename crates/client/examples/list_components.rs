use reactive_graph_client as client;

use client::ReactiveGraphClient;
use client::ReactiveGraphClientError;
use client::ReactiveGraphClientExecutionError;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::types::component::ComponentTableContainer;

#[derive(Debug)]
#[allow(unused)]
enum ListComponentsError {
    ReactiveGraphClientError(ReactiveGraphClientError),
    ReactiveGraphClientExecutionError(ReactiveGraphClientExecutionError),
}

/// This simple example shows how to connect to a runtime and list all components.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), ListComponentsError> {
    // Connect to localhost:31415 (default port)
    let client = ReactiveGraphClient::new_default().map_err(ListComponentsError::ReactiveGraphClientError)?;
    // Use reactive_graph_client to fetch the list of components
    let components = client
        .types()
        .components()
        .get_all_components()
        .await
        .map_err(ListComponentsError::ReactiveGraphClientExecutionError)?
        .unwrap_or_default();
    // Convert client model into table model
    let components = ComponentTableContainer::from(components);
    // Generate table
    let table = components.table();
    // Print table
    println!("{}", table);
    Ok(())
}
