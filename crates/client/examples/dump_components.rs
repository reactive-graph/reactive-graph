use reactive_graph_client as client;

use client::ReactiveGraphClient;
use client::ReactiveGraphClientError;
use client::ReactiveGraphClientExecutionError;

#[derive(Debug)]
enum DumpComponentsError {
    ReactiveGraphClient(ReactiveGraphClientError),
    ReactiveGraphClientExecution(ReactiveGraphClientExecutionError),
    Serialization(serde_json::Error),
}

/// This simple example shows how to connect to a runtime and dump a JSON with all components.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), DumpComponentsError> {
    // Connect to localhost:31415 (default port)
    let client = ReactiveGraphClient::new_default().map_err(DumpComponentsError::ReactiveGraphClient)?;
    // Use reactive_graph_client to fetch the list of components
    let components = client
        .types()
        .components()
        .get_all_components()
        .await
        .map_err(DumpComponentsError::ReactiveGraphClientExecution)?
        .unwrap_or_default();
    if components.len() == 0 {
        println!("No components found.");
        return Ok(());
    }
    let json = serde_json::to_string_pretty(&components).map_err(DumpComponentsError::Serialization)?;
    println!("{}", json);
    Ok(())
}
