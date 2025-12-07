use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

use reactive_graph_dynamic_graph_api::SchemaBuilderContext;
use reactive_graph_dynamic_graph_api::SchemaBuilderContextManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespacedTypeManager;
use reactive_graph_type_system_api::RelationTypeManager;

#[derive(Component)]
pub struct SchemaBuilderContextManagerImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,

    namespaced_type_manager: Arc<dyn NamespacedTypeManager + Send + Sync>,
}

impl SchemaBuilderContextManagerImpl {}

#[async_trait]
#[component_alias]
impl SchemaBuilderContextManager for SchemaBuilderContextManagerImpl {
    fn get_schema_builder_context(&self) -> SchemaBuilderContext {
        SchemaBuilderContext::new(
            self.namespaced_type_manager.clone(),
            self.component_manager.clone(),
            self.entity_type_manager.clone(),
            self.relation_type_manager.clone(),
            self.flow_type_manager.clone(),
        )
    }
}

#[async_trait]
impl Lifecycle for SchemaBuilderContextManagerImpl {}
