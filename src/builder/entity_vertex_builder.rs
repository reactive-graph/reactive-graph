use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::api::{EntityVertexCreationError, EntityVertexManager};

#[allow(dead_code)]
pub struct EntityVertexBuilder {
    type_name: String,
    id: Option<Uuid>,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl EntityVertexBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> EntityVertexBuilder {
        EntityVertexBuilder {
            type_name: type_name.into(),
            id: None,
            properties: HashMap::new(),
        }
    }

    pub fn id<'a>(&'a mut self, id: Uuid) -> &'a mut EntityVertexBuilder {
        self.id = Some(id);
        self
    }

    pub fn property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
        value: Value,
    ) -> &'a mut EntityVertexBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn create<'a>(
        &'a mut self,
        entity_vertex_manager: Arc<dyn EntityVertexManager>,
    ) -> Result<Uuid, EntityVertexCreationError> {
        if self.id.is_some() {
            entity_vertex_manager.create_with_id(
                self.type_name.clone(),
                self.id.unwrap(),
                self.properties.clone(),
            )
        } else {
            entity_vertex_manager.create(self.type_name.clone(), self.properties.clone())
        }
    }
}
