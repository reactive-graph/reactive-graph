use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use log::error;
use log::warn;
use wildmatch::WildMatch;

use crate::api::flow_type_manager::FlowTypeImportError;
use crate::api::flow_type_manager::FlowTypeManager;
use crate::api::EntityTypeManager;
use crate::api::Lifecycle;
use crate::api::RelationTypeManager;
use crate::api::SystemEvent;
use crate::api::SystemEventManager;
use crate::di::component;
use crate::di::provides;
use crate::di::wrapper;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::EntityInstance;
use crate::model::Extension;
use crate::model::FlowType;
use crate::model::PropertyType;
use crate::model::RelationInstance;
use crate::plugins::FlowTypeProvider;

#[wrapper]
pub struct FlowTypesStorage(RwLock<Vec<FlowType>>);

#[provides]
fn create_flow_types_storage() -> FlowTypesStorage {
    FlowTypesStorage(RwLock::new(Vec::new()))
}

#[component]
pub struct FlowTypeManagerImpl {
    event_manager: Wrc<dyn SystemEventManager>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,

    flow_types: FlowTypesStorage,
}

impl FlowTypeManagerImpl {}

#[async_trait]
#[provides]
impl FlowTypeManager for FlowTypeManagerImpl {
    fn register(&self, flow_type: FlowType) -> FlowType {
        // Check that the entity types of every declared entity instance exists
        for entity_type_name in flow_type.uses_entity_types() {
            if !self.entity_type_manager.has(&entity_type_name) {
                warn!("Flow type {} not fully initialized: No entity type named {}", flow_type.name.clone(), entity_type_name);
            }
        }
        // Check that the relation type of every declared relation instance exists
        for relation_type_name in flow_type.uses_relation_types() {
            if !self.relation_type_manager.has(&relation_type_name) {
                warn!("Flow type {} not fully initialized: No relation type named {}", flow_type.name.clone(), relation_type_name);
            }
        }
        // TODO: Check that entity instances referenced by a relation instance exists
        // TODO: Check that relation instances outbound entity has correct entity_type
        // TODO: Check that relation instances inbound entity has correct entity_type
        self.flow_types.0.write().unwrap().push(flow_type.clone());
        debug!("Registered flow type {}", flow_type.name);
        self.event_manager.emit_event(SystemEvent::FlowTypeCreated(flow_type.name.clone()));
        flow_type
    }

    fn get_flow_types(&self) -> Vec<FlowType> {
        self.flow_types.0.read().unwrap().to_vec()
    }

    fn has(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: &str) -> Option<FlowType> {
        self.flow_types.0.read().unwrap().iter().find(|flow_type| &flow_type.name == name).cloned()
    }

    fn find(&self, search: &str) -> Vec<FlowType> {
        let matcher = WildMatch::new(search);
        self.flow_types
            .0
            .read()
            .unwrap()
            .iter()
            .filter(|flow_type| matcher.matches(flow_type.name.as_str()))
            .cloned()
            .collect()
    }

    fn count(&self) -> usize {
        self.flow_types.0.read().unwrap().len()
    }

    fn create(
        &self,
        type_name: String,
        name: String,
        namespace: String,
        entity_instances: Vec<EntityInstance>,
        relation_instances: Vec<RelationInstance>,
        variables: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) {
        self.register(FlowType::new(
            type_name,
            name,
            namespace,
            String::new(),
            entity_instances.to_vec(),
            relation_instances.to_vec(),
            variables.to_vec(),
            extensions.to_vec(),
        ));
    }

    fn delete(&self, name: &str) {
        let event = SystemEvent::FlowTypeDeleted(name.to_string());
        self.flow_types.0.write().unwrap().retain(|flow_type| flow_type.name != name);
        self.event_manager.emit_event(event);
    }

    fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let flow_type: FlowType = serde_json::from_reader(reader)?;
        self.register(flow_type.clone());
        Ok(flow_type)
    }

    fn export(&self, name: &str, path: &str) {
        if let Some(flow_type) = self.get(&name) {
            match File::create(path) {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &flow_type);
                    if result.is_err() {
                        error!("Failed to export flow type {} to {}: {}", name, path, result.err().unwrap());
                    }
                }
                Err(error) => error!("Failed to export flow type {} to {}: {}", name, path, error.to_string()),
            }
        }
    }

    fn add_provider(&self, flow_type_provider: Arc<dyn FlowTypeProvider>) {
        for flow_type in flow_type_provider.get_flow_types() {
            self.register(flow_type);
        }
    }
}

impl Lifecycle for FlowTypeManagerImpl {
    fn shutdown(&self) {
        // TODO: remove?
        self.flow_types.0.write().unwrap().clear();
    }
}
