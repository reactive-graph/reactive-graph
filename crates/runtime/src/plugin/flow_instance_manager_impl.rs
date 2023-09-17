use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::api::ReactiveFlowManager;
use crate::model::FlowInstance;
use crate::model::FlowTypeId;
use crate::plugins::FlowInstanceManager;
use crate::reactive::ReactiveFlow;
use crate::rt_api::ReactiveFlowCreationError;

pub struct FlowInstanceManagerImpl {
    reactive_flow_manager: Arc<dyn ReactiveFlowManager>,
}

impl FlowInstanceManagerImpl {
    pub fn new(reactive_flow_manager: Arc<dyn ReactiveFlowManager>) -> Self {
        Self { reactive_flow_manager }
    }
}

impl FlowInstanceManager for FlowInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_flow_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<ReactiveFlow> {
        self.reactive_flow_manager.get(id)
    }

    fn get_by_label(&self, label: &str) -> Option<ReactiveFlow> {
        self.reactive_flow_manager.get_by_label(label)
    }

    fn create(&self, flow_instance: FlowInstance) -> Result<ReactiveFlow, ReactiveFlowCreationError> {
        self.reactive_flow_manager.create(flow_instance)
    }

    fn create_from_type(
        &self,
        ty: &FlowTypeId,
        variables: HashMap<String, Value>,
        properties: HashMap<String, Value>,
    ) -> Result<ReactiveFlow, ReactiveFlowCreationError> {
        self.reactive_flow_manager.create_from_type(ty, variables, properties)
    }

    fn delete(&self, id: Uuid) -> bool {
        self.reactive_flow_manager.delete(id)
    }
}
