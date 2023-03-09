use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::model::EntityInstance;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::PropertyTypeDefinition;

#[allow(dead_code)]
pub struct EntityInstanceBuilder {
    ty: EntityTypeId,
    id: Option<Uuid>,
    properties: HashMap<String, Value>,
}

#[allow(dead_code)]
impl EntityInstanceBuilder {
    pub fn new<ET: Into<EntityTypeId>>(ty: ET) -> EntityInstanceBuilder {
        EntityInstanceBuilder {
            ty: ty.into(),
            id: None,
            properties: HashMap::new(),
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> EntityInstanceBuilder {
        EntityInstanceBuilder::new(EntityTypeId::new_from_type(namespace, type_name))
    }

    pub fn id(&mut self, id: Uuid) -> &mut EntityInstanceBuilder {
        self.id = Some(id);
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut EntityInstanceBuilder {
        self.properties.insert(property_name.into(), value);
        self
    }

    pub fn property_with_default(&mut self, property: &dyn PropertyTypeDefinition) -> &mut EntityInstanceBuilder {
        self.properties.insert(property.property_name().into(), property.default_value());
        self
    }

    pub fn build(&self) -> EntityInstance {
        EntityInstance::new(self.ty.clone(), self.id.unwrap_or_else(Uuid::new_v4), self.properties.clone())
    }
}

impl From<EntityType> for EntityInstanceBuilder {
    fn from(entity_type: EntityType) -> Self {
        let mut builder = EntityInstanceBuilder {
            ty: entity_type.ty.clone(),
            id: None,
            properties: HashMap::new(),
        };
        for property_type in entity_type.properties {
            builder.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        builder
    }
}
