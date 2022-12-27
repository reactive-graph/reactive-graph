extern crate test;

use std::ops::DerefMut;
use std::ops::Index;
use std::process::Termination;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use test::Bencher;

use crate::Mutability::Mutable;
use inexor_rgf_core_frp::Stream;
use rand::Rng;
use serde_json::json;
use stopwatch2::Stopwatch;
use uuid::Uuid;

use crate::tests::utils::r_string;
use crate::ReactivePropertyInstance;

#[test]
fn reactive_property_instance_test() {
    let uuid = Uuid::new_v4();

    let property_name = r_string();

    let initial_property_value = r_string();

    let initial_property_value_json = json!(initial_property_value);

    let reactive_property_instance = ReactivePropertyInstance {
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
    let reactive_property_instance = ReactivePropertyInstance::new(uuid, property_name.clone(), Mutable, initial_property_value_json);

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
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, bool_value)
            .as_bool()
            .unwrap()
    );

    let u64 = json!(123);
    assert_eq!(
        123,
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, u64)
            .as_u64()
            .unwrap()
    );

    let i64 = json!(-123);
    assert_eq!(
        -123,
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, i64)
            .as_i64()
            .unwrap()
    );

    let f64 = json!(-1.23);
    assert_eq!(
        -1.23,
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, f64)
            .as_f64()
            .unwrap()
    );

    let rand_str = r_string();
    let s = json!(rand_str.clone());
    assert_eq!(
        rand_str.clone(),
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, s)
            .as_string()
            .unwrap()
    );

    let a = json!([1, 2, 3]);
    let i = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, a);
    assert_eq!(json!(1), i.as_array().unwrap().index(0).clone());
    assert_eq!(json!(2), i.as_array().unwrap().index(1).clone());
    assert_eq!(json!(3), i.as_array().unwrap().index(2).clone());

    let o = json!({
        "k": "v"
    });
    let i = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, o);
    assert_eq!(json!("v"), i.as_object().unwrap().index("k").clone());
}

#[test]
fn reactive_property_instance_eq_bool_test() {
    let property_name = r_string();

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
    assert!(instance1 == instance2);

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
    assert!(instance1 != instance2);

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(true));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
    assert!(instance1 != instance2);

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(false));
    assert!(instance1 == instance2);
}

#[test]
fn reactive_property_instance_eq_number_test() {
    let property_name = r_string();

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1));
    assert!(instance1 == instance2);

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(2));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(3));
    assert!(instance1 != instance2);
}

#[test]
fn reactive_property_instance_eq_float_test() {
    let property_name = r_string();

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(0.0));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(0.0));
    assert!(instance1 == instance2);

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1.0));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(1.1));
    assert!(instance1 != instance2);
}

#[test]
fn reactive_property_instance_eq_string_test() {
    let property_name = r_string();
    let property_value = r_string();

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(property_value.clone()));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(property_value.clone()));
    assert!(instance1 == instance2);

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(r_string()));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), Mutable, json!(r_string()));
    assert!(instance1 != instance2);
}

#[test]
fn reactive_property_instance_stream_test() {
    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

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
    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

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

    let mut rng = rand::thread_rng();

    let number: u64 = rng.gen();
    instance1.set(json!(number));
}

#[bench]
fn reactive_property_instance_stream_benchmark(bencher: &mut Bencher) -> impl Termination {
    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

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

    let mut rng = rand::thread_rng();

    bencher.iter(move || {
        let number: u64 = rng.gen();
        instance1.set(json!(number));
        assert_eq!(number, v.load(Ordering::Relaxed));
    })
}

#[test]
#[ignore]
fn reactive_property_instance_stream_mt_benchmark() {
    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), r_string(), Mutable, json!(0));

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
            let mut rng = rand::thread_rng();
            let mut s = Stopwatch::default();
            s.start();
            for _iteration in 1..NUM_ITERATIONS {
                let number: u64 = rng.gen();
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
