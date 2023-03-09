use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::api::ReactiveFlowInstanceManager;
use crate::model::FlowInstance;
use crate::model::FlowTypeId;
use crate::model::ReactiveFlowInstance;
use crate::plugins::FlowInstanceCreationError;
use crate::plugins::FlowInstanceManager;

pub struct FlowInstanceManagerImpl {
    reactive_flow_instance_manager: Arc<dyn ReactiveFlowInstanceManager>,
}

impl FlowInstanceManagerImpl {
    pub fn new(reactive_flow_instance_manager: Arc<dyn ReactiveFlowInstanceManager>) -> Self {
        Self {
            reactive_flow_instance_manager,
        }
    }
}

impl FlowInstanceManager for FlowInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.reactive_flow_instance_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<Arc<ReactiveFlowInstance>> {
        self.reactive_flow_instance_manager.get(id)
    }

    fn get_by_label(&self, label: &str) -> Option<Arc<ReactiveFlowInstance>> {
        self.reactive_flow_instance_manager.get_by_label(label)
    }

    fn create(&self, flow_instance: FlowInstance) -> Result<Arc<ReactiveFlowInstance>, FlowInstanceCreationError> {
        let reactive_flow_instance = self.reactive_flow_instance_manager.create(flow_instance);
        match reactive_flow_instance {
            Ok(reactive_flow_instance) => Ok(reactive_flow_instance),
            Err(_) => Err(FlowInstanceCreationError::Failed),
        }
    }

    fn create_from_type(
        &self,
        ty: &FlowTypeId,
        variables: HashMap<String, Value>,
        properties: HashMap<String, Value>,
    ) -> Result<Arc<ReactiveFlowInstance>, FlowInstanceCreationError> {
        match self.reactive_flow_instance_manager.create_from_type(ty, variables, properties) {
            Ok(reactive_flow_instance) => Ok(reactive_flow_instance),
            Err(_) => Err(FlowInstanceCreationError::Failed),
        }
    }

    fn delete(&self, id: Uuid) {
        self.reactive_flow_instance_manager.delete(id);
    }
}
