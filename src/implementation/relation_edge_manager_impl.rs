use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use async_trait::async_trait;
use indradb::{Edge, EdgeKey, EdgeProperties, EdgeQueryExt, SpecificEdgeQuery, SpecificVertexQuery, Transaction, VertexQueryExt};
use serde_json::Value;
use uuid::Uuid;
use waiter_di::*;

use crate::api::{GraphDatabase, RelationEdgeCreationError, RelationEdgeManager, RelationTypeManager};

// This service operates on the graph database.

#[component]
pub struct RelationEdgeManagerImpl {
    graph_database: Wrc<dyn GraphDatabase>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,
}

#[async_trait]
#[provides]
impl RelationEdgeManager for RelationEdgeManagerImpl {
    fn has(&self, edge_key: EdgeKey) -> bool {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let edges = transaction.get_edges(SpecificEdgeQuery::single(edge_key));
            return edges.unwrap().is_empty();
        }
        false
    }

    fn get(&self, edge_key: EdgeKey) -> Option<Edge> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let edges = transaction.get_edges(SpecificEdgeQuery::single(edge_key));
            if edges.is_ok() {
                let edges = edges.unwrap();
                if !edges.is_empty() {
                    return Some(edges.first().unwrap().clone());
                }
            }
        }
        None
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Edge> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let vertex_query = SpecificVertexQuery::single(outbound_entity_id);
            let edge_query = vertex_query.outbound(1000);
            let edges = transaction.get_edges(edge_query);
            if edges.is_ok() {
                return edges.unwrap();
            }
        }
        Vec::new()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Edge> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let vertex_query = SpecificVertexQuery::single(inbound_entity_id);
            let edge_query = vertex_query.inbound(1000);
            let edges = transaction.get_edges(edge_query);
            if edges.is_ok() {
                return edges.unwrap();
            }
        }
        Vec::new()
    }

    fn get_properties(&self, edge_key: EdgeKey) -> Option<EdgeProperties> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let result = transaction.get_all_edge_properties(SpecificEdgeQuery::single(edge_key));
            if result.is_ok() {
                let edge_properties = result.unwrap();
                if edge_properties.is_empty() {
                    // == 1 ?
                    let edge_properties = edge_properties[0].clone();
                    return Some(edge_properties);
                }
            }
        }
        None
    }

    fn create(&self, edge_key: EdgeKey, properties: HashMap<String, Value>) -> Result<EdgeKey, RelationEdgeCreationError> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_err() {
            return Err(RelationEdgeCreationError::NoTransaction);
        }
        let transaction = r_transaction.unwrap();

        let type_name = edge_key.t.0.clone();
        if !self.relation_type_manager.has_starts_with(type_name.clone()) {
            return Err(RelationEdgeCreationError::RelationTypeMissing(type_name).into());
        }
        let relation_type = self.relation_type_manager.get_starts_with(type_name).unwrap();

        let result = transaction.create_edge(&edge_key);
        if result.is_err() {
            // Should not happen when using indradb::InternalMemoryDatastore
            return Err(RelationEdgeCreationError::GraphDatabaseError(result.err().unwrap()).into());
        }
        let edge_query = SpecificEdgeQuery::single(edge_key.clone());
        for property_type in relation_type.properties {
            let property_name = property_type.name;
            let value = properties.get(&*property_name.clone());
            if value.is_none() {
                // Missing required property
                return Err(RelationEdgeCreationError::MissingRequiredProperty(property_name).into());
            }
            let value = value.unwrap();
            let property_query = edge_query.clone().property(property_name);
            let property_result = transaction.set_edge_properties(property_query, value);
            if property_result.is_err() {
                // Should not happen when using indradb::InternalMemoryDatastore
                return Err(RelationEdgeCreationError::GraphDatabaseError(property_result.err().unwrap()).into());
            }
        }
        return Ok(edge_key);
    }

    fn commit(&self, edge_key: EdgeKey, properties: HashMap<String, Value, RandomState>) {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let q = SpecificEdgeQuery::single(edge_key);
            for (property_name, value) in properties {
                let property_query = q.clone().property(property_name);
                let _property_result = transaction.set_edge_properties(property_query, &value);
                // if !property_result.is_ok() {
                //     return Err(EntityVertexCreationError.into());
                // }
            }
        }
    }

    fn delete(&self, edge_key: EdgeKey) -> bool {
        if self.has(edge_key.clone()) {
            let r_transaction = self.graph_database.get_transaction();
            if r_transaction.is_ok() {
                let transaction = r_transaction.unwrap();
                let result = transaction.delete_edges(SpecificEdgeQuery::single(edge_key));
                return match result {
                    Ok(_) => true,
                    Err(_) => false,
                };
            }
        }
        false
    }
}
