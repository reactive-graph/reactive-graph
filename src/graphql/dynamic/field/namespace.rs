use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::TypeRef;

pub fn namespace_field(namespace: &String) -> Field {
    Field::new(namespace, TypeRef::named_nn(namespace), |_ctx| {
        FieldFuture::new(async move {
            let v = FieldValue::value("test");
            Ok(Some(v))
        })
    })
}
