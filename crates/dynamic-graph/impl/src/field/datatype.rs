use async_graphql::dynamic::TypeRef;

use inexor_rgf_graph::DataType;
use inexor_rgf_graph::Mutability::Immutable;
use inexor_rgf_graph::PropertyType;

/// Maps the data type of a property to the GraphQL type ref.
pub fn to_type_ref(data_type: &DataType) -> TypeRef {
    match data_type {
        DataType::Null => TypeRef::named(TypeRef::STRING),
        DataType::Bool => TypeRef::named(TypeRef::BOOLEAN),
        DataType::Number => TypeRef::named(TypeRef::FLOAT), // TODO: NUMBER
        DataType::String => TypeRef::named(TypeRef::STRING),
        // Array = scalar JSON
        DataType::Array => TypeRef::named("JSON"),
        // Object = scalar JSON
        DataType::Object => TypeRef::named("JSON"),
        // Any = any data type
        DataType::Any => TypeRef::named("JSON"),
    }
}

pub fn to_input_type_ref(property: &PropertyType, is_optional: bool) -> Option<TypeRef> {
    if is_optional || property.mutability == Immutable {
        match property.data_type {
            DataType::Null => None,
            DataType::Bool => Some(TypeRef::named(TypeRef::BOOLEAN)),
            DataType::Number => Some(TypeRef::named(TypeRef::FLOAT)), // TODO: "Numeric" (oneof)
            DataType::String => Some(TypeRef::named(TypeRef::STRING)),
            DataType::Array => Some(TypeRef::named_nn_list("JSON")),
            DataType::Object => Some(TypeRef::named("JSON")),
            DataType::Any => Some(TypeRef::named("JSON")),
        }
    } else {
        match property.data_type {
            DataType::Null => None,
            DataType::Bool => Some(TypeRef::named_nn(TypeRef::BOOLEAN)),
            DataType::Number => Some(TypeRef::named_nn(TypeRef::FLOAT)), // TODO: "Numeric" (oneof)
            DataType::String => Some(TypeRef::named_nn(TypeRef::STRING)),
            DataType::Array => Some(TypeRef::named_nn_list_nn("JSON")),
            DataType::Object => Some(TypeRef::named_nn("JSON")),
            DataType::Any => Some(TypeRef::named_nn("JSON")),
        }
    }
}
