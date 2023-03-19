use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use async_trait::async_trait;
use indradb::Datastore;
use indradb::Edge;
use indradb::EdgeKey;
use indradb::EdgeProperties;
use indradb::EdgeQueryExt;
use indradb::SpecificEdgeQuery;
use indradb::SpecificVertexQuery;
use indradb::VertexQueryExt;
use serde_json::Value;
use uuid::Uuid;

use crate::api::GraphDatabase;
use crate::api::RelationEdgeCreationError;
use crate::api::RelationEdgeManager;
use crate::api::RelationTypeManager;
use crate::di::*;
use crate::model::property_identifier;
use crate::model::RelationInstanceTypeId;

// This service operates on the graph database.

#[component]
pub struct RelationEdgeManagerImpl {
    graph_database: Wrc<dyn GraphDatabase>,

    relation_type_manager: Wrc<dyn RelationTypeManager>,
}

#[async_trait]
#[provides]
impl RelationEdgeManager for RelationEdgeManagerImpl {
    fn has(&self, edge_key: &EdgeKey) -> bool {
        if let Ok(edges) = self
            .graph_database
            .get_datastore()
            .get_edges(SpecificEdgeQuery::single(edge_key.clone()).into())
        {
            return !edges.is_empty();
        }
        false
    }

    fn get(&self, edge_key: &EdgeKey) -> Option<Edge> {
        if let Ok(edges) = self
            .graph_database
            .get_datastore()
            .get_edges(SpecificEdgeQuery::single(edge_key.clone()).into())
        {
            return edges.first().cloned();
        }
        None
    }

    fn get_by_outbound_entity(&self, outbound_entity_id: Uuid) -> Vec<Edge> {
        if let Ok(edges) = self
            .graph_database
            .get_datastore()
            .get_edges(SpecificVertexQuery::single(outbound_entity_id).outbound().into())
        {
            return edges;
        }
        Vec::new()
    }

    fn get_by_inbound_entity(&self, inbound_entity_id: Uuid) -> Vec<Edge> {
        if let Ok(edges) = self
            .graph_database
            .get_datastore()
            .get_edges(SpecificVertexQuery::single(inbound_entity_id).inbound().into())
        {
            return edges;
        }
        Vec::new()
    }

    fn get_properties(&self, edge_key: &EdgeKey) -> Option<EdgeProperties> {
        if let Ok(edge_properties) = self
            .graph_database
            .get_datastore()
            .get_all_edge_properties(SpecificEdgeQuery::single(edge_key.clone()).into())
        {
            if !edge_properties.is_empty() {
                return Some(edge_properties[0].clone());
            }
        }
        None
    }

    fn create(&self, edge_key: &EdgeKey, properties: HashMap<String, Value>) -> Result<EdgeKey, RelationEdgeCreationError> {
        let ty = RelationInstanceTypeId::try_from(&edge_key.t).map_err(|_| RelationEdgeCreationError::InvalidEdgeKey(edge_key.t.to_string()))?;
        let relation_ty = ty.relation_type_id();
        let relation_type = self
            .relation_type_manager
            .get(&relation_ty)
            .ok_or(RelationEdgeCreationError::RelationTypeMissing(relation_ty.clone()))?;

        let datastore = self.graph_database.get_datastore();
        let _ = datastore.create_edge(edge_key).map_err(|e| RelationEdgeCreationError::GraphDatabaseError(e))?;
        let edge_query = SpecificEdgeQuery::single(edge_key.clone());
        for property_type in relation_type.properties {
            let property_name = property_type.name;
            let value = properties.get(&*property_name.clone());
            if value.is_none() {
                // Missing required property
                return Err(RelationEdgeCreationError::MissingRequiredProperty(property_name));
            }
            let value = value.unwrap();
            let property_query = edge_query.clone().property(property_identifier(&property_name));
            let property_result = datastore.set_edge_properties(property_query, value.clone());
            if property_result.is_err() {
                // Should not happen when using indradb::InternalMemoryDatastore
                return Err(RelationEdgeCreationError::GraphDatabaseError(property_result.err().unwrap()));
            }
        }
        Ok(edge_key.clone())
    }

    fn commit(&self, edge_key: &EdgeKey, properties: HashMap<String, Value, RandomState>) {
        let datastore = self.graph_database.get_datastore();
        for (property_name, value) in properties {
            let _property_result = datastore.set_edge_properties(
                SpecificEdgeQuery::single(edge_key.clone())
                    .clone()
                    .property(property_identifier(&property_name)),
                value,
            );
        }
    }

    fn delete(&self, edge_key: &EdgeKey) -> bool {
        if self.has(edge_key) {
            return self
                .graph_database
                .get_datastore()
                .delete_edges(SpecificEdgeQuery::single(edge_key.clone()).into())
                .is_ok();
        }
        false
    }
}
