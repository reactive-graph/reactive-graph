use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::model::{EntityInstance, EntityType};

#[allow(dead_code)]
pub struct EntityInstanceBuilder {
    type_name: String,
    id: Option<Uuid>,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl EntityInstanceBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> EntityInstanceBuilder {
        EntityInstanceBuilder {
            type_name: type_name.into(),
            id: None,
            properties: HashMap::new(),
        }
    }

    pub fn id<'a>(&'a mut self, id: Uuid) -> &'a mut EntityInstanceBuilder {
        self.id = Some(id);
        self
    }

    pub fn property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
        value: Value,
    ) -> &'a mut EntityInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn get(&mut self) -> EntityInstance {
        if self.id.is_some() {
            EntityInstance::new(
                self.type_name.clone(),
                self.id.unwrap(),
                self.properties.clone(),
            )
        } else {
            EntityInstance::new(
                self.type_name.clone(),
                Uuid::new_v4(),
                self.properties.clone(),
            )
        }
    }

    // pub fn create<'a>(
    //     &'a mut self,
    //     entity_instance_manager: Arc<dyn EntityInstanceManager>,
    // ) -> Result<Uuid, EntityInstanceCreationError> {
    //     if self.id.is_some() {
    //         entity_instance_manager.create_with_id(
    //             self.type_name.clone(),
    //             self.id.unwrap(),
    //             self.properties.clone(),
    //         )
    //     } else {
    //         entity_instance_manager.create(self.type_name.clone(), self.properties.clone())
    //     }
    // }
}

impl From<EntityType> for EntityInstanceBuilder {
    fn from(entity_type: EntityType) -> Self {
        let mut builder = EntityInstanceBuilder {
            type_name: entity_type.name.clone(),
            id: None,
            properties: HashMap::new(),
        };
        for property_type in entity_type.properties {
            builder.property(
                property_type.name.clone(),
                property_type.data_type.default_value(),
            );
        }
        builder
    }
}
