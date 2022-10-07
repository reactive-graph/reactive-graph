use std::collections::HashMap;

use indradb::EdgeKey;
use indradb::EdgeProperties;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use uuid::Uuid;

use crate::fully_qualified_identifier;
use crate::get_namespace_and_type_name;
use crate::MutablePropertyInstanceSetter;
use crate::PropertyInstanceGetter;
use crate::NAMESPACE_RELATION_TYPE;

/// Relation instances are edges from an outbound entity instance to an
/// inbound entity instance.
///
/// The relation instance is of a relation type. The relation type defines
/// the entity types of the outbound entity instance and the inbound entity
/// instance. Furthermore the relation type defines which properties
/// (name, data type, socket type) a relation instance have to have.
///
/// In constrast to the relation type, the relation instance stores values/
/// documents in it's properties.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelationInstance {
    /// The namespace the relation instance belongs to.
    pub namespace: String,

    /// The id of the outbound vertex.
    pub outbound_id: Uuid,

    /// The name of the relation type
    #[serde(alias = "type")]
    pub type_name: String,

    /// The id of the inbound vertex.
    pub inbound_id: Uuid,

    /// Textual description of the relation instance.
    #[serde(default = "String::new")]
    pub description: String,

    /// The properties of then relation instance.
    ///
    /// Each property is represented by it's name (String) and it's value. The value is
    /// a representation of a JSON. Therefore the value can be boolean, number, string,
    /// array or an object. For more information about the data types please look at
    /// https://docs.serde.rs/serde_json/value/enum.Value.html
    #[serde(default = "HashMap::new")]
    pub properties: HashMap<String, Value>,
}

impl RelationInstance {
    /// Constructs a new relation instance with the given outbound_id, type, inbound_id and properties
    pub fn new<S: Into<String>>(namespace: S, outbound_id: Uuid, type_name: S, inbound_id: Uuid, properties: HashMap<String, Value>) -> RelationInstance {
        RelationInstance {
            namespace: namespace.into(),
            outbound_id,
            type_name: type_name.into(),
            inbound_id,
            description: String::new(),
            properties,
        }
    }

    /// Constructs a new relation instance with the given outbound_id, type, inbound_id but without properties
    pub fn new_without_properties<S: Into<String>>(namespace: S, outbound_id: Uuid, type_name: S, inbound_id: Uuid) -> RelationInstance {
        RelationInstance {
            namespace: namespace.into(),
            outbound_id,
            type_name: type_name.into(),
            inbound_id,
            description: String::new(),
            properties: HashMap::new(),
        }
    }

    pub fn get_key(&self) -> EdgeKey {
        let t = fully_qualified_identifier(&self.namespace, &self.type_name, &NAMESPACE_RELATION_TYPE);
        EdgeKey::new(self.outbound_id, t, self.inbound_id)
    }
}

impl From<EdgeProperties> for RelationInstance {
    fn from(properties: EdgeProperties) -> Self {
        let (namespace, type_name) = get_namespace_and_type_name(&properties.edge.key.t);
        RelationInstance {
            namespace,
            outbound_id: properties.edge.key.outbound_id,
            type_name,
            inbound_id: properties.edge.key.inbound_id,
            description: String::new(),
            properties: properties.props.iter().map(|p| (p.name.to_string(), p.value.clone())).collect(),
        }
    }
}

impl PropertyInstanceGetter for RelationInstance {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.properties.get(&property_name.into()).cloned()
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
        self.properties.get(&property_name.into()).and_then(|p| p.as_str().map(|s| s.to_string()))
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_array().map(Vec::clone))
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.properties.get(&property_name.into()).and_then(|p| p.as_object()).map(Map::clone)
    }
}

impl MutablePropertyInstanceSetter for RelationInstance {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        if let Some(property_value) = self.properties.get_mut(&property_name.into()) {
            *property_value = value
        }
    }
}
