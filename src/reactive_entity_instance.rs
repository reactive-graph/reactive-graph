use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use indradb::VertexProperties;
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::{EntityInstance, ReactivePropertyInstance};
use crate::{PropertyInstanceGetter, PropertyInstanceSetter};

pub struct ReactiveEntityInstance {
    pub type_name: String,

    pub id: Uuid,

    pub description: String,

    pub properties: HashMap<String, ReactivePropertyInstance>,
    // TODO: pub components: Vec<String>
    // TODO: pub fn is_a(component: String) -> bool {}
}

impl ReactiveEntityInstance {
    pub fn tick(&self) {
        for (_, property_instance) in self.properties.iter() {
            property_instance.tick();
        }
    }
}

impl From<VertexProperties> for ReactiveEntityInstance {
    fn from(properties: VertexProperties) -> Self {
        let id = properties.vertex.id;
        let instance_properties = properties
            .props
            .iter()
            .map(|named_property| {
                (
                    named_property.name.clone(),
                    ReactivePropertyInstance::new(
                        id,
                        named_property.name.clone(),
                        named_property.value.clone(),
                    ),
                )
            })
            .collect();
        ReactiveEntityInstance {
            type_name: properties.vertex.t.0,
            id,
            description: String::new(),
            properties: instance_properties,
        }
    }
}

impl From<EntityInstance> for ReactiveEntityInstance {
    fn from(instance: EntityInstance) -> Self {
        let properties = instance
            .properties
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    ReactivePropertyInstance::new(instance.id, name.clone(), value.clone()),
                )
            })
            .collect();
        ReactiveEntityInstance {
            type_name: instance.type_name.clone(),
            id: instance.id,
            description: instance.description,
            properties,
        }
    }
}

impl From<Arc<ReactiveEntityInstance>> for EntityInstance {
    fn from(instance: Arc<ReactiveEntityInstance>) -> Self {
        let properties = instance
            .properties
            .iter()
            .map(|(name, property_instance)| {
                (
                    name.clone(),
                    property_instance.value.read().unwrap().deref().clone(),
                )
            })
            .collect();
        EntityInstance {
            type_name: instance.type_name.clone(),
            id: instance.id,
            description: instance.description.clone(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for ReactiveEntityInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(&property_name.into()).map(|p| p.get())
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.properties
            .get(&property_name.into())
            .and_then(|p| p.as_bool())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.properties
            .get(&property_name.into())
            .and_then(|p| p.as_u64())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.properties
            .get(&property_name.into())
            .and_then(|p| p.as_i64())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.properties
            .get(&property_name.into())
            .and_then(|p| p.as_f64())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.properties
            .get(&property_name.into())
            .and_then(|p| p.as_string())
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.properties
            .get(&property_name.into())
            .and_then(|p| p.as_array())
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.properties
            .get(&property_name.into())
            .and_then(|p| p.as_object())
    }
}

impl PropertyInstanceSetter for ReactiveEntityInstance {
    fn set<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set(value);
        }
    }

    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value) {
        if let Some(instance) = self.properties.get(&property_name.into()) {
            instance.set_no_propagate(value);
        }
    }

    // TODO: fn set(&self, Map<String, Value>
    // TODO: Set values transactional: first set all values internally, then send all affected streams
}
