use std::collections::BTreeMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use dashmap::DashMap;
use dashmap::DashSet;
use schemars::gen::SchemaGenerator;
use schemars::JsonSchema;
use schemars::schema::InstanceType;
use schemars::schema::ObjectValidation;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

use crate::HashableValue;
use crate::MutablePropertyInstanceSetter;
use crate::PropertyInstanceGetter;
use crate::PropertyTypes;

pub struct ContainerPropertyInstance<IdType: Clone> {
    /// Id of the container
    pub id: IdType,

    /// Property name
    pub name: String,

    /// Store the current value
    pub value: Value,
}

impl <IdType: Clone> ContainerPropertyInstance<IdType> {
    pub fn new <N: Into<String>>(id: IdType, name: N, value: Value) -> Self {
        ContainerPropertyInstance {
            id,
            name: name.into(),
            value
        }
    }
}

pub type PropertyNames = DashSet<String>;

/// Container for property instances.
/// PropertyInstances exposes all functionality from the underlying DashMap<String, Value>.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PropertyInstances(DashMap<String, Value>);

impl PropertyInstances {

    /// Creates an empty map of property instances.
    pub fn new() -> Self {
        PropertyInstances(DashMap::new())
    }

    /// Creates property instances from a property type and initializes the values with the
    /// default values of the data type of the property.
    pub fn new_from_property_types_with_defaults(property_types: &PropertyTypes) -> Self {
        let properties = Self::new();
        for property_type in property_types.iter() {
            properties.insert(property_type.key().clone(), property_type.data_type.default_value());
        }
        properties
    }

    /// Add or replace property instance with the given name.
    /// Consumes and returns self (builder style).
    pub fn property<N: Into<String>, V: Into<Value>>(self, name: N, value: V) -> Self {
        self.insert(name.into(), value.into());
        self
    }

    /// Returns all property names.
    pub fn names(&self) -> PropertyNames {
        self.iter().map(|property| property.key().clone()).collect()
    }

    // TODO: pub fn validate(&self) -> Result<(), PropertyInstancesValidationError> {}

    /// Returns a sorted list of property instances.
    pub fn to_map(&self) -> BTreeMap<String, Value> {
        self.0.iter().map(|property| {
            (property.key().clone(), property.value().clone())
        }).collect()
        // let mut tys: HashMap<String, Value> = self.0.iter().map(|ty| ty.clone()).collect();
        // tys.sort();
        // tys
    }
}

impl PropertyInstanceGetter for PropertyInstances {
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value> {
        self.0.get(&property_name.into()).map(|property| property.value().clone())
    }

    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool> {
        self.0.get(&property_name.into()).and_then(|p| p.as_bool())
    }

    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64> {
        self.0.get(&property_name.into()).and_then(|p| p.as_u64())
    }

    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64> {
        self.0.get(&property_name.into()).and_then(|p| p.as_i64())
    }

    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64> {
        self.0.get(&property_name.into()).and_then(|p| p.as_f64())
    }

    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String> {
        self.0.get(&property_name.into()).and_then(|p| p.as_str().map(|s| s.to_string()))
    }

    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>> {
        self.0.get(&property_name.into()).and_then(|p| p.as_array().map(Vec::clone))
    }

    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>> {
        self.0.get(&property_name.into()).and_then(|p| p.as_object().map(Map::clone))
    }
}

impl MutablePropertyInstanceSetter for PropertyInstances {
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value) {
        if let Some(mut property_value) = self.0.get_mut(&property_name.into()) {
            let v = property_value.value_mut();
            *v = value;
        }
    }
}

impl Deref for PropertyInstances {
    type Target = DashMap<String, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PropertyInstances {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for PropertyInstances {
    type Item = (String, Value);
    type IntoIter = dashmap::iter::OwningIter<String, Value>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for PropertyInstances {
    fn eq(&self, other: &Self) -> bool {
        let this = self.to_map();
        let other = other.to_map();
        this.eq(&other)
    }
}

impl Eq for PropertyInstances {}

impl Hash for PropertyInstances {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.iter().for_each(| property | {
            property.key().hash(hasher);
            HashableValue(property.value()).hash(hasher);
        });
    }
}

impl JsonSchema for PropertyInstances {
    fn schema_name() -> String {
        "PropertyInstances".to_owned()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {

        let subschema = gen.subschema_for::<Value>();
        SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                additional_properties: Some(Box::new(subschema)),
                ..Default::default()
            })),
            ..Default::default()
        }
            .into()
    }
}

// impl From<HashMap<String, Value>> for PropertyInstances {
//     fn from(tys: HashMap<String, Value>) -> Self {
//         PropertyInstances(tys.into_iter().collect())
//     }
// }
//
// impl From<PropertyInstances> for HashMap<String, Value> {
//     fn from(tys: PropertyInstances) -> Self {
//         tys.to_map()
//     }
// }

impl From<BTreeMap<String, Value>> for PropertyInstances {
    fn from(tys: BTreeMap<String, Value>) -> Self {
        PropertyInstances(tys.into_iter().collect())
    }
}

impl From<PropertyInstances> for BTreeMap<String, Value> {
    fn from(tys: PropertyInstances) -> Self {
        tys.to_map()
    }
}

impl From<&PropertyInstances> for BTreeMap<String, Value> {
    fn from(tys: &PropertyInstances) -> Self {
        tys.0.iter().map(|ty| {
            (ty.key().clone(), ty.value().clone())
        }).collect()
    }
}

impl From<DashMap<String, Value>> for PropertyInstances {
    fn from(tys: DashMap<String, Value>) -> Self {
        PropertyInstances(tys)
    }
}

impl From<&DashMap<String, Value>> for PropertyInstances {
    fn from(tys: &DashMap<String, Value>) -> Self {
        PropertyInstances(tys.clone())
    }
}

impl From<PropertyInstances> for DashMap<String, Value> {
    fn from(tys: PropertyInstances) -> Self {
        tys.0
    }
}

// impl From<PropertyInstances> for HashMap<String, Value> {
//     fn from(property_instances: PropertyInstances) -> Self {
//         property_instances.0.into_iter().collect::<HashMap<String, Value>>()
//     }
// }

impl FromIterator<(String, Value)> for PropertyInstances {
    fn from_iter<I: IntoIterator<Item=(String, Value)>>(iter: I) -> Self {
        let properties = PropertyInstances::new();
        for (property_name, property_value) in iter {
            properties.insert(property_name, property_value);
        }
        properties
    }
}

#[cfg(any(test, feature = "test"))]
use default_test::DefaultTest;
#[cfg(any(test, feature = "test"))]
use crate::test_utils::default_from::DefaultFrom;
#[cfg(any(test, feature = "test"))]
use rand::Rng;
#[cfg(any(test, feature = "test"))]
use crate::test_utils::r_string;
#[cfg(any(test, feature = "test"))]
use serde_json::json;

#[cfg(any(test, feature = "test"))]
impl DefaultTest for PropertyInstances {
    fn default_test() -> Self {
        let property_instances = PropertyInstances::new();
        let mut rng = rand::thread_rng();
        for _ in 0 .. rng.gen_range(0..10) {
            property_instances.insert(r_string(), json!(r_string()));
        }
        property_instances
    }
}

#[cfg(any(test, feature = "test"))]
impl DefaultFrom<PropertyTypes> for PropertyInstances {
    fn default_from(property_types: &PropertyTypes) -> Self {
        let properties = Self::new();
        for property_type in property_types.iter() {
            properties.insert(property_type.key().clone(), property_type.data_type.default_value_test());
        }
        properties
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests
}
