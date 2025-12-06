pub use namespace::*;

use async_graphql::dynamic::Field;
use async_graphql::dynamic::Object;

pub mod create;
pub mod delete;
pub mod entity;
pub mod export;
pub mod flow;
pub mod json;
pub mod namespace;
pub mod property;
pub mod property_instance;
pub mod reactive_instance;
pub mod relation;
pub mod trigger;
pub mod update;

pub fn optional_field_to_vec(field: Option<Field>) -> Vec<Field> {
    match field {
        Some(field) => vec![field],
        None => vec![],
    }
}

pub fn add_optional_field(object: Object, field: Option<Field>) -> Object {
    if let Some(field) = field {
        return object.field(field);
    }
    object
}
