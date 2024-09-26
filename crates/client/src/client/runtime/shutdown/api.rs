use crate::client::runtime::shutdown::mutations::shutdown::mutations::shutdown;
use std::sync::Arc;

use crate::ReactiveGraphClient;
use crate::ReactiveGraphClientExecutionError;

pub struct Shutdown {
    client: Arc<ReactiveGraphClient>,
}

impl Shutdown {
    pub fn new(client: Arc<ReactiveGraphClient>) -> Self {
        Self { client }
    }

    pub async fn shutdown(&self) -> Result<bool, ReactiveGraphClientExecutionError> {
        self.client.execute_runtime(shutdown(), |data| data.shutdown).await
    }
}
