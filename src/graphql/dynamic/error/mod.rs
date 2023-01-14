use async_graphql::Error;
use uuid::Uuid;

use crate::model::EntityTypeId;
use crate::model::PropertyType;

pub fn data_type_error(property: &PropertyType) -> Error {
    Error::new(format!("Invalid datatype: Property {} is of datatype {}!", &property.name, &property.data_type))
}

pub fn mutability_error(property: &PropertyType) -> Error {
    Error::new(format!("Can't update property {} which is {}!", &property.name, &property.mutability))
}

pub fn number_error(property: &PropertyType) -> Error {
    Error::new(format!("Can't update numeric property {} because input wasn't a number", &property.name))
}

pub fn entity_instance_not_found_error(id: &Uuid) -> Error {
    Error::new(format!("Entity instance with id {} not found", id))
}

pub fn entity_instance_not_of_entity_type_error(id: &Uuid, ty: &EntityTypeId) -> Error {
    Error::new(format!("Entity instance {} is not a {}", id, &ty))
}
