use crate::api::ReactiveFlowManager;
use crate::model::{Flow, ReactiveFlow};
use crate::plugins::flow_manager::FlowCreationError;
use crate::plugins::FlowManager;
use std::sync::Arc;
use uuid::Uuid;

pub struct FlowManagerImpl {
    reactive_flow_manager: Arc<dyn ReactiveFlowManager>,
}

impl FlowManagerImpl {
    pub fn new(reactive_flow_manager: Arc<dyn ReactiveFlowManager>) -> Self {
        Self { reactive_flow_manager }
    }
}
impl FlowManager for FlowManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_flow_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlow>> {
        self.reactive_flow_manager.get(id)
    }

    fn create(&self, flow: Flow) -> Result<Arc<ReactiveFlow>, FlowCreationError> {
        let reactive_flow = self.reactive_flow_manager.create(flow);
        match reactive_flow {
            Ok(reactive_flow) => Ok(reactive_flow),
            Err(_) => {
                return Err(FlowCreationError::Failed);
            }
        }
    }

    fn delete(&self, id: Uuid) {
        self.reactive_flow_manager.delete(id);
    }
}
