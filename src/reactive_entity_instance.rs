use std::sync::Arc;

use dashmap::DashMap;
use dashmap::DashSet;
use indradb::VertexProperties;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::get_namespace_and_type_name;
use crate::Component;
use crate::ComponentContainer;
use crate::EntityInstance;
use crate::PropertyInstanceGetter;
use crate::PropertyInstanceSetter;
use crate::PropertyType;
use crate::ReactiveBehaviourContainer;
use crate::ReactivePropertyContainer;
use crate::ReactivePropertyInstance;

pub struct ReactiveEntityInstance {
    /// The namespace the entity instance belongs to.
    pub namespace: String,

    /// The name of the entity type.
    pub type_name: String,

    /// The unique identifier of the entity instance.
    pub id: Uuid,

    /// An optional description of the entity instance.
    pub description: String,

    /// The reactive properties.
    pub properties: DashMap<String, ReactivePropertyInstance>,

    /// The names of the components which are applied on this entity instance.
    pub components: DashSet<String>,

    /// The names of the behaviours which are applied on this entity instance.
    pub behaviours: DashSet<String>,
}

impl ReactiveEntityInstance {}

impl ReactivePropertyContainer for ReactiveEntityInstance {
    fn tick(&self) {
        for property_instance in &self.properties {
            property_instance.tick();
        }
    }

    fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    fn add_property<S: Into<String>>(&self, name: S, value: Value) {
        let name = name.into();
        if !self.properties.contains_key(&name) {
            let property_instance = ReactivePropertyInstance::new(self.id, name.clone(), value);
            self.properties.insert(name, property_instance);
        }
    }

    fn add_property_by_type(&self, property: &PropertyType) {
        let property_instance = ReactivePropertyInstance::new(self.id, &property.name, property.data_type.default_value());
        self.properties.insert(property.name.clone(), property_instance);
    }

    fn remove_property<S: Into<String>>(&self, name: S) {
        let name = name.into();
        self.properties.retain(|property_name, _| property_name != &name);
    }

    fn observe_with_handle<F>(&self, name: &str, subscriber: F, handle_id: u128)
    where
        F: FnMut(&Value) + 'static,
    {
        if let Some(property_instance) = self.properties.get(name) {
            property_instance.stream.read().unwrap().observe_with_handle(subscriber, handle_id);
        }
    }

    fn remove_observer(&self, name: &str, handle_id: u128) {
        if let Some(property_instance) = self.properties.get(name) {
            property_instance.stream.read().unwrap().remove(handle_id);
        }
    }
}

impl ComponentContainer for ReactiveEntityInstance {
    fn get_components(&self) -> Vec<String> {
        self.components.iter().map(|c| c.key().clone()).collect()
    }

    fn add_component<S: Into<String>>(&self, component: S) {
        self.components.insert(component.into());
    }

    fn add_component_with_properties(&self, component: &Component) {
        self.add_component(&component.name);
        for property_type in component.properties.iter() {
            if !self.properties.contains_key(&property_type.name) {
                self.add_property_by_type(property_type);
            }
        }
    }

    fn remove_component<S: Into<String>>(&self, component: S) {
        self.components.remove(component.into().as_str());
    }

    fn is_a<S: Into<String>>(&self, component: S) -> bool {
        self.components.contains(component.into().as_str())
    }
}

impl ReactiveBehaviourContainer for ReactiveEntityInstance {
    fn add_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.insert(behaviour.into());
    }

    fn remove_behaviour<S: Into<String>>(&self, behaviour: S) {
        self.behaviours.remove(behaviour.into().as_str());
    }

    fn behaves_as<S: Into<String>>(&self, behaviour: S) -> bool {
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
        let (namespace, type_name) = get_namespace_and_type_name(&properties.vertex.t);
        ReactiveEntityInstance {
            namespace,
            type_name,
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
            namespace: instance.namespace.clone(),
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
            namespace: instance.namespace.clone(),
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
