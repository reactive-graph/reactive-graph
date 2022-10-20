use crate::model::Component;

#[derive(Debug)]
pub enum ComponentProviderError {
    InitializationError,
}

pub trait ComponentProvider: Send + Sync {
    fn get_components(&self) -> Vec<Component>;
}

#[macro_export]
macro_rules! component_provider {
    ($component_provider:expr) => {{
        let component_provider = $component_provider.clone();
        let component_provider: Result<Arc<dyn ComponentProvider>, _> = <dyn query_interface::Object>::query_arc(component_provider);
        if component_provider.is_err() {
            return Err(ComponentProviderError::InitializationError);
        }
        Ok(component_provider.ok())
    }};
}
