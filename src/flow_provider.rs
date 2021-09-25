use crate::model::Flow;

pub trait FlowProvider: Send + Sync {
    fn get_flows(&self) -> Vec<Flow>;
}
