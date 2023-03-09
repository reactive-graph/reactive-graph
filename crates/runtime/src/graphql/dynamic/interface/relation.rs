use async_graphql::dynamic::Interface;
use async_graphql::dynamic::InterfaceField;
use async_graphql::dynamic::TypeRef;

pub const INTERFACE_RELATION: &str = "Relation";
pub const INTERFACE_RELATION_FIELD_KEY: &str = "key";
pub const INTERFACE_RELATION_FIELD_INSTANCE_ID: &str = "instance_id";

pub fn get_relation_interface() -> Interface {
    Interface::new(INTERFACE_RELATION)
        .description("Relations have a outbound entity and a inbound entity as well as components and properties,")
        .field(InterfaceField::new(INTERFACE_RELATION_FIELD_KEY, TypeRef::named_nn(TypeRef::ID)))
        .field(InterfaceField::new(INTERFACE_RELATION_FIELD_INSTANCE_ID, TypeRef::named_nn(TypeRef::ID)))
}
