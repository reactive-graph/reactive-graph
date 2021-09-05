use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;
use log::error;
use serde_json::Value;
use uuid::Uuid;
use waiter_di::*;

use crate::api::{
    EntityInstanceCreationError, EntityInstanceImportError, EntityInstanceManager,
    EntityVertexManager,
};
use crate::model::EntityInstance;

#[component]
pub struct EntityInstanceManagerImpl {
    entity_vertex_manager: Wrc<dyn EntityVertexManager>,
}

#[async_trait]
#[provides]
impl EntityInstanceManager for EntityInstanceManagerImpl {
    fn has(&self, id: Uuid) -> bool {
        self.entity_vertex_manager.has(id)
    }

    fn get(&self, id: Uuid) -> Option<EntityInstance> {
        let properties = self.entity_vertex_manager.get_properties(id);
        if properties.is_some() {
            return Some(EntityInstance::from(properties.unwrap()));
        }
        None
    }

    fn create(
        &self,
        type_name: String,
        properties: HashMap<String, Value, RandomState>,
    ) -> Result<Uuid, EntityInstanceCreationError> {
        let result = self.entity_vertex_manager.create(type_name, properties);
        if result.is_err() {
            return Err(EntityInstanceCreationError::EntityVertexCreationError(
                result.err().unwrap(),
            ));
        }
        return Ok(result.unwrap());
    }

    fn create_with_id(
        &self,
        type_name: String,
        id: Uuid,
        properties: HashMap<String, Value, RandomState>,
    ) -> Result<Uuid, EntityInstanceCreationError> {
        let result = self
            .entity_vertex_manager
            .create_with_id(type_name, id, properties);
        if result.is_err() {
            return Err(EntityInstanceCreationError::EntityVertexCreationError(
                result.err().unwrap(),
            ));
        }
        return Ok(result.unwrap());
    }

    fn create_from_instance(
        &self,
        entity_instance: EntityInstance,
    ) -> Result<Uuid, EntityInstanceCreationError> {
        let result = self.entity_vertex_manager.create_with_id(
            entity_instance.type_name.clone(),
            entity_instance.id,
            entity_instance.properties.clone(),
        );
        if result.is_err() {
            return Err(EntityInstanceCreationError::EntityVertexCreationError(
                result.err().unwrap(),
            ));
        }
        return Ok(result.unwrap());
    }

    fn commit(&self, entity_instance: EntityInstance) {
        self.entity_vertex_manager
            .commit(entity_instance.id, entity_instance.properties.clone());
    }

    fn delete(&self, id: Uuid) {
        self.entity_vertex_manager.delete(id);
    }

    fn import(&self, path: String) -> Result<Uuid, EntityInstanceImportError> {
        let file = File::open(path);
        if file.is_ok() {
            let file = file.unwrap();
            let reader = BufReader::new(file);
            let entity_instance = serde_json::from_reader(reader);
            if entity_instance.is_ok() {
                let entity_instance: EntityInstance = entity_instance.unwrap();
                if !self.has(entity_instance.id) {
                    let result = self.entity_vertex_manager.create_with_id(
                        entity_instance.type_name,
                        entity_instance.id,
                        entity_instance.properties,
                    );
                    if result.is_ok() {
                        return Ok(entity_instance.id);
                    }
                }
                // TODO: Err(EntityInstanceExistsError.into())
            }
            // TODO: Err(EntityInstanceDeserializationError.into())
        }
        Err(EntityInstanceImportError.into())
    }

    fn export(&self, id: Uuid, path: String) {
        let o_entity_instance = self.get(id.clone());
        if o_entity_instance.is_some() {
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &o_entity_instance.unwrap());
                    if result.is_err() {
                        error!(
                            "Failed to export entity instance {} to {}: {}",
                            id,
                            path,
                            result.err().unwrap()
                        );
                    }
                }
                Err(error) => {
                    error!(
                        "Failed to export entity instance {} to {}: {}",
                        id,
                        path,
                        error.to_string()
                    );
                }
            }
        }
    }
}
