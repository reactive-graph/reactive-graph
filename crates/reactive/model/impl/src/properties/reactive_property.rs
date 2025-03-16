use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

use dashmap::DashMap;
use dashmap::iter::OwningIter;
use serde_json::Map;
use serde_json::Value;

use crate::Stream;

use reactive_graph_graph::ContainerPropertyInstance;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::Mutability::Mutable;
use reactive_graph_graph::PropertyInstances;

pub struct ReactiveProperty<IdType: Clone> {
    /// The parent identifier (entity: uuid, relation: RelationInstanceId)
    pub id: IdType,

    /// Property name
    pub name: String,

    /// The property instance is mutable or immutable.
    pub mutability: Mutability,

    /// The reactive stream
    pub stream: Arc<RwLock<Stream<'static, Value>>>,

    /// Store the current value
    pub value: RwLock<Value>,
}

impl<IdType: Clone> ReactiveProperty<IdType> {
    pub fn new<S: Into<String>>(id: IdType, name: S, mutability: Mutability, value: Value) -> ReactiveProperty<IdType> {
        ReactiveProperty {
            id,
            name: name.into(),
            mutability,
            stream: Arc::new(RwLock::new(Stream::new())),
            value: RwLock::new(value),
        }
    }

    pub fn get(&self) -> Value {
        self.value.read().unwrap().clone()
    }

    pub fn set_checked(&self, value: Value) {
        if self.mutability == Mutable {
            self.set(value);
        }
    }

    pub fn set(&self, value: Value) {
        let mut writer = self.value.write().unwrap();
        *writer.deref_mut() = value.clone();
        self.stream.read().unwrap().send(&value);
    }

    pub fn set_no_propagate_checked(&self, value: Value) {
        if self.mutability == Mutable {
            self.set_no_propagate(value);
        }
    }

    pub fn set_no_propagate(&self, value: Value) {
        let mut writer = self.value.write().unwrap();
        *writer.deref_mut() = value;
    }

    /// Send a value down the stream, but does not change the current value
    pub fn send(&self, signal: &Value) {
        self.stream.read().unwrap().send(signal);
    }

    /// Resend the current value manually if mutable
    pub fn tick_checked(&self) {
        if self.mutability == Mutable {
            self.tick();
        }
    }

    /// Resend the current value manually
    pub fn tick(&self) {
        // println!("tick {}::{}", self.id, self.name);
        let value = self.value.read().unwrap().deref().clone();
        self.stream.read().unwrap().send(&value);
    }

    pub fn set_mutability(&mut self, mutability: Mutability) {
        self.mutability = mutability;
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
        self.get().as_str().map(String::from)
    }

    pub fn as_array(&self) -> Option<Vec<Value>> {
        self.get().as_array().cloned()
    }

    pub fn as_object(&self) -> Option<Map<String, Value>> {
        self.get().as_object().cloned()
    }
}

impl<IdType: Clone> PartialEq for ReactiveProperty<IdType> {
    fn eq(&self, other: &Self) -> bool {
        self.value.read().unwrap().deref() == other.value.read().unwrap().deref()
    }
}

impl<IdType: Clone> From<ReactiveProperty<IdType>> for ContainerPropertyInstance<IdType> {
    fn from(property: ReactiveProperty<IdType>) -> Self {
        let reader = property.value.read().unwrap();
        ContainerPropertyInstance::new(property.id, property.name, reader.clone())
    }
}

impl<IdType: Clone> From<&ReactiveProperty<IdType>> for ContainerPropertyInstance<IdType> {
    fn from(property: &ReactiveProperty<IdType>) -> Self {
        let reader = property.value.read().unwrap();
        ContainerPropertyInstance::new(property.id.clone(), property.name.clone(), reader.clone())
    }
}

// #[derive(Default)]
pub struct ReactiveProperties<IdType: Clone>(DashMap<String, ReactiveProperty<IdType>>);

impl<IdType: Clone> ReactiveProperties<IdType> {
    /// Constructs an empty reactive properties container.
    pub fn new() -> Self {
        ReactiveProperties(DashMap::new())
    }

