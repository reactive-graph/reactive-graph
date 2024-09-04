use crate::client::runtime::shutdown::mutations::shutdown::mutations::shutdown;
use std::sync::Arc;

use crate::InexorRgfClient;
use crate::InexorRgfClientExecutionError;

pub struct Shutdown {
    client: Arc<InexorRgfClient>,
}

impl Shutdown {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn shutdown(&self) -> Result<bool, InexorRgfClientExecutionError> {
        self.client.execute_runtime(shutdown(), |data| data.shutdown).await
    }
}
