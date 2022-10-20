use crate::model::RelationType;

#[derive(Debug)]
pub enum RelationTypeProviderError {
    InitializationError,
}

pub trait RelationTypeProvider: Send + Sync {
    fn get_relation_types(&self) -> Vec<RelationType>;
}

#[macro_export]
macro_rules! relation_type_provider {
    ($relation_type_provider:expr) => {{
        let relation_type_provider = $relation_type_provider.clone();
        let relation_type_provider: Result<Arc<dyn RelationTypeProvider>, _> = <dyn query_interface::Object>::query_arc(relation_type_provider);
        if relation_type_provider.is_err() {
            return Err(RelationTypeProviderError::InitializationError);
        }
        Ok(relation_type_provider.ok())
    }};
}
