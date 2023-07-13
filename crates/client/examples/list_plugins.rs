use inexor_rgf_client::InexorRgfClient;
use inexor_rgf_client::InexorRgfClientError;
use inexor_rgf_client::InexorRgfClientExecutionError;

#[derive(Debug)]
enum SimpleClientError {
    InexorRgfClientError(InexorRgfClientError),
    InexorRgfClientExecutionError(InexorRgfClientExecutionError),
}

/// This simple example shows how to connect to a runtime and list all plugins.
///
/// Note: A runtime must running at the default port (31415).
#[tokio::main]
async fn main() -> Result<(), SimpleClientError> {
    // Connect to localhost:31415 (default port)
    let client = InexorRgfClient::new_default().map_err(SimpleClientError::InexorRgfClientError)?;
    // Use inexor_rgf_client to fetch the list of plugins
    let plugins = client
        .system()
        .plugins()
        .get_all()
        .await
        .map_err(SimpleClientError::InexorRgfClientExecutionError)?;
    // Print the list of plugins
    if plugins.len() == 0 {
        println!("No plugins found.");
        return Ok(());
    }
    plugins
        .iter()
        .for_each(|plugin| println!("| {:<50} | {:<70} |", plugin.name, plugin.description));
    Ok(())
}