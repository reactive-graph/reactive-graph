use async_graphql::dynamic::TypeRef;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespacedType;

#[inline]
pub fn object_type_name<T: Into<NamespacedType>>(ty: T, root_object_type: RootObjectType) -> String {
    format!("{}{root_object_type}", ty.into().fully_qualified_type_name())
}

#[inline]
pub fn object_type_ref<T: Into<NamespacedType>>(ty: T, root_object_type: RootObjectType) -> TypeRef {
    TypeRef::named_nn(object_type_name(ty, root_object_type))
}

#[inline]
pub fn object_type_ref_list<T: Into<NamespacedType>>(ty: T, root_object_type: RootObjectType) -> TypeRef {
    TypeRef::named_nn_list_nn(object_type_name(ty, root_object_type))
}

#[inline]
pub fn object_type_ref_return_list<T: Into<NamespacedType>>(ty: T) -> TypeRef {
    object_type_ref_list(ty, RootObjectType::Query)
}

#[inline]
pub fn namespace_type_name(namespace: &Namespace, root_object_type: RootObjectType) -> String {
    format!("{}{root_object_type}", namespace.fully_qualified_type_name())
}

#[inline]
pub fn namespace_type_ref(namespace: &Namespace, root_object_type: RootObjectType) -> TypeRef {
    TypeRef::named_nn(namespace_type_name(namespace, root_object_type))
}
