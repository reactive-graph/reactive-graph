use std::collections::HashMap;

use async_trait::async_trait;
use indradb::{SpecificVertexQuery, Transaction, Vertex, VertexProperties, VertexQueryExt};
use log::debug;
use serde_json::Value;
use uuid::Uuid;
use waiter_di::*;

use crate::api::{EntityTypeManager, EntityVertexCreationError, EntityVertexManager, GraphDatabase};

// This service operates on the graph database.

#[component]
pub struct EntityVertexManagerImpl {
    graph_database: Wrc<dyn GraphDatabase>,

    entity_type_manager: Wrc<dyn EntityTypeManager>,
}

#[async_trait]
#[provides]
impl EntityVertexManager for EntityVertexManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        if let Ok(transaction) = self.graph_database.get_transaction() {
            if let Ok(vertices) = transaction.get_vertices(SpecificVertexQuery::single(id)) {
                return !vertices.is_empty();
            }
        }
        false
    }

    fn get(&self, id: Uuid) -> Option<Vertex> {
        if let Ok(transaction) = self.graph_database.get_transaction() {
            if let Ok(vertices) = transaction.get_vertices(SpecificVertexQuery::single(id)) {
                return vertices.first().cloned();
            }
        }
        None
    }

    fn get_properties(&self, id: Uuid) -> Option<VertexProperties> {
        if let Ok(transaction) = self.graph_database.get_transaction() {
            if let Ok(vertex_properties) = transaction.get_all_vertex_properties(SpecificVertexQuery::single(id)) {
                return vertex_properties.first().cloned();
            }
        }
        None
    }

    fn create(&self, type_name: String, properties: HashMap<String, Value>) -> Result<Uuid, EntityVertexCreationError> {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_err() {
            return Err(EntityVertexCreationError::NoTransaction);
        }
        let transaction = r_transaction.unwrap();

        if !self.entity_type_manager.has(type_name.clone()) {
            return Err(EntityVertexCreationError::EntityTypeMissing(type_name));
        }
        let entity_type = self.entity_type_manager.get(type_name).unwrap();

        // TODO: check if the given properties are suitable for the entity type
        let result = transaction.create_vertex_from_type(entity_type.t);
        if result.is_err() {
            return Err(EntityVertexCreationError::GraphDatabaseError(result.err().unwrap()));
        }
        let id = result.unwrap();

        let q = SpecificVertexQuery::single(id);
        for (property_name, value) in properties {
            let property_result = transaction.set_vertex_properties(q.clone().property(property_name), &value);
            if property_result.is_err() {
                // TODO: rollback: remove vertex
                return Err(EntityVertexCreationError::GraphDatabaseError(property_result.err().unwrap()));
            }
        }
        debug!("Created vertex {}", id);
        Ok(id)
    }

    fn create_with_id(&self, type_name: String, id: Uuid, properties: HashMap<String, Value>) -> Result<Uuid, EntityVertexCreationError> {
        if self.has(id) {
            return Err(EntityVertexCreationError::UuidTaken(id));
        }

        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_err() {
            return Err(EntityVertexCreationError::NoTransaction);
        }
        let transaction = r_transaction.unwrap();

        if !self.entity_type_manager.has(type_name.clone()) {
            return Err(EntityVertexCreationError::EntityTypeMissing(type_name));
        }
        let entity_type = self.entity_type_manager.get(type_name).unwrap();

        let result = transaction.create_vertex(&Vertex::with_id(id, entity_type.t));
        if result.is_err() {
            return Err(EntityVertexCreationError::GraphDatabaseError(result.err().unwrap()));
        }
        if !result.unwrap() {
            // UuidTaken, should not happen as checked before
            return Err(EntityVertexCreationError::UuidTaken(id));
        }

        let q = SpecificVertexQuery::single(id);
        for (property_name, value) in properties {
            let property_result = transaction.set_vertex_properties(q.clone().property(property_name), &value);
            if property_result.is_err() {
                // TODO: rollback: remove vertex
                return Err(EntityVertexCreationError::GraphDatabaseError(property_result.err().unwrap()));
            }
        }
        debug!("Created vertex with id {}", id);
        Ok(id)
    }

    fn commit(&self, id: Uuid, properties: HashMap<String, Value>) {
        let r_transaction = self.graph_database.get_transaction();
        if r_transaction.is_ok() {
            let transaction = r_transaction.unwrap();
            let q = SpecificVertexQuery::single(id);
            for (property_name, value) in properties {
                let property_query = q.clone().property(property_name);
                let _property_result = transaction.set_vertex_properties(property_query, &value);
                // if !property_result.is_ok() {
                //     return Err(EntityVertexCreationError.into());
                // }
            }
        }
    }

    fn delete(&self, id: Uuid) {
        if self.has(id) {
            let r_transaction = self.graph_database.get_transaction();
            if r_transaction.is_ok() {
                let transaction = r_transaction.unwrap();
                let _result = transaction.delete_vertices(SpecificVertexQuery::single(id));
            }
        }
    }
}
