use async_graphql::dynamic::TypeRef;
use std::sync::LazyLock;

pub static TYPE_REF_ID: LazyLock<TypeRef> = LazyLock::new(|| TypeRef::named_nn(TypeRef::ID));
pub static TYPE_REF_LIST_OF_IDS: LazyLock<TypeRef> = LazyLock::new(|| TypeRef::named_nn_list_nn(TypeRef::ID));
pub static TYPE_REF_JSON: LazyLock<TypeRef> = LazyLock::new(|| TypeRef::named_nn("JSON"));
pub static TYPE_REF_LIST_OF_JSONS: LazyLock<TypeRef> = LazyLock::new(|| TypeRef::named_nn_list_nn("JSON"));
