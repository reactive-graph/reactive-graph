use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

use inexor_rgf_core_bidule::Stream;
use serde_json::json;
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
        value: RwLock::new(initial_property_value_json),
    };

    // Check that the meta data is correct
    assert_eq!(uuid, reactive_property_instance.id);
    assert_eq!(property_name.clone(), reactive_property_instance.name);
    assert_eq!(
        initial_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );

    // Set: Send to "stream", write inner "value"

    let new_property_value = r_string();
    let new_property_value_json = json!(new_property_value);

    reactive_property_instance.set(new_property_value_json);

    // Check that the inner value has changed
    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance.get().as_str().unwrap()
    );

    // Send: Send to "stream", do not change the inner "value" (!)

    let send_property_value = r_string();
    let send_property_value_json = json!(send_property_value);

    reactive_property_instance.send(&send_property_value_json);

    // Check that the inner value has not changed
    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance.get().as_str().unwrap()
    );

    // Check that the inner value is the same
    assert_ne!(
        send_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_ne!(
        send_property_value.as_str(),
        reactive_property_instance.get().as_str().unwrap()
    );

    // Create an observer which sinks on a variable

    let observed_value_json = Arc::new(RwLock::new(reactive_property_instance.get()));
    let inner_observed_value_json = Arc::clone(&observed_value_json);
    reactive_property_instance
        .stream
        .read()
        .unwrap()
        .observe(move |value| {
            let mut writer = inner_observed_value_json.write().unwrap();
            *writer.deref_mut() = value.clone();
        });

    reactive_property_instance.send(&send_property_value_json);

    // Check that the observer gets the sent value
    assert_eq!(
        send_property_value.as_str(),
        observed_value_json.read().unwrap().as_str().unwrap()
    );
    // Check that the value hasn't changed
    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance.get().as_str().unwrap()
    );

    // Resend the last value

    let tick_value_json = Arc::new(RwLock::new(json!("")));
    let i_tick_value_json = Arc::clone(&tick_value_json);
    reactive_property_instance
        .stream
        .read()
        .unwrap()
        .observe(move |value| {
            let mut writer = i_tick_value_json.write().unwrap();
            *writer.deref_mut() = value.clone();
        });

    reactive_property_instance.tick();

    // Check that the inner value has been sent to the observer
    assert_eq!(
        new_property_value.as_str(),
        tick_value_json.read().unwrap().as_str().unwrap()
    );
}

#[test]
fn create_reactive_property_instance_test() {
    let uuid = Uuid::new_v4();
    let property_name = r_string();
    let initial_property_value = r_string();
    let initial_property_value_json = json!(initial_property_value);
    let reactive_property_instance =
        ReactivePropertyInstance::new(uuid, property_name.clone(), initial_property_value_json);

    assert_eq!(uuid, reactive_property_instance.id);
    assert_eq!(property_name.clone(), reactive_property_instance.name);
    assert_eq!(
        initial_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );

    // Set: Send to "stream", write "value"

    let new_property_value = r_string();
    let new_property_value_json = json!(new_property_value);
    reactive_property_instance.set_no_propagate(new_property_value_json);

    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance.get().as_str().unwrap()
    );

    // Send: Send to "stream", do not change "value"

    let send_property_value = r_string();
    let send_property_value_json = json!(send_property_value);
    reactive_property_instance.send(&send_property_value_json);

    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_eq!(
        new_property_value.as_str(),
        reactive_property_instance.get().as_str().unwrap()
    );
    assert_ne!(
        send_property_value.as_str(),
        reactive_property_instance
            .value
            .read()
            .unwrap()
            .as_str()
            .unwrap()
    );
    assert_ne!(
        send_property_value.as_str(),
        reactive_property_instance.get().as_str().unwrap()
    );
}

#[test]
fn reactive_property_instance_typed_getter_test() {
    let property_name = r_string();

    let bool_value = json!(true);
    assert_eq!(
        bool_value.as_bool().unwrap(),
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), bool_value)
            .as_bool()
            .unwrap()
    );

    let u64 = json!(123);
    assert_eq!(
        123,
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), u64)
            .as_u64()
            .unwrap()
    );

    let i64 = json!(-123);
    assert_eq!(
        -123,
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), i64)
            .as_i64()
            .unwrap()
    );

    let f64 = json!(-1.23);
    assert_eq!(
        -1.23,
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), f64)
            .as_f64()
            .unwrap()
    );

    let rand_str = r_string();
    let s = json!(rand_str.clone());
    assert_eq!(
        rand_str.clone(),
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), s)
            .as_string()
            .unwrap()
    );

    // TODO: unit test as_array
    // let array = json!(["an", "array"]);

    // TODO: unit test as_object
    // let v = json!({ "a": "some string", "b": false });
}

#[test]
fn reactive_property_instance_eq_bool_test() {
    let property_name = r_string();

    let instance1 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(true));
    let instance2 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(true));
    assert!(instance1 == instance2);

    let instance1 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(false));
    let instance2 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(true));
    assert!(instance1 != instance2);

    let instance1 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(true));
    let instance2 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(false));
    assert!(instance1 != instance2);

    let instance1 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(false));
    let instance2 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(false));
    assert!(instance1 == instance2);
}

#[test]
fn reactive_property_instance_eq_number_test() {
    let property_name = r_string();

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(1));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(1));
    assert!(instance1 == instance2);

    let instance1 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(2));
    let instance2 = ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(3));
    assert!(instance1 != instance2);
}

#[test]
fn reactive_property_instance_eq_float_test() {
    let property_name = r_string();

    let instance1 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(0.0));
    let instance2 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(0.0));
    assert!(instance1 == instance2);

    let instance1 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(1.0));
    let instance2 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(1.1));
    assert!(instance1 != instance2);
}

#[test]
fn reactive_property_instance_eq_string_test() {
    let property_name = r_string();
    let property_value = r_string();

    let instance1 = ReactivePropertyInstance::new(
        Uuid::new_v4(),
        property_name.clone(),
        json!(property_value.clone()),
    );
    let instance2 = ReactivePropertyInstance::new(
        Uuid::new_v4(),
        property_name.clone(),
        json!(property_value.clone()),
    );
    assert!(instance1 == instance2);

    let instance1 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(r_string()));
    let instance2 =
        ReactivePropertyInstance::new(Uuid::new_v4(), property_name.clone(), json!(r_string()));
    assert!(instance1 != instance2);
}
