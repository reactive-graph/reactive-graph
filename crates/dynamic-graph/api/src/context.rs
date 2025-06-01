use std::sync::Arc;

use reactive_graph_type_system_api::ComponentManager;
use reactive_graph_type_system_api::EntityTypeManager;
use reactive_graph_type_system_api::FlowTypeManager;
use reactive_graph_type_system_api::NamespaceManager;
use reactive_graph_type_system_api::RelationTypeManager;

#[derive(Clone)]
pub struct SchemaBuilderContext {
    pub namespace_manager: Arc<dyn NamespaceManager + Send + Sync>,
    pub component_manager: Arc<dyn ComponentManager + Send + Sync>,
    pub entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    pub relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    pub flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
}

impl SchemaBuilderContext {
    pub fn new(
        namespace_manager: Arc<dyn NamespaceManager + Send + Sync>,
        component_manager: Arc<dyn ComponentManager + Send + Sync>,
        entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
        relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
        flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
    ) -> SchemaBuilderContext {
        SchemaBuilderContext {
            namespace_manager,
            component_manager,
            entity_type_manager,
            relation_type_manager,
            flow_type_manager,
        }
    }
}
