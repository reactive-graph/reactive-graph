use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use indradb::Type;
use log::{debug, error, warn};
use waiter_di::*;

use crate::api::ComponentManager;
use crate::api::RelationTypeManager;
use crate::api::{EntityTypeManager, Lifecycle, RelationTypeImportError};
use crate::model::{Extension, PropertyType, RelationType};
use crate::plugins::RelationTypeProvider;

#[wrapper]
pub struct RelationTypes(RwLock<std::vec::Vec<RelationType>>);

#[provides]
fn create_external_type_dependency() -> RelationTypes {
    RelationTypes(RwLock::new(std::vec::Vec::new()))
}

#[component]
pub struct RelationTypeManagerImpl {
    component_manager: Wrc<dyn ComponentManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_types: RelationTypes,
}

#[async_trait]
#[provides]
impl RelationTypeManager for RelationTypeManagerImpl {
    fn register(&self, mut relation_type: RelationType) {
        debug!("Registered relation type {}", relation_type.type_name.clone());
        // Construct the type
        relation_type.t = Type::new(relation_type.type_name.clone()).unwrap();
        if relation_type.outbound_type != "*" && !self.entity_type_manager.has(relation_type.outbound_type.clone()) {
            warn!(
                "Relation type {} not initialized: Outbound entity type does not exist {}",
                relation_type.type_name.clone(),
                relation_type.outbound_type.clone()
            );
            // TODO: Result
            return;
        }
        if relation_type.inbound_type != "*" && !self.entity_type_manager.has(relation_type.inbound_type.clone()) {
            warn!(
                "Relation type {} not initialized: Inbound entity type does not exist {}",
                relation_type.type_name.clone(),
                relation_type.inbound_type.clone()
            );
            // TODO: Result
            return;
        }
        for component_name in relation_type.components.iter() {
            match self.component_manager.get(component_name.clone()) {
                Some(component) => relation_type.properties.append(&mut component.properties.to_vec()),
                None => warn!(
                    "Relation type {} not fully initialized: No component named {}",
                    relation_type.type_name.clone(),
                    component_name
                ),
            }
        }

        self.relation_types.0.write().unwrap().push(relation_type);
        // TODO: Result
    }

    fn get_relation_types(&self) -> Vec<RelationType> {
        self.relation_types.0.read().unwrap().to_vec()
    }

    fn has(&self, type_name: String) -> bool {
        self.get(type_name).is_some()
    }

    fn has_starts_with(&self, type_name: String) -> bool {
        self.get_starts_with(type_name).is_some()
    }

    fn get(&self, type_name: String) -> Option<RelationType> {
        self.relation_types
            .0
            .read()
            .unwrap()
            .iter()
            .find(|relation_type| relation_type.type_name == type_name)
            .cloned()
    }

    fn get_starts_with(&self, type_name_starts_with: String) -> Option<RelationType> {
        // Exact match has higher priority
        match self.get(type_name_starts_with.clone()) {
            Some(relation_type) => Some(relation_type),
            None => {
                // Fuzzy match has lower priority
                self.relation_types
                    .0
                    .read()
                    .unwrap()
                    .iter()
                    .find(|relation_type| type_name_starts_with.starts_with(relation_type.type_name.as_str()))
                    .map(|relation_type| {
                        let mut relation_type = relation_type.clone();
                        relation_type.full_name = type_name_starts_with.clone();
                        relation_type
                    })
            }
        }
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
        self.register(RelationType::new(
            outbound_type,
            type_name,
            inbound_type,
            components.to_vec(),
            behaviours.to_vec(),
            properties.to_vec(),
            extensions.to_vec(),
        ));
    }

    fn delete(&self, type_name: String) {
        self.relation_types
            .0
            .write()
            .unwrap()
            .retain(|relation_type| relation_type.type_name != type_name);
    }

    fn import(&self, path: String) -> Result<RelationType, RelationTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_type: RelationType = serde_json::from_reader(reader)?;
        self.register(relation_type.clone());
        Ok(relation_type)
    }

    fn export(&self, type_name: String, path: String) {
        let o_relation_type = self.get(type_name.clone());
        if o_relation_type.is_some() {
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &o_relation_type.unwrap());
                    if result.is_err() {
                        error!("Failed to export relation type {} to {}: {}", type_name, path, result.err().unwrap());
                    }
                }
                Err(error) => {
                    error!("Failed to export relation type {} to {}: {}", type_name, path, error.to_string());
                }
            }
        }
    }

    fn add_provider(&self, relation_type_provider: Arc<dyn RelationTypeProvider>) {
        for relation_type in relation_type_provider.get_relation_types() {
            debug!("Registering relation type: {}", relation_type.type_name);
            self.register(relation_type);
        }
    }
}

impl Lifecycle for RelationTypeManagerImpl {
    fn init(&self) {}

    fn shutdown(&self) {
        // TODO: remove?
        self.relation_types.0.write().unwrap().clear();
    }
}
