use async_graphql::dynamic::TypeRef;

use crate::model::DataType;

/// Maps the data type of a property to the GraphQL type ref.
pub fn to_type_ref(data_type: &DataType) -> TypeRef {
    match data_type {
        DataType::Null => TypeRef::named(TypeRef::STRING),
        DataType::Bool => TypeRef::named(TypeRef::BOOLEAN),
        DataType::Number => TypeRef::named(TypeRef::FLOAT), // TODO TypeRef::NUMBER???
        DataType::String => TypeRef::named(TypeRef::STRING),
        // TODO: Scalar JSON
        DataType::Array => TypeRef::named_list_nn(TypeRef::STRING),
        // TODO: Scalar JSON
        DataType::Object => TypeRef::named(TypeRef::STRING),
        // TODO: Scalar JSON
        DataType::Any => TypeRef::named(TypeRef::STRING),
    }
}
