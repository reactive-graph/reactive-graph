use async_graphql::dynamic::Interface;
use async_graphql::dynamic::InterfaceField;
use async_graphql::dynamic::TypeRef;

pub const INTERFACE_ENTITY: &str = "Entity";
pub const INTERFACE_ENTITY_FIELD_ID: &str = "id";

pub fn get_entity_interface() -> Interface {
    Interface::new(INTERFACE_ENTITY).field(InterfaceField::new(INTERFACE_ENTITY_FIELD_ID, TypeRef::named_nn(TypeRef::ID)))
}
