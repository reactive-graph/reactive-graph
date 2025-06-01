use async_graphql::dynamic::Interface;
use async_graphql::dynamic::InterfaceField;
use async_graphql::dynamic::TypeRef;

pub const INTERFACE_FLOW: &str = "Flow";
pub const INTERFACE_FLOW_FIELD_ID: &str = "id";

pub fn get_flow_interface() -> Interface {
    Interface::new(INTERFACE_FLOW)
        .description("Flows have entities and relations.")
        .field(InterfaceField::new(INTERFACE_FLOW_FIELD_ID, TypeRef::named_nn(TypeRef::ID)))
}
