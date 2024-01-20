use std::sync::Arc;

use inexor_rgf_type_system_api::ComponentManager;
use inexor_rgf_type_system_api::EntityTypeManager;
use inexor_rgf_type_system_api::NamespaceManager;
use inexor_rgf_type_system_api::RelationTypeManager;

#[derive(Clone)]
pub struct SchemaBuilderContext {
    pub namespace_manager: Arc<dyn NamespaceManager + Send + Sync>,
    pub component_manager: Arc<dyn ComponentManager + Send + Sync>,
    pub entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
    pub relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
}

impl SchemaBuilderContext {
    pub fn new(
        namespace_manager: Arc<dyn NamespaceManager + Send + Sync>,
        component_manager: Arc<dyn ComponentManager + Send + Sync>,
        entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,
        relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
    ) -> SchemaBuilderContext {
        SchemaBuilderContext {
            namespace_manager,
            component_manager,
            entity_type_manager,
            relation_type_manager,
        }
    }
}
