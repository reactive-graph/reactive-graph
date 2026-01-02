use crate::object_type_name::namespace_type_ref;
use crate::type_ref::TYPE_REF_ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use convert_case::Case;
use convert_case::Casing;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::Namespace;

pub const NAMESPACE_FIELD_NAME: &str = "_namespace";

pub fn namespace_field_name(namespace: &Namespace) -> String {
    namespace
        .last_segment()
        .map(|segment| segment.as_ref().to_case(Case::Snake))
        .unwrap_or(namespace.fully_qualified_type_name())
}

/// Namespace object field in the parent object
pub fn namespace_field(namespace: Namespace, root_object_type: RootObjectType) -> Field {
    Field::new(namespace_field_name(&namespace), namespace_type_ref(&namespace, root_object_type), move |_| {
        FieldFuture::new({
            let namespace_inner = namespace.clone();
            async move { Ok(Some(FieldValue::value(namespace_inner.to_string()))) }
        })
    })
}

/// Field _namespace in the namespace object
pub fn namespace_path_field(namespace: Namespace) -> Field {
    let description = format!("The fully qualified namespace: {namespace}");
    Field::new(NAMESPACE_FIELD_NAME, TYPE_REF_ID.clone(), move |_| {
        FieldFuture::new({
            let namespace_inner = namespace.clone();
            async move { Ok(Some(FieldValue::value(namespace_inner.to_string()))) }
        })
    })
    .description(description)
}

pub fn root_object_namespace_path_field() -> Field {
    Field::new(NAMESPACE_FIELD_NAME, TYPE_REF_ID.clone(), move |_ctx| {
        FieldFuture::new(async move { Ok(Some(FieldValue::value(""))) })
    })
    .description("Root namespace")
}

// use async_graphql::dynamic::Field;
// use async_graphql::dynamic::FieldFuture;
// use async_graphql::dynamic::FieldValue;
// use async_graphql::dynamic::TypeRef;
// use convert_case::Case::Camel;
// use convert_case::Case::Pascal;
// use convert_case::Casing;
// use reactive_graph_graph::NamespacedType;
//
// pub fn namespace_field_name(namespace: &NamespacedType) -> String {
//     namespace.namespace().to_string().to_case(Camel)
// }
//
// pub fn namespace_type_name(namespace: &NamespacedType) -> String {
//     format!("{}Namespace", namespace.namespace().to_string().to_case(Pascal))
// }
//
// pub fn namespace_mutation_type_name(namespace: &NamespacedType) -> String {
//     format!("{}Mutations", namespace.namespace().to_string().to_case(Pascal))
// }
//
// pub fn namespace_query_type_ref(namespace: &NamespacedType) -> TypeRef {
//     TypeRef::named_nn(namespace_type_name(namespace))
// }
//
// pub fn namespace_mutation_type_ref(namespace: &NamespacedType) -> TypeRef {
//     TypeRef::named_nn(namespace_mutation_type_name(namespace))
// }
//
// pub fn namespace_query_field(namespace: &NamespacedType) -> Field {
//     let namespace_inner = namespace.clone();
//     Field::new(namespace_field_name(namespace), namespace_query_type_ref(namespace), move |_ctx| {
//         let namespace = namespace_inner.clone();
//         FieldFuture::new(async move { Ok(Some(FieldValue::value(namespace.namespace().to_string()))) })
//     })
// }
//
// pub fn namespace_mutation_field(namespace: &NamespacedType) -> Field {
//     let namespace_inner = namespace.clone();
//     Field::new(namespace_field_name(namespace), namespace_mutation_type_ref(namespace), move |_ctx| {
//         let namespace = namespace_inner.clone();
//         FieldFuture::new(async move { Ok(Some(FieldValue::value(namespace.namespace().to_string()))) })
//     })
// }
