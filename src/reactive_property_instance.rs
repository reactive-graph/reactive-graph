use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

use inexor_rgf_core_frp::Stream;
use serde_json::{Map, Value};
use uuid::Uuid;

pub struct ReactivePropertyInstance {
    /// Vertex uuid
    pub id: Uuid,

    /// Property name
    pub name: String,

    /// The reactive stream
    pub stream: Arc<RwLock<Stream<'static, Value>>>,

    /// Store the current value
    pub value: RwLock<Value>,
}

impl ReactivePropertyInstance {
    pub fn new<S: Into<String>>(id: Uuid, name: S, value: Value) -> ReactivePropertyInstance {
        ReactivePropertyInstance {
            id,
            name: name.into(),
            stream: Arc::new(RwLock::new(Stream::new())),
            value: RwLock::new(value),
        }
    }

    pub fn get(&self) -> Value {
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, value: Value) {
        let mut writer = self.value.write().unwrap();
        *writer.deref_mut() = value.clone();
        self.stream.read().unwrap().send(&value);
    }

    pub fn set_no_propagate(&self, value: Value) {
        let mut writer = self.value.write().unwrap();
        *writer.deref_mut() = value;
    }

    /// Send a value down the stream, but does not change the current value
    pub fn send(&self, signal: &Value) {
        self.stream.read().unwrap().send(signal);
    }

    /// Resend the current value manually
    pub fn tick(&self) {
        // println!("tick {}::{}", self.id, self.name);
        let value = self.value.read().unwrap().deref().clone();
        self.stream.read().unwrap().send(&value);
    }

    pub fn as_bool(&self) -> Option<bool> {
        self.get().as_bool()
    }

    pub fn as_u64(&self) -> Option<u64> {
        self.get().as_u64()
    }

    pub fn as_i64(&self) -> Option<i64> {
        self.get().as_i64()
    }

    pub fn as_f64(&self) -> Option<f64> {
        self.get().as_f64()
    }

    pub fn as_string(&self) -> Option<String> {
        self.get().as_str().and_then(|s| Some(String::from(s)))
    }

    pub fn as_array(&self) -> Option<Vec<Value>> {
        self.get().as_array().map(Vec::clone)
    }

    pub fn as_object(&self) -> Option<Map<String, Value>> {
        self.get().as_object().map(Map::clone)
    }
}

impl PartialEq for ReactivePropertyInstance {
    fn eq(&self, other: &Self) -> bool {
        self.value.read().unwrap().deref() == other.value.read().unwrap().deref()
    }
}

// impl Add for ReactivePropertyInstance {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         self.value.read().unwrap().deref() + rhs.value.read().unwrap().deref()
//     }
// }

// impl AddAssign for ReactivePropertyInstance {
//     fn add_assign(&mut self, rhs: Self) {
//         let v = *self.value.read().unwrap().deref() + *rhs.value.read().unwrap().deref();
//     }
// }

// TODO: implement PartialEq traits for bool, u64, i64, f64, string, &str
// This makes it possible to simplify comparison:
// if entity.get(name) == 32_i64 () { /* ...*/ }
// 1. as_i64() -> Option
// 2. if None -> false
// 3. if Some -> Compare -> true or false

// TODO: Implement operators
// https://doc.rust-lang.org/std/ops/index.html
// Add, A
// Sub

// TODO: Implement is_
// self.value.read().unwrap().is_boolean()
// is_64 is_array is_boolean is_i64 is_null is_number is_object is_string is_u64
