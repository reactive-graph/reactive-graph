use async_graphql::dynamic::TypeRef;

use crate::type_ref::TYPE_REF_JSON;
use crate::type_ref::TYPE_REF_LIST_OF_JSONS;
use reactive_graph_graph::DataType;
use reactive_graph_graph::Mutability::Immutable;
use reactive_graph_graph::PropertyType;

/// Maps the data type of property to the GraphQL type ref.
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
            DataType::Array => Some(TYPE_REF_LIST_OF_JSONS.clone()),
            DataType::Object => Some(TYPE_REF_JSON.clone()),
            DataType::Any => Some(TYPE_REF_JSON.clone()),
        }
    }
}
