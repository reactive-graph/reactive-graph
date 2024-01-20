use inexor_rgf_client as client;

use client::InexorRgfClient;
use client::InexorRgfClientError;
use client::InexorRgfClientExecutionError;
use inexor_rgf_client::schema_plugin::Plugin;

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
    let plugins = client.plugins().get_all().await.map_err(SimpleClientError::InexorRgfClientExecutionError)?;
    // Print the list of plugins
    if plugins.len() == 0 {
        println!("No plugins found.");
        return Ok(());
    }
    let table: String = plugins.iter().map(row).collect();
    println!("{}{}{}{}{}", top(), header("Name", "State", "Description"), line(), table, bottom(),);
    Ok(())
}

fn row(plugin: &Plugin) -> String {
    format!("║ {:<40} ║ {:<8} ║ {:<70} ║\n", plugin.short_name, plugin.state, plugin.description)
}

fn header(c1: &str, c2: &str, c3: &str) -> String {
    format!("║{: ^42}║{: ^10}║{: ^72}║\n", c1, c2, c3)
}

fn top() -> String {
    format!("╔{}╦{}╦{}╗\n", "═".repeat(42), "═".repeat(10), "═".repeat(72))
}

fn line() -> String {
    format!("╠{}╬{}╬{}╣\n", "═".repeat(42), "═".repeat(10), "═".repeat(72))
}

fn bottom() -> String {
    format!("╚{}╩{}╩{}╝\n", "═".repeat(42), "═".repeat(10), "═".repeat(72))
}
