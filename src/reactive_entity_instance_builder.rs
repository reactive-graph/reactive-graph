use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::model::EntityType;
use crate::model::ReactiveEntityInstance;
use crate::EntityInstanceBuilder;

#[allow(dead_code)]
pub struct ReactiveEntityInstanceBuilder {
    type_name: String,
    builder: EntityInstanceBuilder,
}

#[allow(dead_code)]
impl ReactiveEntityInstanceBuilder {
    pub fn new<S: Into<String>>(type_name: S) -> ReactiveEntityInstanceBuilder {
        let type_name: String = type_name.into();
        ReactiveEntityInstanceBuilder {
            type_name: type_name.clone(),
            builder: EntityInstanceBuilder::new(type_name),
        }
    }

    pub fn id(&mut self, id: Uuid) -> &mut ReactiveEntityInstanceBuilder {
        self.builder.id(id);
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut ReactiveEntityInstanceBuilder {
        self.builder.property(property_name.into(), value);
        self
    }

    pub fn get(&mut self) -> Arc<ReactiveEntityInstance> {
        Arc::new(ReactiveEntityInstance::from(self.builder.get()))
    }
}

impl From<EntityType> for ReactiveEntityInstanceBuilder {
    fn from(entity_type: EntityType) -> Self {
        let mut builder = ReactiveEntityInstanceBuilder::new(entity_type.name.clone());
        for property_type in entity_type.properties {
            builder.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        builder
    }
}
