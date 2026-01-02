use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::TypeRef;

pub const QUERY_ARGUMENT_ID: &str = "id";
pub const QUERY_ARGUMENT_OUTBOUND_ID: &str = "outboundId";
pub const QUERY_ARGUMENT_INBOUND_ID: &str = "inboundId";
pub const QUERY_ARGUMENT_INSTANCE_ID: &str = "instanceId";
pub const QUERY_ARGUMENT_IDS: &str = "ids";
pub const QUERY_ARGUMENT_LABEL: &str = "label";

#[inline]
pub fn query_argument_id() -> InputValue {
    InputValue::new(QUERY_ARGUMENT_ID, TypeRef::named(TypeRef::ID))
}

#[inline]
pub fn query_argument_ids() -> InputValue {
    InputValue::new(QUERY_ARGUMENT_IDS, TypeRef::named_nn_list(TypeRef::ID))
}

#[inline]
pub fn query_argument_label() -> InputValue {
    InputValue::new(QUERY_ARGUMENT_LABEL, TypeRef::named(TypeRef::STRING))
}

#[inline]
pub fn query_argument_outbound_id() -> InputValue {
    InputValue::new(QUERY_ARGUMENT_OUTBOUND_ID, TypeRef::named(TypeRef::ID))
}

#[inline]
pub fn query_argument_inbound_id() -> InputValue {
    InputValue::new(QUERY_ARGUMENT_INBOUND_ID, TypeRef::named(TypeRef::ID))
}

#[inline]
pub fn query_argument_instance_id() -> InputValue {
    InputValue::new(QUERY_ARGUMENT_INSTANCE_ID, TypeRef::named(TypeRef::ID))
}
