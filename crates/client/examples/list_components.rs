use reactive_graph_client as client;

use client::InexorRgfClient;
use client::InexorRgfClientError;
use client::InexorRgfClientExecutionError;
use reactive_graph_graph::Component;
use reactive_graph_graph::NamespacedTypeGetter;

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
    // Print the list of components
    if components.len() == 0 {
        println!("No components found.");
        return Ok(());
    }
    let table: String = components.iter().map(row).collect();
    println!("{}{}{}{}{}", top(), header("Namespace", "Name"), line(), table, bottom(),);
    Ok(())
}

fn row(component: &Component) -> String {
    format!("║ {:<40} ║ {:<40} ║\n", component.namespace(), component.type_name())
}

fn header(c1: &str, c2: &str) -> String {
    format!("║{: ^42}║{: ^42}║\n", c1, c2)
}

fn top() -> String {
    format!("╔{}╦{}╗\n", "═".repeat(42), "═".repeat(42))
}

fn line() -> String {
    format!("╠{}╬{}╣\n", "═".repeat(42), "═".repeat(42))
}

fn bottom() -> String {
    format!("╚{}╩{}╝\n", "═".repeat(42), "═".repeat(42))
}
