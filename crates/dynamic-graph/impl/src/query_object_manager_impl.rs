use std::sync::Arc;

use async_graphql::dynamic::Object;
use async_trait::async_trait;
use reactive_graph_dynamic_graph_api::EntityQueryObjectFactory;
use reactive_graph_dynamic_graph_api::QueryObjectManager;
use reactive_graph_dynamic_graph_api::flow_query_object_factory::FlowQueryObjectFactory;
use reactive_graph_dynamic_graph_api::relation_query_object_factory::RelationQueryObjectFactory;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::Component;
use springtime_di::component_alias;

#[derive(Component)]
pub struct QueryObjectManagerImpl {
    entity_query_object_factory: Arc<dyn EntityQueryObjectFactory + Send + Sync>,
    relation_query_object_factory: Arc<dyn RelationQueryObjectFactory + Send + Sync>,
    flow_query_object_factory: Arc<dyn FlowQueryObjectFactory + Send + Sync>,
}

impl QueryObjectManagerImpl {}

#[async_trait]
#[component_alias]
impl QueryObjectManager for QueryObjectManagerImpl {
    fn get_query_objects(&self) -> Vec<Object> {
        let mut query_objects = Vec::new();
        query_objects.append(&mut self.entity_query_object_factory.create_query_objects());
        query_objects.append(&mut self.relation_query_object_factory.create_query_objects());
        query_objects.append(&mut self.flow_query_object_factory.create_query_objects());
        query_objects
    }
}

#[async_trait]
impl Lifecycle for QueryObjectManagerImpl {}
