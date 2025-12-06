use serde_json::Map;
use serde_json::Value;

use crate::Mutability;

pub trait PropertyInstanceGetter {
    /// Returns the json value of the given property by name
    fn get(&self, property_name: &str) -> Option<Value>;

    /// Returns the boolean value of the given property by name
    fn as_bool(&self, property_name: &str) -> Option<bool>;

    /// Returns the u64 value of the given property by name
    fn as_u64(&self, property_name: &str) -> Option<u64>;

    /// Returns the i64 value of the given property by name
    fn as_i64(&self, property_name: &str) -> Option<i64>;

    /// Returns the f64 value of the given property by name
    fn as_f64(&self, property_name: &str) -> Option<f64>;

    /// Returns the string value of the given property by name
    fn as_string(&self, property_name: &str) -> Option<String>;

    /// Returns the string value of the given property by name
    fn as_array(&self, property_name: &str) -> Option<Vec<Value>>;

    /// Returns the string value of the given property by name
    fn as_object(&self, property_name: &str) -> Option<Map<String, Value>>;

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
    fn set_checked(&self, property_name: &str, value: Value);

    /// Sets the value of the given property by name
    fn set(&self, property_name: &str, value: Value);

    /// Sets the value of the given property by name if the property is mutable. Sends the value
    /// down the stream.
    fn set_no_propagate_checked(&self, property_name: &str, value: Value);

    /// Sets the value of the given property by name. Sends the value down the stream.
    fn set_no_propagate(&self, property_name: &str, value: Value);

    /// Returns the mutability of the property by name.
    fn mutability(&self, property_name: &str) -> Option<Mutability>;

    /// Sets the mutability of the property by name.
    fn set_mutability(&self, property_name: &str, mutability: Mutability);
}

