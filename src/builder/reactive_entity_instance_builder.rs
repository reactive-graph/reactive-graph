use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::api::{ReactiveEntityInstanceCreationError, ReactiveEntityInstanceManager};
use crate::model::EntityType;
use crate::model::ReactiveEntityInstance;

#[allow(dead_code)]
pub struct ReactiveEntityInstanceBuilder {
    type_name: String,
    id: Option<Uuid>,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl ReactiveEntityInstanceBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> ReactiveEntityInstanceBuilder {
        ReactiveEntityInstanceBuilder {
            type_name: type_name.into(),
            id: None,
            properties: HashMap::new(),
        }
    }

    pub fn id<'a>(&'a mut self, id: Uuid) -> &'a mut ReactiveEntityInstanceBuilder {
        self.id = Some(id);
        self
    }

    pub fn property<'a, S: Into<String>>(
        &'a mut self,
        property_name: S,
        value: Value,
    ) -> &'a mut ReactiveEntityInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn create<'a>(
        &'a mut self,
        reactive_entity_instance_manager: Arc<dyn ReactiveEntityInstanceManager>,
    ) -> Result<Arc<ReactiveEntityInstance>, ReactiveEntityInstanceCreationError> {
        if self.id.is_some() {
            reactive_entity_instance_manager.create_with_id(
                self.type_name.clone(),
                self.id.unwrap(),
                self.properties.clone(),
            )
        } else {
            reactive_entity_instance_manager.create(self.type_name.clone(), self.properties.clone())
        }
    }

    // pub fn get(&mut self) -> Arc<ReactiveEntityInstance> {
    //     if self.id.is_some() {
    //         Arc::new(ReactiveEntityInstance::new(self.type_name.clone(), self.id.unwrap(), self.properties.clone()))
    //     } else {
    //         Arc::new(ReactiveEntityInstance::new(self.type_name.clone(), Uuid::new_v4(), self.properties.clone()))
    //     }
    // }
}

impl From<EntityType> for ReactiveEntityInstanceBuilder {
    fn from(entity_type: EntityType) -> Self {
        let mut builder = ReactiveEntityInstanceBuilder {
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
