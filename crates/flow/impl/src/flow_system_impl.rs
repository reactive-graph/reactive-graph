use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_flow_api::FlowSystem;
use reactive_graph_flow_api::FlowTypeSystemRegistrator;
use reactive_graph_lifecycle::Lifecycle;

#[derive(Component)]
pub struct FlowSystemImpl {
    flow_type_system_registrator: Arc<dyn FlowTypeSystemRegistrator + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl FlowSystem for FlowSystemImpl {
    fn get_flow_type_system_registrator(&self) -> Arc<dyn FlowTypeSystemRegistrator + Send + Sync> {
        self.flow_type_system_registrator.clone()
    }
}

#[async_trait]
impl Lifecycle for FlowSystemImpl {
    async fn init(&self) {
        self.flow_type_system_registrator.init().await;
    }

    async fn post_init(&self) {
        self.flow_type_system_registrator.post_init().await;
    }

    async fn pre_shutdown(&self) {
        self.flow_type_system_registrator.pre_shutdown().await;
    }

    async fn shutdown(&self) {
        self.flow_type_system_registrator.shutdown().await;
    }
}
