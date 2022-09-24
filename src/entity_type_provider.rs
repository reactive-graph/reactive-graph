use crate::model::EntityType;

#[derive(Debug)]
pub enum EntityTypeProviderError {
    InitializationError,
}

pub trait EntityTypeProvider: Send + Sync {
    fn get_entity_types(&self) -> Vec<EntityType>;
}
