use crate::RootObjectType;
use async_graphql::dynamic::Field;
use async_trait::async_trait;
use reactive_graph_graph::Component;
use reactive_graph_graph::Namespace;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait ComponentQueryFieldFactory: Send + Sync + Lifecycle {
    /// Creates the query fields for components in the given namespace.
    fn create_query_fields(&self, namespace: &Namespace) -> Vec<Field>;

    /// Creates the query field for the given component.
    fn create_query_field(&self, component: &Component, interface_root_object_type: RootObjectType, instance_root_object_type: RootObjectType) -> Field;
}
