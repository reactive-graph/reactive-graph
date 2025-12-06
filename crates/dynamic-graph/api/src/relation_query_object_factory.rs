use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::RelationType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait RelationQueryObjectFactory: Send + Sync + Lifecycle {
    fn create_query_objects(&self) -> Vec<Object>;

    fn create_query_object(&self, relation_type: RelationType) -> Object;
}
