use crate::model::FlowInstance;

pub trait FlowInstanceProvider: Send + Sync {
    fn get_flow_instances(&self) -> Vec<FlowInstance>;
}
