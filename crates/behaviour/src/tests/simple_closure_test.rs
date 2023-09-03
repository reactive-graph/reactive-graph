// TODO: move unit test to plugin
// TODO: simple closure has been renamed probably

use std::sync::{Arc, RwLock};

use serde_json::{json, Value};

use crate::model::PropertyInstanceSetter;
use crate::entity::simple_closure::{SimpleClosureProperties, SimpleClosure, SimpleClosureReactiveEntityFactory as Factory};
use std::ops::{Deref, DerefMut};
use crate::model::ReactiveEntityFactory;

#[test]
fn simple_closure_test () {

    // We want that our closure writes into this variable
    let value: Arc<RwLock<i64>> = Arc::new(RwLock::new(0));

    // The type name is irrelevant
    let type_name = "simple_closure";

    // Construct an entity instance with one property named "input"
    let entity = Factory::new(type_name);

    // Go into scope
    {
        // Increase Reference Counter (which can be used in the closure
        let closure_value = value.clone();

        // Define a closure (within this scope)
        // The closure writes into "closure_value".
        // "closure_value" points to the same memory location as "value"
        // and is protected by a RwLock.
        let closure = move | v: &Value | {
            *closure_value.write().unwrap().deref_mut() = v.as_i64().unwrap();
        };

        // Now create the "SimpleClosure" behaviour on top of the entity instance
        // Pass the closure to the behaviour
        let simple_closure = SimpleClosure::new(entity.clone(), Box::new(closure));

        // Check that the type name is correct
        assert_eq!(type_name, simple_closure.type_name());

        // Check that the handle id is not zero
        assert_ne!(0, simple_closure.handle_id);

        // Check that the value is still the initial value
        assert_eq!(0, *value.read().unwrap().deref());

        // Set the entity instance value to another value
        entity.set(SimpleClosureProperties::INPUT.to_string(), json!(10));

        // Check that the value has changed
        // That means that the closure has modified the value
        assert_eq!(10, *value.read().unwrap().deref());
    } // simple_closure goes out of scope!

    // Set the entity instance value to yet another value
    entity.set(SimpleClosureProperties::INPUT.to_string(), json!(20));

    // Check that the value is still the same
    // Because the simple_closure has gone out of scope
    // That means the closure has been removed as subscriber of the entity instance property
    assert_eq!(10, *value.read().unwrap().deref());

    // Congrats!
}
