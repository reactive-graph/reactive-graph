use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::EntityType;

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

    pub fn id(&mut self, id: Uuid) -> &mut EntityInstanceBuilder {
        self.id = Some(id);
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut EntityInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn get(&mut self) -> EntityInstance {
        if self.id.is_some() {
            EntityInstance::new(self.type_name.clone(), self.id.unwrap(), self.properties.clone())
        } else {
            EntityInstance::new(self.type_name.clone(), Uuid::new_v4(), self.properties.clone())
        }
    }
}

impl From<EntityType> for EntityInstanceBuilder {
    fn from(entity_type: EntityType) -> Self {
        let mut builder = EntityInstanceBuilder {
            type_name: entity_type.name.clone(),
            id: None,
            properties: HashMap::new(),
        };
        for property_type in entity_type.properties {
            builder.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        builder
    }
}
