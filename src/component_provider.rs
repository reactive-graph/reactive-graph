use crate::model::Component;

pub trait ComponentProvider: Send + Sync {
    fn get_components(&self) -> Vec<Component>;
}
