use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_graph::EntityType;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

#[injectable]
#[async_trait]
pub trait EntityQueryObjectFactory: Send + Sync + Lifecycle {
    fn create_query_objects(&self) -> Vec<Object>;

    fn create_query_object(&self, entity_type: EntityType) -> Object;
}
