use reactive_graph_client::InexorRgfClient;
use reactive_graph_client::InexorRgfClientError;
use reactive_graph_client::InexorRgfClientExecutionError;
use reactive_graph_table_model::container::TableContainer;
use reactive_graph_table_model::system::instance::InstanceInfos;

#[derive(Debug)]
enum SimpleClientError {
    InexorRgfClientError(InexorRgfClientError),
    InexorRgfClientExecutionError(InexorRgfClientExecutionError),
}

/// This example shows how to connect to a runtime and list all remotes.
///
/// Note: A runtime must run at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), SimpleClientError> {
    // Connect to localhost:31415 (default port)
    let client = InexorRgfClient::new_default().map_err(SimpleClientError::InexorRgfClientError)?;
    // Use reactive_graph_client to fetch the list of plugins
    let remotes = client
        .runtime()
        .remotes()
        .get_all()
        .await
        .map_err(SimpleClientError::InexorRgfClientExecutionError)?;
    // Convert client model into table model
    let remotes = InstanceInfos::from(remotes);
    // Generate table
    let table = remotes.table();
    // Print table
    println!("{}", table);
    Ok(())
}
