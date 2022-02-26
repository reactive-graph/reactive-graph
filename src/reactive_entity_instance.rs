use std::sync::Arc;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::VertexProperties;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::EntityInstance;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::ReactivePropertyInstance;

pub struct ReactiveEntityInstance {
    pub type_name: String,

    pub id: Uuid,

    pub description: String,

    pub properties: DashMap<String, ReactivePropertyInstance>,

    /// The names of the components which are applied on this entity instance.
    pub components: DashSet<String>,

    /// The names of the behaviours which are applied on this entity instance.
    pub behaviours: DashSet<String>,
}

impl ReactiveEntityInstance {
    pub fn tick(&self) {
        for property_instance in &self.properties {
            property_instance.tick();
        }
    }

    pub fn add_property<S: Into<String>>(&self, name: S, value: Value) {
        let name = name.into();
        if !self.properties.contains_key(name.as_str()) {
            let property_instance = ReactivePropertyInstance::new(self.id, name.clone(), value);
            self.properties.insert(name, property_instance);
        }
    }

    pub fn add_component<S: Into<String>>(&self, component: S) {
        self.components.insert(component.into());
    }

    pub fn remove_component<S: Into<String>>(&self, component: S) {
        self.components.remove(component.into().as_str());
    }

    /// Returns true, if the entity instance is composed with the given component.
    pub fn is_a<S: Into<String>>(&self, component: S) -> bool {
        self.components.contains(component.into().as_str())
    }

    pub fn add_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.insert(behaviour.into());
    }

    pub fn remove_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.remove(behaviour.into().as_str());
    }

    /// Returns true, if the entity instance behaves as the given behaviour.
    pub fn behaves_as<S: Into<String>>(&self, behaviour: S) -> bool {
        self.behaviours.contains(behaviour.into().as_str())
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
                    named_property.name.to_string(),
                    ReactivePropertyInstance::new(id, named_property.name.to_string(), named_property.value.clone()),
                )
            })
            .collect();
        ReactiveEntityInstance {
            type_name: properties.vertex.t.to_string(),
            id,
            description: String::new(),
            properties: instance_properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        }
    }
}

impl From<EntityInstance> for ReactiveEntityInstance {
    fn from(instance: EntityInstance) -> Self {
        let properties = instance
            .properties
            .iter()
            .map(|(name, value)| (name.clone(), ReactivePropertyInstance::new(instance.id, name.clone(), value.clone())))
            .collect();
        ReactiveEntityInstance {
            type_name: instance.type_name.clone(),
            id: instance.id,
            description: instance.description,
            properties,
            components: DashSet::new(),
            behaviours: DashSet::new(),
        }
    }
}

impl From<Arc<ReactiveEntityInstance>> for EntityInstance {
    fn from(instance: Arc<ReactiveEntityInstance>) -> Self {
        let properties = instance
            .properties
            .iter()
            .map(|property_instance| (property_instance.key().clone(), property_instance.get()))
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
        self.properties.get(&property_name.into()).and_then(|p| p.as_bool())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_u64())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_i64())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_f64())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_string())
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_array())
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_object())
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
