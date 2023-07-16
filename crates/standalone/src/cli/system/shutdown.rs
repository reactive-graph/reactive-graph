use std::sync::Arc;

use crate::client::InexorRgfClient;

pub(crate) async fn shutdown(client: &Arc<InexorRgfClient>) {
    match client.system().shutdown().shutdown().await {
        Ok(true) => println!("Shutting down..."),
        Ok(false) => eprintln!("Shutdown rejected"),
        Err(e) => eprintln!("Shutdown rejected: {}", e),
    }
}