    /// Constructs a reactive properties container with the given properties which gets bound.
    pub fn new_with_id_from_properties<I: Into<IdType>, P: Into<PropertyInstances>>(id: I, properties: P) -> ReactiveProperties<IdType> {
        let id = id.into();
        let reactive_properties = ReactiveProperties::new();
        for (property_name, value) in properties.into().into_iter() {
            reactive_properties.insert(property_name.clone(), ReactiveProperty::new(id.clone(), property_name.clone(), Mutable, value));
        }
        reactive_properties
    }

    pub fn property<P: Into<ReactiveProperty<IdType>>>(self, property: P) -> Self {
        let property = property.into();
        self.insert(property.name.clone(), property);
        self
    }
}

impl<IdType: Clone> Default for ReactiveProperties<IdType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<IdType: Clone> Deref for ReactiveProperties<IdType> {
    type Target = DashMap<String, ReactiveProperty<IdType>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<IdType: Clone> DerefMut for ReactiveProperties<IdType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<IdType: Clone> IntoIterator for ReactiveProperties<IdType> {
    type Item = (String, ReactiveProperty<IdType>);
    type IntoIter = OwningIter<String, ReactiveProperty<IdType>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<IdType: Clone> FromIterator<ReactiveProperty<IdType>> for ReactiveProperties<IdType> {
    fn from_iter<I: IntoIterator<Item = ReactiveProperty<IdType>>>(iter: I) -> Self {
        let properties = ReactiveProperties::new();
        for property in iter {
            properties.insert(property.name.clone(), property);
        }
        properties
    }
}

impl<IdType: Clone> From<ReactiveProperties<IdType>> for PropertyInstances {
    fn from(properties: ReactiveProperties<IdType>) -> Self {
        let property_instances = PropertyInstances::new();
        for (property_name, property) in properties.into_iter() {
            property_instances.insert(property_name, property.get());
        }
        property_instances
    }
}

impl<IdType: Clone> From<&ReactiveProperties<IdType>> for PropertyInstances {
    fn from(properties: &ReactiveProperties<IdType>) -> Self {
        let property_instances = PropertyInstances::new();
        for property in properties.0.iter() {
            property_instances.insert(property.key().clone(), property.get());
        }
        property_instances
    }
}

// impl Add for ReactiveProperty {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         self.value.read().unwrap().deref() + rhs.value.read().unwrap().deref()
//     }
// }

// impl AddAssign for ReactiveProperty {
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

#[cfg(test)]
pub mod tests {
    use std::ops::DerefMut;
    use std::ops::Index;
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering;
    use std::thread;

    use rand::Rng;
    use serde_json::json;
    use stopwatch2::Stopwatch;
    use uuid::Uuid;

    use crate::Stream;

    use crate::ReactiveProperty;
    use reactive_graph_graph::Mutability::Mutable;
    use reactive_graph_test_utils::r_string;

    #[test]
    fn reactive_property_instance_test() {
        let uuid = Uuid::new_v4();

        let property_name = r_string();

        let initial_property_value = r_string();

        let initial_property_value_json = json!(initial_property_value);

        let reactive_property_instance = ReactiveProperty {
            id: uuid,
            name: property_name.clone(),
            stream: Arc::new(RwLock::new(Stream::new())),
            mutability: Mutable,
            value: RwLock::new(initial_property_value_json),
        };

        // Check that the meta data is correct
        assert_eq!(uuid, reactive_property_instance.id);
        assert_eq!(property_name.clone(), reactive_property_instance.name);
        assert_eq!(initial_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());

        // Set: Send to "stream", write inner "value"

        let new_property_value = r_string();
        let new_property_value_json = json!(new_property_value);

        reactive_property_instance.set(new_property_value_json);

        // Check that the inner value has changed
        assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
        assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

        // Send: Send to "stream", do not change the inner "value" (!)

        let send_property_value = r_string();
        let send_property_value_json = json!(send_property_value);

        reactive_property_instance.send(&send_property_value_json);

        // Check that the inner value has not changed
        assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
        assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

        // Check that the inner value is the same
        assert_ne!(send_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
        assert_ne!(send_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

        // Create an observer which sinks on a variable

        let observed_value_json = Arc::new(RwLock::new(reactive_property_instance.get()));
        let inner_observed_value_json = Arc::clone(&observed_value_json);
        reactive_property_instance.stream.read().unwrap().observe(move |value| {
            let mut writer = inner_observed_value_json.write().unwrap();
            *writer.deref_mut() = value.clone();
        });

        reactive_property_instance.send(&send_property_value_json);

        // Check that the observer gets the sent value
        assert_eq!(send_property_value.as_str(), observed_value_json.read().unwrap().as_str().unwrap());
        // Check that the value hasn't changed
        assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

        // Resend the last value

        let tick_value_json = Arc::new(RwLock::new(json!("")));
        let i_tick_value_json = Arc::clone(&tick_value_json);
        reactive_property_instance.stream.read().unwrap().observe(move |value| {
            let mut writer = i_tick_value_json.write().unwrap();
            *writer.deref_mut() = value.clone();
        });

        reactive_property_instance.tick();

        // Check that the inner value has been sent to the observer
        assert_eq!(new_property_value.as_str(), tick_value_json.read().unwrap().as_str().unwrap());
    }

    #[test]
    fn create_reactive_property_instance_test() {
        let uuid = Uuid::new_v4();
        let property_name = r_string();
        let initial_property_value = r_string();
        let initial_property_value_json = json!(initial_property_value);
        let reactive_property_instance = ReactiveProperty::new(uuid, property_name.clone(), Mutable, initial_property_value_json);

        assert_eq!(uuid, reactive_property_instance.id);
        assert_eq!(property_name.clone(), reactive_property_instance.name);
        assert_eq!(initial_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());

        // Set: Send to "stream", write "value"

        let new_property_value = r_string();
        let new_property_value_json = json!(new_property_value);
        reactive_property_instance.set_no_propagate(new_property_value_json);

        assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
        assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());

        // Send: Send to "stream", do not change "value"

        let send_property_value = r_string();
        let send_property_value_json = json!(send_property_value);
        reactive_property_instance.send(&send_property_value_json);

        assert_eq!(new_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
        assert_eq!(new_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());
        assert_ne!(send_property_value.as_str(), reactive_property_instance.value.read().unwrap().as_str().unwrap());
        assert_ne!(send_property_value.as_str(), reactive_property_instance.get().as_str().unwrap());
    }

    #[test]
    fn reactive_property_instance_typed_getter_test() {
        let property_name = r_string();

        let bool_value = json!(true);
        assert_eq!(
            bool_value.as_bool().unwrap(),
            ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, bool_value)
                .as_bool()
                .unwrap()
        );

        let u64 = json!(123);
        assert_eq!(123, ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, u64).as_u64().unwrap());

        let i64 = json!(-123);
        assert_eq!(-123, ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, i64).as_i64().unwrap());

        let f64 = json!(-1.23);
        assert_eq!(-1.23, ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, f64).as_f64().unwrap());

        let rand_str = r_string();
        let s = json!(rand_str.clone());
        assert_eq!(
            rand_str.clone(),
            ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, s).as_string().unwrap()
        );

