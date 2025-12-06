use async_graphql::dynamic::Object;
use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

use reactive_graph_dynamic_graph_api::ComponentMutationObjectFactory;
use reactive_graph_dynamic_graph_api::EntityMutationObjectFactory;
use reactive_graph_dynamic_graph_api::FlowMutationObjectFactory;
use reactive_graph_dynamic_graph_api::MutationObjectManager;
use reactive_graph_dynamic_graph_api::RelationMutationObjectFactory;
use reactive_graph_lifecycle::Lifecycle;

#[derive(Component)]
pub struct MutationObjectManagerImpl {
    component_mutation_object_factory: Arc<dyn ComponentMutationObjectFactory + Send + Sync>,
    entity_mutation_object_factory: Arc<dyn EntityMutationObjectFactory + Send + Sync>,
    relation_mutation_object_factory: Arc<dyn RelationMutationObjectFactory + Send + Sync>,
    flow_mutation_object_factory: Arc<dyn FlowMutationObjectFactory + Send + Sync>,
}

impl MutationObjectManagerImpl {}

#[async_trait]
#[component_alias]
impl MutationObjectManager for MutationObjectManagerImpl {
    fn get_mutation_objects(&self) -> Vec<Object> {
        let mut query_objects = Vec::new();
        query_objects.append(&mut self.component_mutation_object_factory.create_mutation_objects());
        query_objects.append(&mut self.entity_mutation_object_factory.create_mutation_objects());
        query_objects.append(&mut self.relation_mutation_object_factory.create_mutation_objects());
        query_objects.append(&mut self.flow_mutation_object_factory.create_mutation_objects());
        query_objects
    }
}

#[async_trait]
impl Lifecycle for MutationObjectManagerImpl {}
