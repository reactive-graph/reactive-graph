use std::collections::HashMap;

use async_trait::async_trait;
use indradb::Datastore;
use indradb::SpecificVertexQuery;
use indradb::Vertex;
use indradb::VertexProperties;
use indradb::VertexQueryExt;
use log::debug;
use serde_json::Value;
use uuid::Uuid;

use crate::api::EntityTypeManager;
use crate::api::EntityVertexCreationError;
use crate::api::EntityVertexManager;
use crate::api::GraphDatabase;
use crate::di::*;
use crate::model::property_identifier;
use crate::model::EntityTypeId;
use crate::model::TypeDefinitionGetter;

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
        if let Ok(vertices) = self.graph_database.get_datastore().get_vertices(SpecificVertexQuery::single(id).into()) {
            return !vertices.is_empty();
        }
        false
    }

    fn get(&self, id: Uuid) -> Option<Vertex> {
        if let Ok(vertices) = self.graph_database.get_datastore().get_vertices(SpecificVertexQuery::single(id).into()) {
            return vertices.first().cloned();
        }
        None
    }

    fn get_properties(&self, id: Uuid) -> Option<VertexProperties> {
        if let Ok(vertex_properties) = self
            .graph_database
            .get_datastore()
            .get_all_vertex_properties(SpecificVertexQuery::single(id).into())
        {
            return vertex_properties.first().cloned();
        }
        None
    }

    fn create(&self, ty: &EntityTypeId, properties: HashMap<String, Value>) -> Result<Uuid, EntityVertexCreationError> {
        match self.entity_type_manager.get(ty) {
            Some(entity_type) => {
                // TODO: check if the given properties are suitable for the entity type
                match self.graph_database.get_datastore().create_vertex_from_type(entity_type.type_id()) {
                    Ok(id) => {
                        let q = SpecificVertexQuery::single(id);
                        let datastore = self.graph_database.get_datastore();
                        for (property_name, value) in properties {
                            let property_result = datastore.set_vertex_properties(q.clone().property(property_identifier(&property_name)), value);
                            if property_result.is_err() {
                                // TODO: rollback: remove vertex
                                return Err(EntityVertexCreationError::GraphDatabaseError(property_result.err().unwrap()));
                            }
                        }
                        debug!("Created vertex {}", id);
                        Ok(id)
                    }
                    Err(e) => Err(EntityVertexCreationError::GraphDatabaseError(e)),
                }
            }
            None => Err(EntityVertexCreationError::EntityTypeMissing(ty.clone())),
        }
    }

    fn create_with_id(&self, ty: &EntityTypeId, id: Uuid, properties: HashMap<String, Value>) -> Result<Uuid, EntityVertexCreationError> {
        match self.entity_type_manager.get(ty) {
            Some(entity_type) => {
                match self.graph_database.get_datastore().create_vertex(&Vertex::with_id(id, entity_type.type_id())) {
                    Ok(true) => {
                        let q = SpecificVertexQuery::single(id);
                        let datastore = self.graph_database.get_datastore();
                        for (property_name, value) in properties {
                            let property_result = datastore.set_vertex_properties(q.clone().property(property_identifier(&property_name)), value);
                            if property_result.is_err() {
                                // TODO: rollback: remove vertex
                                return Err(EntityVertexCreationError::GraphDatabaseError(property_result.err().unwrap()));
                            }
                        }
                        debug!("Created vertex with id {}", id);
                        Ok(id)
                    }
                    Ok(false) => Err(EntityVertexCreationError::UuidTaken(id)),
                    Err(e) => Err(EntityVertexCreationError::GraphDatabaseError(e)),
                }
            }
            None => Err(EntityVertexCreationError::EntityTypeMissing(ty.clone())),
        }
    }

    fn commit(&self, id: Uuid, properties: HashMap<String, Value>) {
        let datastore = self.graph_database.get_datastore();
        for (property_name, value) in properties {
            let _property_result = datastore.set_vertex_properties(SpecificVertexQuery::single(id).property(property_identifier(&property_name)), value);
        }
    }

    fn delete(&self, id: Uuid) {
        if self.has(id) {
            let _result = self.graph_database.get_datastore().delete_vertices(SpecificVertexQuery::single(id).into());
        }
    }
}
