use async_graphql::dynamic::*;

pub mod creation;
pub mod mutation;
pub mod query;

pub mod id;

pub mod property;

pub mod outbound;

pub mod inbound;

fn optional_field_to_vec(field: Option<Field>) -> Vec<Field> {
    match field {
        Some(field) => vec![field],
        None => vec![],
    }
}