        let a = json!([1, 2, 3]);
        let i = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, a);
        assert_eq!(json!(1), i.as_array().unwrap().index(0).clone());
        assert_eq!(json!(2), i.as_array().unwrap().index(1).clone());
        assert_eq!(json!(3), i.as_array().unwrap().index(2).clone());

        let o = json!({
            "k": "v"
        });
        let i = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, o);
        assert_eq!(json!("v"), i.as_object().unwrap().index("k").clone());
    }

    #[test]
    fn reactive_property_instance_eq_bool_test() {
        let property_name = r_string();

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
        assert!(instance1 == instance2);

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
        assert!(instance1 != instance2);

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
        assert!(instance1 != instance2);

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
        assert!(instance1 == instance2);
    }

    #[test]
    fn reactive_property_instance_eq_number_test() {
        let property_name = r_string();

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1));
        assert!(instance1 == instance2);

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(2));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(3));
        assert!(instance1 != instance2);
    }

    #[test]
    fn reactive_property_instance_eq_float_test() {
        let property_name = r_string();

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(0.0));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(0.0));
        assert!(instance1 == instance2);

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1.0));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1.1));
        assert!(instance1 != instance2);
    }

    #[test]
    fn reactive_property_instance_eq_string_test() {
        let property_name = r_string();
        let property_value = r_string();

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(property_value.clone()));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(property_value.clone()));
        assert!(instance1 == instance2);

        let instance1 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(r_string()));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(r_string()));
        assert!(instance1 != instance2);
    }

    #[test]
    fn reactive_property_instance_stream_test() {
        let instance1 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

        let v = Arc::new(AtomicU64::new(0));

        {
            let v = v.clone();
            let writer = instance2.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            writer.observe_with_handle(
                move |value| {
                    v.store(value.as_u64().unwrap(), Ordering::Relaxed);
                },
                handle_id,
            );
        }

        {
            let writer = instance1.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            writer.observe_with_handle(
                move |value| {
                    instance2.set(value.clone());
                },
                handle_id,
            );
        }

        instance1.set(json!(1));
        assert_eq!(1, v.load(Ordering::Relaxed));
    }

    // TODO: implement cycle loop protection!
    #[test]
    #[ignore]
    fn reactive_property_instance_stream_loop_test() {
        let instance1 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

        {
            let writer = instance2.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            let instance1_stream = instance1.stream.clone();
            writer.observe_with_handle(
                move |value| {
                    instance1_stream.write().unwrap().send(value);
                },
                handle_id,
            );
        }

        {
            let writer = instance1.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            let instance2_stream = instance2.stream.clone();
            writer.observe_with_handle(
                move |value| {
                    instance2_stream.write().unwrap().send(value);
                },
                handle_id,
            );
        }

        let mut rng = rand::rng();

        let number: u64 = rng.random();
        instance1.set(json!(number));
    }

    #[test]
    #[ignore]
    fn reactive_property_instance_stream_mt_benchmark() {
        let instance1 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
        let instance2 = ReactiveProperty::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

        let v = Arc::new(AtomicU64::new(0));

        {
            let v = v.clone();
            let writer = instance2.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            writer.observe_with_handle(
                move |value| {
                    v.store(value.as_u64().unwrap(), Ordering::Relaxed);
                },
                handle_id,
            );
        }

        {
            let writer = instance1.stream.write().unwrap();
            let handle_id = Uuid::new_v4().as_u128();
            writer.observe_with_handle(
                move |value| {
                    instance2.set(value.clone());
                },
                handle_id,
            );
        }

        const NUM_THREADS: i32 = 256;
        const NUM_ITERATIONS: i32 = 10000;

        println!("starting");
        let mut handles = Vec::new();
        let mut s1 = Stopwatch::default();
        s1.start();
        for thread_no in 1..NUM_THREADS {
            let v = v.clone();
            let stream = instance1.stream.clone();
            let handle = thread::spawn(move || {
                let mut rng = rand::rng();
                let mut s = Stopwatch::default();
                s.start();
                for _iteration in 1..NUM_ITERATIONS {
                    let number: u64 = rng.random();
                    let value = json!(number);
                    let writer = stream.write().unwrap();
                    writer.send(&value);
                    let new_value = v.load(Ordering::Relaxed);
                    assert_eq!(number, new_value);
                }
                s.stop();
                println!("finished thread [{}] in {:?}", thread_no, s.elapsed());
            });
            handles.push(handle)
        }
        println!("started {} parallel threads in {:?}", NUM_THREADS, s1.elapsed());
        s1.start();
        println!("running");
        handles.into_iter().for_each(move |handle| {
            handle.join().unwrap();
        });
        println!(
            "finished {} parallel threads with each {} stream propagations (total {}) in {:?}",
            NUM_THREADS,
            NUM_ITERATIONS,
            NUM_THREADS * NUM_ITERATIONS,
            s1.elapsed()
        );
    }
}
