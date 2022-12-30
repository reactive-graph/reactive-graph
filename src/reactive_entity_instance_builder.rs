use std::sync::Arc;

use serde_json::Value;
use uuid::Uuid;

use crate::model::ComponentContainer;
use crate::model::ComponentTypeId;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::model::PropertyTypeDefinition;
use crate::model::ReactiveEntityInstance;
use crate::EntityInstanceBuilder;

#[allow(dead_code)]
pub struct ReactiveEntityInstanceBuilder {
    ty: EntityTypeId,
    components: Vec<ComponentTypeId>,
    builder: EntityInstanceBuilder,
}

#[allow(dead_code)]
impl ReactiveEntityInstanceBuilder {
    pub fn new<ET: Into<EntityTypeId>>(ty: ET) -> ReactiveEntityInstanceBuilder {
        let ty = ty.into();
        ReactiveEntityInstanceBuilder {
            ty: ty.clone(),
            components: Vec::new(),
            builder: EntityInstanceBuilder::new(ty),
        }
    }

    pub fn new_from_type<S: Into<String>>(namespace: S, type_name: S) -> ReactiveEntityInstanceBuilder {
        ReactiveEntityInstanceBuilder::new(EntityTypeId::new_from_type(namespace, type_name))
    }

    pub fn id(&mut self, id: Uuid) -> &mut ReactiveEntityInstanceBuilder {
        self.builder.id(id);
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, value: Value) -> &mut ReactiveEntityInstanceBuilder {
        self.builder.property(property_name.into(), value);
        self
    }

    pub fn property_with_default(&mut self, property: &dyn PropertyTypeDefinition) -> &mut ReactiveEntityInstanceBuilder {
        self.builder.property_with_default(property);
        self
    }

    pub fn set_properties_defaults(&mut self, entity_type: EntityType) -> &mut ReactiveEntityInstanceBuilder {
        for property_type in entity_type.properties {
            self.property(property_type.name.clone(), property_type.data_type.default_value());
        }
        self
    }

    pub fn component<C: Into<ComponentTypeId>>(&mut self, ty: C) -> &mut ReactiveEntityInstanceBuilder {
        self.components.push(ty.into());
        self
    }

    pub fn build(&self) -> Arc<ReactiveEntityInstance> {
        let entity_instance = ReactiveEntityInstance::from(self.builder.build());
        for component in self.components.iter() {
            entity_instance.add_component(component.clone());
        }
        Arc::new(entity_instance)
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
