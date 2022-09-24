use crate::model::FlowInstance;

#[derive(Debug)]
pub enum FlowInstanceProviderError {
    InitializationError,
}

pub trait FlowInstanceProvider: Send + Sync {
    fn get_flow_instances(&self) -> Vec<FlowInstance>;
}