#[macro_export]
macro_rules! rx_accessor {
    // Special accessors
    (trigger) => {
        fn trigger(&self) {
            $crate::PropertyInstanceSetter::set(self, "trigger", serde_json::json!(true));
        }
    };
    (pub trigger) => {
        pub fn trigger(&self) {
            $crate::PropertyInstanceSetter::set(self, "trigger", serde_json::json!(true));
        }
    };
    // Getters
    (get $getter_name: ident value) => {
        fn $getter_name(&self) -> Option<serde_json::Value> {
            $crate::PropertyInstanceGetter::get(self, stringify!($getter_name))
        }
    };
    (pub get $getter_name: ident value) => {
        pub fn $getter_name(&self) -> Option<serde_json::Value> {
            $crate::PropertyInstanceGetter::get(self, stringify!($getter_name))
        }
    };
    (get $getter_name: ident bool) => {
        fn $getter_name(&self) -> Option<bool> {
            $crate::PropertyInstanceGetter::as_bool(self, stringify!($getter_name))
        }
    };
    (pub get $getter_name: ident bool) => {
        pub fn $getter_name(&self) -> Option<bool> {
            $crate::PropertyInstanceGetter::as_bool(self, stringify!($getter_name))
        }
    };
    (get $getter_name: ident u64) => {
        fn $getter_name(&self) -> Option<u64> {
            $crate::PropertyInstanceGetter::as_u64(self, stringify!($getter_name))
        }
    };
    (pub get $getter_name: ident u64) => {
        pub fn $getter_name(&self) -> Option<u64> {
            $crate::PropertyInstanceGetter::as_u64(self, stringify!($getter_name))
        }
    };
    (get $getter_name: ident i64) => {
        fn $getter_name(&self) -> Option<i64> {
            $crate::PropertyInstanceGetter::as_i64(self, stringify!($getter_name))
        }
    };
    (pub get $getter_name: ident i64) => {
        pub fn $getter_name(&self) -> Option<i64> {
            $crate::PropertyInstanceGetter::as_i64(self, stringify!($getter_name))
        }
    };
    (get $getter_name: ident f64) => {
        fn $getter_name(&self) -> Option<f64> {
            $crate::PropertyInstanceGetter::as_f64(self, stringify!($getter_name))
        }
    };
    (pub get $getter_name: ident f64) => {
        pub fn $getter_name(&self) -> Option<f64> {
            $crate::PropertyInstanceGetter::as_f64(self, stringify!($getter_name))
        }
    };
    (get $getter_name: ident string) => {
        fn $getter_name(&self) -> Option<String> {
            $crate::PropertyInstanceGetter::as_string(self, stringify!($getter_name)).map(String::from)
        }
    };
    (pub get $getter_name: ident string) => {
        pub fn $getter_name(&self) -> Option<String> {
            $crate::PropertyInstanceGetter::as_string(self, stringify!($getter_name)).map(String::from)
        }
    };
    (get $getter_name: ident array) => {
        fn $getter_name(&self) -> Option<Vec<serde_json::Value>> {
            $crate::PropertyInstanceGetter::as_array(self, stringify!($getter_name))
        }
    };
    (pub get $getter_name: ident array) => {
        pub fn $getter_name(&self) -> Option<Vec<serde_json::Value>> {
            $crate::PropertyInstanceGetter::as_array(self, stringify!($getter_name))
        }
    };
    (get $getter_name: ident object) => {
        fn $getter_name(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
            $crate::PropertyInstanceGetter::as_object(self, stringify!($getter_name))
        }
    };
    (pub get $getter_name: ident object) => {
        pub fn $getter_name(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
            $crate::PropertyInstanceGetter::as_object(self, stringify!($getter_name))
        }
    };
    // Setters
    (set $setter_name: ident value) => {
        fn $setter_name(&self, v: serde_json::Value) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), v);
        }
    };
    (pub set $setter_name: ident value) => {
        pub fn $setter_name(&self, v: serde_json::Value) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), v);
        }
    };
    (set $setter_name: ident bool) => {
        fn $setter_name(&self, v: bool) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (pub set $setter_name: ident bool) => {
        pub fn $setter_name(&self, v: bool) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident u64) => {
        fn $setter_name(&self, v: u64) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (pub set $setter_name: ident u64) => {
        pub fn $setter_name(&self, v: u64) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident i64) => {
        fn $setter_name(&self, v: i64) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (pub set $setter_name: ident i64) => {
        pub fn $setter_name(&self, v: i64) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident f64) => {
        fn $setter_name(&self, v: f64) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (pub set $setter_name: ident f64) => {
        pub fn $setter_name(&self, v: f64) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident string) => {
        fn $setter_name<S: Into<String>>(&self, v: S) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v.into()));
        }
    };
    (pub set $setter_name: ident string) => {
        pub fn $setter_name<S: Into<String>>(&self, v: S) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v.into()));
        }
    };
    (set $setter_name: ident array) => {
        fn $setter_name(&self, v: Vec<serde_json::Value>) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (pub set $setter_name: ident array) => {
        pub fn $setter_name(&self, v: Vec<serde_json::Value>) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (set $setter_name: ident object) => {
        fn $setter_name(&self, v: serde_json::Map<String, serde_json::Value>) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    (pub set $setter_name: ident object) => {
        pub fn $setter_name(&self, v: serde_json::Map<String, serde_json::Value>) {
            $crate::PropertyInstanceSetter::set(self, stringify!($setter_name), serde_json::json!(v));
        }
    };
    // data (getter + setter)
    (data $property_name: ident value) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<serde_json::Value> {
                $crate::PropertyInstanceGetter::get(self, stringify!($property_name))
            }
            fn [<set_ $property_name>](&self, v: serde_json::Value) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), v);
            }
        }
    };
    (pub data $property_name: ident value) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<serde_json::Value> {
                $crate::PropertyInstanceGetter::get(self, stringify!($property_name))
            }
            pub fn [<set_ $property_name>](&self, v: serde_json::Value) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), v);
            }
        }
    };
    (data $property_name: ident bool) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<bool> {
                $crate::PropertyInstanceGetter::as_bool(self, stringify!($property_name))
            }
            fn [<set_ $property_name>](&self, v: bool) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (pub data $property_name: ident bool) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<bool> {
                $crate::PropertyInstanceGetter::as_bool(self, stringify!($property_name))
            }
            pub fn [<set_ $property_name>](&self, v: bool) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (data $property_name: ident u64) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<u64> {
                $crate::PropertyInstanceGetter::as_u64(self, stringify!($property_name))
            }
            fn [<set_ $property_name>](&self, v: u64) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (pub data $property_name: ident u64) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<u64> {
                $crate::PropertyInstanceGetter::as_u64(self, stringify!($property_name))
            }
            pub fn [<set_ $property_name>](&self, v: u64) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (data $property_name: ident i64) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<i64> {
                $crate::PropertyInstanceGetter::as_i64(self, stringify!($property_name))
            }
            fn [<set_ $property_name>](&self, v: i64) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (pub data $property_name: ident i64) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<i64> {
                $crate::PropertyInstanceGetter::as_i64(self, stringify!($property_name))
            }
            pub fn [<set_ $property_name>](&self, v: i64) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (data $property_name: ident f64) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<f64> {
                $crate::PropertyInstanceGetter::as_f64(self, stringify!($property_name))
            }
            fn [<set_ $property_name>](&self, v: f64) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (pub data $property_name: ident f64) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<f64> {
                $crate::PropertyInstanceGetter::as_f64(self, stringify!($property_name))
            }
            pub fn [<set_ $property_name>](&self, v: f64) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (data $property_name: ident string) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<String> {
                $crate::PropertyInstanceGetter::as_string(self, stringify!($property_name)).map(String::from)
            }
            fn [<set_ $property_name>]<S: Into<String>>(&self, v: S) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v.into()));
            }
        }
    };
    (pub data $property_name: ident string) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<String> {
                $crate::PropertyInstanceGetter::as_string(self, stringify!($property_name)).map(String::from)
            }
            pub fn [<set_ $property_name>]<S: Into<String>>(&self, v: S) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v.into()));
            }
        }
    };
    (data $property_name: ident array) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<Vec<serde_json::Value>> {
                $crate::PropertyInstanceGetter::as_array(self, stringify!($property_name))
            }
            fn [<set_ $property_name>](&self, v: Vec<serde_json::Value>) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (pub data $property_name: ident array) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<Vec<serde_json::Value>> {
                $crate::PropertyInstanceGetter::as_array(self, stringify!($property_name))
            }
            pub fn [<set_ $property_name>](&self, v: Vec<serde_json::Value>) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (data $property_name: ident object) => {
        paste::paste! {
            fn [<get_ $property_name>](&self) -> Option<serde_json::Map<String, serde_json::Value>> {
                $crate::PropertyInstanceGetter::as_object(self, stringify!($property_name))
            }
            fn [<set_ $property_name>](&self, v: serde_json::Map<String, serde_json::Value>) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
    (pub data $property_name: ident object) => {
        paste::paste! {
            pub fn [<get_ $property_name>](&self) -> Option<serde_json::Map<String, serde_json::Value>> {
                $crate::PropertyInstanceGetter::as_object(self, stringify!($property_name))
            }
            pub fn [<set_ $property_name>](&self, v: serde_json::Map<String, serde_json::Value>) {
                $crate::PropertyInstanceSetter::set(self, stringify!($property_name), serde_json::json!(v));
            }
        }
    };
}
