use inexor_rgf_core_model::Flow;

pub trait FlowProvider: Send + Sync {
    fn get_flows(&self) -> Vec<Flow>;
}
