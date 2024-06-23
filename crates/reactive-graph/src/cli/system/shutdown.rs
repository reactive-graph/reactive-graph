use std::sync::Arc;

use crate::cli::error::CommandError::Rejected;
use crate::cli::result::CommandResult;
use reactive_graph_client::InexorRgfClient;

pub(crate) async fn shutdown(client: &Arc<InexorRgfClient>) -> CommandResult {
    match client.runtime().shutdown().shutdown().await {
        Ok(true) => Ok("Shutting down...".into()),
        Ok(false) => Err(Rejected("Server rejected shutdown".to_string())),
        Err(e) => Err(e.into()),
    }
}
