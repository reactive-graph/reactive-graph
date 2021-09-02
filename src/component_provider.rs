use inexor_rgf_core_model::Component;

pub trait ComponentProvider: Send + Sync {
    fn get_components(&self) -> Vec<Component>;
}
