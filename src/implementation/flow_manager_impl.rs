use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;
use log::error;
use waiter_di::*;

use crate::api::{EntityInstanceManager, FlowCreationError, FlowImportError, FlowManager, RelationInstanceManager};
use crate::model::Flow;

#[component]
pub struct FlowManagerImpl {
    entity_instance_manager: Wrc<dyn EntityInstanceManager>,

    relation_instance_manager: Wrc<dyn RelationInstanceManager>,
}

#[async_trait]
#[provides]
impl FlowManager for FlowManagerImpl {
    fn create(&self, flow: Flow) -> Result<Flow, FlowCreationError> {
        for entity_instance in flow.entity_instances {
            if !self.entity_instance_manager.has(entity_instance.id) {
                let _result = self.entity_instance_manager.create_from_instance(entity_instance.clone());
            }
        }
        for relation_instance in flow.relation_instances {
            let edge_key = relation_instance.get_key();
            if edge_key.is_some() && !self.relation_instance_manager.has(edge_key.unwrap()) {
                let _result = self.relation_instance_manager.create_from_instance(relation_instance.clone());
            }
        }
        Err(FlowCreationError.into())
    }

    fn commit(&self, flow: Flow) {
        for entity_instance in flow.entity_instances {
            if self.entity_instance_manager.has(entity_instance.id) {
                // The entity instance has been updated
                self.entity_instance_manager.commit(entity_instance.clone());
            } else {
                // The entity instance has been added
                let _result = self.entity_instance_manager.create_from_instance(entity_instance.clone());
            }
            // TODO: what happens with removed entity instances?
        }
        for relation_instance in flow.relation_instances {
            let edge_key = relation_instance.get_key();
            if edge_key.is_some() {
                if self.relation_instance_manager.has(edge_key.unwrap()) {
                    // The relation instance has been updated
                    self.relation_instance_manager.commit(relation_instance.clone());
                } else {
                    // The relation instance has been added
                    let _result = self.relation_instance_manager.create_from_instance(relation_instance.clone());
                }
                // TODO: what happens with removed relation instances?
            }
        }
    }

    fn delete(&self, flow: Flow) {
        // Reverse order: first relations then entities
        for relation_instance in flow.relation_instances {
            let edge_key = relation_instance.get_key();
            if edge_key.is_some() {
                self.relation_instance_manager.delete(edge_key.unwrap());
            }
        }
        for entity_instance in flow.entity_instances {
            self.entity_instance_manager.delete(entity_instance.id);
        }
    }

    fn import(&self, path: String) -> Result<Flow, FlowImportError> {
        let file = File::open(path);
        if file.is_ok() {
            let reader = BufReader::new(file.unwrap());
            let flow = serde_json::from_reader(reader);
            if flow.is_ok() {
                let _result = self.create(flow.unwrap());
            }
        }
        Err(FlowImportError.into())
    }

    fn export(&self, flow: Flow, path: String) {
        let r_file = File::create(path.clone());
        match r_file {
            Ok(file) => {
                let result = serde_json::to_writer_pretty(&file, &flow.clone());
                if result.is_err() {
                    error!("Failed to export flow {} to {}: {}", flow.id, path, result.err().unwrap());
                }
            }
            Err(error) => {
                error!("Failed to export flow {} to {}: {}", flow.id, path, error.to_string());
            }
        }
    }
}
