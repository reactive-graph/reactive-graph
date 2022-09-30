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

    fn get_relation_types_by_namespace(&self, namespace: &str) -> Vec<RelationType> {
        self.relation_type_manager.get_relation_types_by_namespace(namespace)
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
        namespace: &str,
        outbound_type: &str,
        type_name: &str,
        inbound_type: &str,
        description: &str,
        components: Vec<String>,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.relation_type_manager
            .create(namespace, outbound_type, type_name, inbound_type, description, components, properties, extensions)
    }

    fn add_component(&self, name: &str, component_name: &str) {
        let _ = self.relation_type_manager.add_component(name, component_name);
    }

    fn remove_component(&self, name: &str, component_name: &str) {
        self.relation_type_manager.remove_component(name, component_name);
    }

    fn add_property(&self, name: &str, property: PropertyType) {
        let _ = self.relation_type_manager.add_property(name, property);
    }

    fn remove_property(&self, name: &str, property_name: &str) {
        self.relation_type_manager.remove_property(name, property_name)
    }

    fn add_extension(&self, name: &str, extension: Extension) {
        let _ = self.relation_type_manager.add_extension(name, extension);
    }

    fn remove_extension(&self, name: &str, extension_name: &str) {
        self.relation_type_manager.remove_extension(name, extension_name)
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
