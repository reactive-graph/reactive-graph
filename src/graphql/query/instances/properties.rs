use async_graphql::scalar;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The named property stores a value/document as JSON representation.
///
/// Each property is represented by it's name (String) and it's value. The value is
/// a representation of a JSON value/document. Therefore the value can be boolean,
/// number, string, array or an object. For more information about the data types
/// please look at https://docs.serde.rs/serde_json/value/enum.Value.html
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GraphQLPropertyInstance {
    /// The name of the property.
    pub name: String,

    /// The value of the property as JSON representation.
    pub value: Value,
}
scalar!(GraphQLPropertyInstance, "Property");
