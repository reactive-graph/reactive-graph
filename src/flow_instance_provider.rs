use crate::model::FlowInstance;

#[derive(Debug)]
pub enum FlowInstanceProviderError {
    InitializationError,
}

pub trait FlowInstanceProvider: Send + Sync {
    fn get_flow_instances(&self) -> Vec<FlowInstance>;
}

#[macro_export]
macro_rules! flow_instance_provider {
    ($flow_instance_provider:expr) => {{
        let flow_instance_provider = $flow_instance_provider.clone();
        let flow_instance_provider: Result<Arc<dyn FlowInstanceProvider>, _> = <dyn query_interface::Object>::query_arc(flow_instance_provider);
        if flow_instance_provider.is_err() {
            return Err(FlowInstanceProviderError::InitializationError);
        }
        Ok(flow_instance_provider.ok())
    }};
}
