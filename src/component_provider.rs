use crate::model::Component;

#[derive(Debug)]
pub enum ComponentProviderError {
    InitializationError,
}

pub trait ComponentProvider: Send + Sync {
    fn get_components(&self) -> Vec<Component>;
}
