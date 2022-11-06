use std::collections::HashMap;
use std::fmt;

use async_trait::async_trait;
use indradb::Vertex;
use indradb::VertexProperties;
use serde_json::Value;
use uuid::Uuid;

use crate::model::EntityTypeId;
use crate::model::TypeDefinitionGetter;

#[derive(Debug)]
pub enum EntityVertexCreationError {
    UuidTaken(Uuid),
    EntityTypeMissing(EntityTypeId),
    GraphDatabaseError(indradb::Error),
}

impl fmt::Display for EntityVertexCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EntityVertexCreationError::UuidTaken(id) => {
                write!(f, "The UUID {} has been already taken!", id)
            }
            EntityVertexCreationError::EntityTypeMissing(ty) => {
                write!(f, "Entity type {} does not exist!", ty)
            }
            EntityVertexCreationError::GraphDatabaseError(error) => write!(f, "Failed to create graph database vertex: {}", error),
        }
    }
}

#[async_trait]
pub trait EntityVertexManager: Send + Sync {
    /// Returns true, if an entity instance vertex exists with the given UUID.
    fn has(&self, id: Uuid) -> bool;

    /// Returns the vertex by UUID.
    fn get(&self, id: Uuid) -> Option<Vertex>;

    /// Returns the vertex properties by UUID. The result contains
    /// the vertex and the type.
    fn get_properties(&self, id: Uuid) -> Option<VertexProperties>;

    /// Creates a new vertex with the given type and the given properties.
    fn create(&self, ty: &EntityTypeId, properties: HashMap<String, Value>) -> Result<Uuid, EntityVertexCreationError>;

    /// Creates a new vertex with the given id, the given type and the given properties.
    /// This is useful for importing an entity instance with the fixed id.
    fn create_with_id(&self, ty: &EntityTypeId, id: Uuid, properties: HashMap<String, Value>) -> Result<Uuid, EntityVertexCreationError>;

    fn commit(&self, id: Uuid, properties: HashMap<String, Value>);

    /// Deletes the vertex with the given id.
    fn delete(&self, id: Uuid);
}
