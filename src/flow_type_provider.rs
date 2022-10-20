use crate::model::FlowType;

#[derive(Debug)]
pub enum FlowTypeProviderError {
    InitializationError,
}

pub trait FlowTypeProvider: Send + Sync {
    fn get_flow_types(&self) -> Vec<FlowType>;
}

#[macro_export]
macro_rules! flow_type_provider {
    ($flow_type_provider:expr) => {{
        let flow_type_provider = $flow_type_provider.clone();
        let flow_type_provider: Result<Arc<dyn FlowTypeProvider>, _> = <dyn query_interface::Object>::query_arc(flow_type_provider);
        if flow_type_provider.is_err() {
            return Err(FlowTypeProviderError::InitializationError);
        }
        Ok(flow_type_provider.ok())
    }};
}
