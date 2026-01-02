use crate::RootObjectType;
use async_graphql::dynamic::Field;
use async_trait::async_trait;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::RelationType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait RelationQueryFieldFactory: Send + Sync + Lifecycle {
    fn create_query_fields(&self, namespace: &Namespace) -> Vec<Field>;
    fn create_query_field(&self, relation_type: &RelationType, root_object_type: RootObjectType) -> Field;
}
