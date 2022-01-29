use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::error;
use serde_json::Value;
use uuid::Uuid;
use waiter_di::*;

use crate::api::{EntityInstanceManager, RelationEdgeManager, RelationInstanceCreationError, RelationInstanceImportError, RelationInstanceManager};
use crate::model::RelationInstance;

#[component]
pub struct RelationInstanceManagerImpl {
    relation_edge_manager: Wrc<dyn RelationEdgeManager>,

    entity_instance_manager: Wrc<dyn EntityInstanceManager>,
}

#[async_trait]
#[provides]
impl RelationInstanceManager for RelationInstanceManagerImpl {
    fn has(&self, edge_key: EdgeKey) -> bool {
        self.relation_edge_manager.has(edge_key)
    }

    fn get(&self, edge_key: EdgeKey) -> Option<RelationInstance> {
        self.relation_edge_manager.get_properties(edge_key).map(RelationInstance::from)
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<RelationInstance> {
        self.relation_edge_manager
            .get_by_outbound_entity(outbound_entity_id)
            .iter()
            .map(|edge| edge.key.clone())
            .filter_map(|edge_key| self.get(edge_key))
            .collect()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<RelationInstance> {
        self.relation_edge_manager
            .get_by_outbound_entity(inbound_entity_id)
            .iter()
            .map(|edge| edge.key.clone())
            .filter_map(|edge_key| self.get(edge_key))
            .collect()
    }

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>) -> Result<EdgeKey, RelationInstanceCreationError> {
        if self.relation_edge_manager.has(edge_key.clone()) {
            // Edge already exists!
            return Err(RelationInstanceCreationError::EdgeAlreadyExists(edge_key));
        }
        if !self.entity_instance_manager.has(edge_key.outbound_id) {
            // Outbound entity does not exist!
            return Err(RelationInstanceCreationError::MissingOutboundEntityInstance(edge_key.outbound_id));
        }
        if !self.entity_instance_manager.has(edge_key.inbound_id) {
            // Inbound entity does not exist!
            return Err(RelationInstanceCreationError::MissingInboundEntityInstance(edge_key.inbound_id));
        }
        let result = self.relation_edge_manager.create(edge_key, properties);
        if result.is_err() {
            return Err(RelationInstanceCreationError::RelationEdgeCreationError(result.err().unwrap()));
        }
        Ok(result.unwrap())
    }

    fn create_from_instance(&self, relation_instance: RelationInstance) -> Result<EdgeKey, RelationInstanceCreationError> {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return Err(RelationInstanceCreationError::InvalidEdgeKey);
        }
        self.create(edge_key.unwrap(), relation_instance.properties)
    }

    fn commit(&self, relation_instance: RelationInstance) {
        if let Some(edge_key) = relation_instance.get_key() {
            self.relation_edge_manager.commit(edge_key, relation_instance.properties);
        }
    }

    fn delete(&self, edge_key: EdgeKey) -> bool {
        self.relation_edge_manager.delete(edge_key)
    }

    fn import(&self, path: String) -> Result<RelationInstance, RelationInstanceImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_instance: RelationInstance = serde_json::from_reader(reader)?;
        if let Some(edge_key) = relation_instance.get_key() {
            if self.has(edge_key.clone()) {
                return Err(RelationInstanceImportError::RelationAlreadyExists(edge_key));
            }
            self.relation_edge_manager
                .create(edge_key, relation_instance.properties.clone())
                .map(|_| relation_instance)
                .map_err(RelationInstanceImportError::RelationEdgeCreation)
        } else {
            Err(RelationInstanceImportError::InvalidEdgeKey)
        }
    }

    fn export(&self, edge_key: EdgeKey, path: String) {
        let relation_instance = self.get(edge_key);
        if relation_instance.is_some() {
            let relation_instance = relation_instance.unwrap();
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &relation_instance);
                    if result.is_err() {
                        // TODO: implement Display trait for RelationInstance
                        error!(
                            "Failed to export relation instance {} {} {} to {}: {}",
                            relation_instance.outbound_id,
                            relation_instance.type_name.clone(),
                            relation_instance.inbound_id,
                            path,
                            result.err().unwrap()
                        );
                    }
                }
                Err(error) => {
                    // TODO: implement Display trait for RelationInstance
                    error!(
                        "Failed to export relation instance {} {} {} to {}: {}",
                        relation_instance.outbound_id,
                        relation_instance.type_name.clone(),
                        relation_instance.inbound_id,
                        path,
                        error.to_string()
                    );
                }
            }
        }
    }
}
