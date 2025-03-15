use reactive_graph_client as client;

use client::ReactiveGraphClient;
use client::ReactiveGraphClientError;
use client::ReactiveGraphClientExecutionError;
use itertools::Itertools;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::types::component::ComponentTypeIdTableContainer;

#[derive(Debug)]
#[allow(unused)]
enum ListComponentTypeIdsError {
    ReactiveGraphClientError(ReactiveGraphClientError),
    ReactiveGraphClientExecutionError(ReactiveGraphClientExecutionError),
}

/// This simple example shows how to connect to a runtime and list all components.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), ListComponentTypeIdsError> {
    // Connect to localhost:31415 (default port)
    let client = ReactiveGraphClient::new_default().map_err(ListComponentTypeIdsError::ReactiveGraphClientError)?;
    // Use reactive_graph_client to fetch the list of components
    let components = client
        .types()
        .components()
        .get_all_components()
        .await
        .map_err(ListComponentTypeIdsError::ReactiveGraphClientExecutionError)?
        .unwrap_or_default();

    let components: ComponentTypeIdTableContainer = components
        .into_iter()
        // Convert full component into only the type ids
        .map(|component| (&component).into())
        // Sort types by namespace and type name
        .sorted()
        .into();
    // Print as table
    println!("{}", components.table());
    Ok(())
}
