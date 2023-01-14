use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;
use convert_case::Case::Camel;
use convert_case::Case::Pascal;
use convert_case::Casing;

pub fn namespace_field_name(namespace: &String) -> String {
    namespace.to_case(Camel)
}

pub fn namespace_type_name(namespace: &String) -> String {
    format!("{}Namespace", namespace.to_case(Pascal))
}

pub fn namespace_mutation_type_name(namespace: &String) -> String {
    format!("{}Mutations", namespace.to_case(Pascal))
}

pub fn namespace_type_ref(namespace: &String) -> TypeRef {
    TypeRef::named_nn(namespace_type_name(namespace))
}

pub fn namespace_mutation_type_ref(namespace: &String) -> TypeRef {
    TypeRef::named_nn(namespace_mutation_type_name(namespace))
}

pub fn namespace_query_field(namespace: &String) -> Field {
    Field::new(namespace_field_name(namespace), namespace_type_ref(namespace), |_ctx| {
        FieldFuture::new(async move {
            let v = FieldValue::value("test");
            Ok(Some(v))
        })
    })
}

pub fn namespace_mutation_field(namespace: &String) -> Field {
    Field::new(namespace_field_name(namespace), namespace_mutation_type_ref(namespace), |_ctx| {
        FieldFuture::new(async move {
            let v = FieldValue::value("test");
            Ok(Some(v))
        })
    })
}
