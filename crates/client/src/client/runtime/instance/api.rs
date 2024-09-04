use std::sync::Arc;

use crate::client::runtime::instance::queries::get_instance_info::queries::get_instance_info;
use crate::InexorRgfClient;
use crate::InexorRgfClientExecutionError;
use reactive_graph_remotes_model::InstanceInfo;

pub struct Instance {
    client: Arc<InexorRgfClient>,
}

impl Instance {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub async fn get_instance_info(&self) -> Result<InstanceInfo, InexorRgfClientExecutionError> {
        self.client.execute_runtime(get_instance_info(), |data| data.instance_info.into()).await
    }
}
