use crate::model::EntityType;

pub trait EntityTypeProvider: Send + Sync {
    fn get_entity_types(&self) -> Vec<EntityType>;
}
