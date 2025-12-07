use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::Namespace;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait QueryNamespaceObjectTreeFactory: Send + Sync + Lifecycle {
    fn get_namespace_objects(&self) -> Vec<Object>;

    fn create_namespace_object(&self, namespace: &Namespace) -> Object;

    fn create_root_object(&self) -> Object;
}
