use reactive_graph_client::ReactiveGraphClient;
use reactive_graph_client::ReactiveGraphClientError;
use reactive_graph_client::ReactiveGraphClientExecutionError;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::system::instance::InstanceInfos;

#[derive(Debug)]
enum ListRemotesError {
    ReactiveGraphClientError(ReactiveGraphClientError),
    ReactiveGraphClientExecutionError(ReactiveGraphClientExecutionError),
}

/// This example shows how to connect to a runtime and list all remotes.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), ListRemotesError> {
    // Connect to localhost:31415 (default port)
    let client = ReactiveGraphClient::new_default().map_err(ListRemotesError::ReactiveGraphClientError)?;
    // Use reactive_graph_client to fetch the list of plugins
    let remotes = client
        .runtime()
        .remotes()
        .get_all()
        .await
        .map_err(ListRemotesError::ReactiveGraphClientExecutionError)?;
    // Convert client model into table model
    let remotes = InstanceInfos::from(remotes);
    // Generate table
    let table = remotes.table();
    // Print table
    println!("{}", table);
    Ok(())
}
