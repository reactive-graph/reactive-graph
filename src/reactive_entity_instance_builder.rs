use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::model::EntityType;
use crate::model::EntityTypeType;
use crate::model::ReactiveEntityInstance;
use crate::EntityInstanceBuilder;

#[allow(dead_code)]
pub struct ReactiveEntityInstanceBuilder {
    ty: EntityTypeType,
    builder: EntityInstanceBuilder,
}

#[allow(dead_code)]
impl ReactiveEntityInstanceBuilder {
    pub fn new(ty: EntityTypeType) -> ReactiveEntityInstanceBuilder {
        ReactiveEntityInstanceBuilder {
            ty: ty.clone(),
            builder: EntityInstanceBuilder::new(ty),
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> ReactiveEntityInstanceBuilder {
        ReactiveEntityInstanceBuilder::new(EntityTypeType::new_from_type(namespace, type_name))
    }

    pub fn id(&mut self, id: Uuid) -> &mut ReactiveEntityInstanceBuilder {
        self.builder.id(id);
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut ReactiveEntityInstanceBuilder {
        self.builder.property(property_name.into(), value);
        self
    }

    pub fn build(&self) -> Arc<ReactiveEntityInstance> {
        Arc::new(ReactiveEntityInstance::from(self.builder.build()))
    }
}

impl From<EntityType> for ReactiveEntityInstanceBuilder {
    fn from(entity_type: EntityType) -> Self {
        let mut builder = ReactiveEntityInstanceBuilder::new(entity_type.ty.clone());
        for property_type in entity_type.properties {
            builder.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        builder
    }
}
