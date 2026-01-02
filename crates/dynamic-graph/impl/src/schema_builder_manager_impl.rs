use async_graphql::dynamic::Schema;
use async_graphql::dynamic::SchemaBuilder;
use async_trait::async_trait;
use reactive_graph_dynamic_graph_api::SchemaBuilderManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_reactive_service_api::ReactiveRelationManager;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespacedTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct SchemaBuilderManagerImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,

    namespaced_type_manager: Arc<dyn NamespacedTypeManager + Send + Sync>,

    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,

    reactive_flow_manager: Arc<dyn ReactiveFlowManager + Send + Sync>,

    reactive_relation_manager: Arc<dyn ReactiveRelationManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
}

impl SchemaBuilderManagerImpl {}

#[async_trait]
#[component_alias]
impl SchemaBuilderManager for SchemaBuilderManagerImpl {
    fn get_schema_builder(&self) -> SchemaBuilder {
        Schema::build("Query", Some("Mutation"), None)
            .data(self.component_manager.clone())
            .data(self.entity_type_manager.clone())
            .data(self.flow_type_manager.clone())
            .data(self.namespaced_type_manager.clone())
            .data(self.reactive_entity_manager.clone())
            .data(self.reactive_relation_manager.clone())
            .data(self.reactive_flow_manager.clone())
            .data(self.relation_type_manager.clone())
    }
}

#[async_trait]
impl Lifecycle for SchemaBuilderManagerImpl {}
