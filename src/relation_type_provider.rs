use crate::model::RelationType;

#[derive(Debug)]
pub enum RelationTypeProviderError {
    InitializationError,
}

pub trait RelationTypeProvider: Send + Sync {
    fn get_relation_types(&self) -> Vec<RelationType>;
}
