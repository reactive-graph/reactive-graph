use crate::model::Extension;
use crate::model::PropertyType;
use crate::model::RelationType;
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

    fn has(&self, type_name: &str) -> bool {
        self.relation_type_manager.has(type_name)
    }

    fn has_starts_with(&self, type_name: &str) -> bool {
        self.relation_type_manager.has_starts_with(type_name)
    }

    fn get(&self, type_name: &str) -> Option<RelationType> {
        self.relation_type_manager.get(type_name)
    }

    fn get_starts_with(&self, type_name_starts_with: &str) -> Option<RelationType> {
        self.relation_type_manager.get_starts_with(type_name_starts_with)
    }

    fn find(&self, search: &str) -> Vec<RelationType> {
        self.relation_type_manager.find(search)
    }

    fn count(&self) -> usize {
        self.relation_type_manager.count()
    }

    fn create(
        &self,
        outbound_type: String,
        type_name: String,
        inbound_type: String,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.relation_type_manager
            .create(outbound_type, type_name, inbound_type, components, properties, extensions)
    }

    fn delete(&self, type_name: &str) {
        self.relation_type_manager.delete(type_name)
    }

    fn import(&self, path: &str) {
        let _result = self.relation_type_manager.import(path);
    }

    fn export(&self, type_name: &str, path: &str) {
        self.relation_type_manager.export(type_name, path)
    }
}
