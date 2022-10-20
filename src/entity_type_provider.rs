use crate::model::EntityType;

#[derive(Debug)]
pub enum EntityTypeProviderError {
    InitializationError,
}

pub trait EntityTypeProvider: Send + Sync {
    fn get_entity_types(&self) -> Vec<EntityType>;
}

#[macro_export]
macro_rules! entity_type_provider {
    ($entity_type_provider:expr) => {{
        let entity_type_provider = $entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> = <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(EntityTypeProviderError::InitializationError);
        }
        Ok(entity_type_provider.ok())
    }};
}
