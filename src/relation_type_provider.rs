use crate::model::RelationType;

pub trait RelationTypeProvider: Send + Sync {
    fn get_relation_types(&self) -> Vec<RelationType>;
}
