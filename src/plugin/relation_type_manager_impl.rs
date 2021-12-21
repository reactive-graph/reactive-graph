use crate::model::{Extension, PropertyType, RelationType};
use crate::plugins::relation_type_manager::RelationTypeCreationError;
use crate::plugins::RelationTypeManager;
use std::sync::Arc;

pub struct RelationTypeManagerImpl {
    relation_type_manager: Arc<dyn crate::api::RelationTypeManager>,
}

impl RelationTypeManagerImpl {
    pub fn new(relation_type_manager: Arc<dyn crate::api::RelationTypeManager>) -> Self {
        Self { relation_type_manager }
    }
}
impl RelationTypeManager for RelationTypeManagerImpl {
    fn get_relation_types(&self) -> Vec<RelationType> {
        self.relation_type_manager.get_relation_types()
    }

    fn has(&self, type_name: String) -> bool {
        self.relation_type_manager.has(type_name)
    }

    fn has_starts_with(&self, type_name: String) -> bool {
        self.relation_type_manager.has_starts_with(type_name)
    }

    fn get(&self, type_name: String) -> Option<RelationType> {
        self.relation_type_manager.get(type_name)
    }

    fn get_starts_with(&self, type_name_starts_with: String) -> Option<RelationType> {
        self.relation_type_manager.get_starts_with(type_name_starts_with)
    }

    fn create(
        &self,
        outbound_type: String,
        type_name: String,
        inbound_type: String,
        components: Vec<String>,
        behaviours: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.relation_type_manager
            .create(outbound_type, type_name, inbound_type, components, behaviours, properties, extensions)
    }

    fn delete(&self, type_name: String) {
        self.relation_type_manager.delete(type_name)
    }
}
