use serde_json::Map;
use serde_json::Value;

use crate::Mutability;

pub trait PropertyInstanceGetter {
    /// Returns the json value of the given property by name
    fn get<S: Into<String>>(&self, property_name: S) -> Option<Value>;

    /// Returns the boolean value of the given property by name
    fn as_bool<S: Into<String>>(&self, property_name: S) -> Option<bool>;

    /// Returns the u64 value of the given property by name
    fn as_u64<S: Into<String>>(&self, property_name: S) -> Option<u64>;

    /// Returns the i64 value of the given property by name
    fn as_i64<S: Into<String>>(&self, property_name: S) -> Option<i64>;

    /// Returns the f64 value of the given property by name
    fn as_f64<S: Into<String>>(&self, property_name: S) -> Option<f64>;

    /// Returns the string value of the given property by name
    fn as_string<S: Into<String>>(&self, property_name: S) -> Option<String>;

    /// Returns the string value of the given property by name
    fn as_array<S: Into<String>>(&self, property_name: S) -> Option<Vec<Value>>;

    /// Returns the string value of the given property by name
    fn as_object<S: Into<String>>(&self, property_name: S) -> Option<Map<String, Value>>;

    // TODO: integrate with other non-primitive types
    // as_byte_array (string BASE64 -> Vec<u8>)
    // as_date (string ISO8601 -> chrono::Date)
    // as_date_time (string ISO8601 -> chrono::DateTime)
    // as_time (string ISO8601 -> chrono::naive::NaiveTime)
    // as_uuid (string uuid -> Uuid)
}

pub trait MutablePropertyInstanceSetter: PropertyInstanceGetter {
    /// Sets the value of the given property by name
    fn set<S: Into<String>>(&mut self, property_name: S, value: Value);

    // TODO: Typed setters
    // set_bool(property_name, value)
    // set_number(property_name, value)
    // set_byte_array(property_name, u8[] -> string BASE64)
    // set_date(property_name, chrono::Date -> string ISO8601)
    // set_date_time(property_name, chrono::Date -> string ISO8601)
    // as_time(property_name, chrono::naive::NaiveTime -> string ISO8601)
    // as_uuid(property_name, Uuid -> string uuid)
}

pub trait PropertyInstanceSetter: PropertyInstanceGetter {
    /// Sets the value of the given property by name if the property is mutable.
    fn set_checked<S: Into<String>>(&self, property_name: S, value: Value);

    /// Sets the value of the given property by name
    fn set<S: Into<String>>(&self, property_name: S, value: Value);

    /// Sets the value of the given property by name if the property is mutable. Sends the value
    /// down the stream.
    fn set_no_propagate_checked<S: Into<String>>(&self, property_name: S, value: Value);

    /// Sets the value of the given property by name. Sends the value down the stream.
    fn set_no_propagate<S: Into<String>>(&self, property_name: S, value: Value);

    /// Returns the mutability of the property by name.
    fn mutability<S: Into<String>>(&self, property_name: S) -> Option<Mutability>;

    /// Sets the mutability of the property by name.
    fn set_mutability<S: Into<String>>(&self, property_name: S, mutability: Mutability);
}

#[macro_export]
macro_rules! rx_accessor {
    (get $getter_name: ident value) => {
        pub fn $getter_name(&self) -> Option<serde_json::Value> {
            self.i.get(stringify!($getter_name))
        }
    };
    (get $getter_name: ident bool) => {
        pub fn $getter_name(&self) -> Option<bool> {
            self.i.as_bool(stringify!($getter_name))
        }
    };
    (get $getter_name: ident u64) => {
        pub fn $getter_name(&self) -> Option<u64> {
            self.i.as_u64(stringify!($getter_name))
        }
    };
    (get $getter_name: ident i64) => {
        pub fn $getter_name(&self) -> Option<i64> {
            self.i.as_i64(stringify!($getter_name))
        }
    };
    (get $getter_name: ident f64) => {
        pub fn $getter_name(&self) -> Option<f64> {
            self.i.as_f64(stringify!($getter_name))
        }
    };
    (get $getter_name: ident string) => {
        pub fn $getter_name(&self) -> Option<String> {
            self.i.as_string(stringify!($getter_name)).map(String::from)
        }
    };
    (get $getter_name: ident array) => {
        pub fn $getter_name(&self) -> Option<Vec<serde_json::Value>> {
            self.i.as_array(stringify!($getter_name))
        }
    };
    (get $getter_name: ident object) => {
        pub fn $getter_name(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
            self.i.as_object(stringify!($getter_name))
        }
    };
    (set $setter_name: ident value) => {
        pub fn $setter_name(&self, v: serde_json::Value) {
            self.i.set(stringify!($setter_name), v);
        }
    };
    (set $setter_name: ident bool) => {
        pub fn $setter_name(&self, v: bool) {
            self.i.set(stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident u64) => {
        pub fn $setter_name(&self, v: u64) {
            self.i.set(stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident i64) => {
        pub fn $setter_name(&self, v: i64) {
            self.i.set(stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident f64) => {
        pub fn $setter_name(&self, v: f64) {
            self.i.set(stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident string) => {
        pub fn $setter_name(&self, v: String) {
            self.i.set(stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident array) => {
        pub fn $setter_name(&self, v: Vec<serde_json::Value>) {
            self.i.set(stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident object) => {
        pub fn $setter_name(&self, v: serde_json::Map<String, serde_json::Value>) {
            self.i.set(stringify!($setter_name), serde_json::json!(v));
        }
    };
}
