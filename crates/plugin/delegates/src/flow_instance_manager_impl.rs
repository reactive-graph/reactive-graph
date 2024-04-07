use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use reactive_graph_graph::FlowInstance;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_reactive_service_api::ReactiveFlowCreationError;
use reactive_graph_reactive_service_api::ReactiveFlowManager;

pub struct FlowInstanceManagerDelegate {
    reactive_flow_manager: Arc<dyn ReactiveFlowManager + Send + Sync>,
}

impl FlowInstanceManagerDelegate {
    pub fn new(reactive_flow_manager: Arc<dyn ReactiveFlowManager + Send + Sync>) -> Self {
        Self { reactive_flow_manager }
    }
}

impl reactive_graph_plugin_api::FlowInstanceManager for FlowInstanceManagerDelegate {
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
        self.reactive_flow_manager.create_reactive_flow(flow_instance)
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
