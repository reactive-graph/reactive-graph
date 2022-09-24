use crate::model::FlowType;

#[derive(Debug)]
pub enum FlowTypeProviderError {
    InitializationError,
}

pub trait FlowTypeProvider: Send + Sync {
    fn get_flow_types(&self) -> Vec<FlowType>;
}
