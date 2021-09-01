use std::collections::HashMap;

use async_graphql::scalar;
use indradb::VertexProperties;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{MutablePropertyInstanceSetter, PropertyInstanceGetter};

/// Entity instances represents an typed object which contains properties.
///
/// The entity type defines the properties (name, data type and socket type).
///
/// In contrast to the entity type the entity instance stores values in it's
/// properties.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntityInstance {
    /// The name of the entity type.
    #[serde(alias = "type")]
    pub type_name: String,

    /// The unique identifier of the entity instance.
    pub id: Uuid,

    /// The description of the entity instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties of then entity instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Value>,
}
scalar!(EntityInstance);

impl EntityInstance {
    pub fn new<S: Into<String>>(
        type_name: S,
        id: Uuid,
        properties: HashMap<String, Value>,
    ) -> EntityInstance {
        EntityInstance {
            type_name: type_name.into(),
            id,
            description: String::from(""),
            properties,
        }
    }
}

impl From<VertexProperties> for EntityInstance {
    fn from(properties: VertexProperties) -> Self {
        let type_name = properties.vertex.t.0.clone();
        let id = properties.vertex.id.clone();
        let properties: HashMap<String, Value> = properties
            .props
            .iter()
            .map(|p| (p.name.clone(), p.value.clone()))
            .collect();
        EntityInstance {
            type_name,
            id,
            description: String::new(),
            properties,
        }
    }
}

impl PropertyInstanceGetter for EntityInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties
            .get(&property_name.into())
            .and_then(|v| Some(v.clone()))
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
            .and_then(|p| p.as_str().and_then(|s| Some(s.to_string())))
    }
}

impl MutablePropertyInstanceSetter for EntityInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        let property_value = self.properties.get_mut(&property_name.into()).unwrap();
        *property_value = value.clone()
    }
}
