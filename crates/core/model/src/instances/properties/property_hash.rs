use std::hash::{Hash, Hasher};
use serde_json::Value;

/// Wrapper for JSON values which allows to create a hash of the value.
/// This enables implementing Hash for property instances.
pub struct HashableValue<'a>(pub &'a Value);

impl<'a> Hash for HashableValue<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Null => 0.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::Number(n) => n.hash(state),
            Value::String(str) => str.hash(state),
            Value::Array(arr) => arr.iter().for_each(|a| HashableValue(a).hash(state)),
            Value::Object(obj) => obj.iter().for_each(|entry| {
                entry.0.hash(state);
                HashableValue(entry.1).hash(state);
            }),
        }
    }
}
