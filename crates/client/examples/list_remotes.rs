use reactive_graph_client::InexorRgfClient;
use reactive_graph_client::InexorRgfClientError;
use reactive_graph_client::InexorRgfClientExecutionError;

#[derive(Debug)]
enum SimpleClientError {
    InexorRgfClientError(InexorRgfClientError),
    InexorRgfClientExecutionError(InexorRgfClientExecutionError),
}

/// This example shows how to connect to a runtime and list all remotes.
///
/// Note: A runtime must running at the default port (31415).
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
    // Print the list of plugins
    if remotes.len() == 0 {
        println!("No remotes found.");
        return Ok(());
    }
    remotes.iter().for_each(|remote| {
        println!(
            "| {:<30} | {:<5} | {:<50} | {:<70} |",
            remote.address.hostname, remote.address.port, remote.name, remote.description
        )
    });
    Ok(())
}
