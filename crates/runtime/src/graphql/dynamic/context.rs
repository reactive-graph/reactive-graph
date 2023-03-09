use std::sync::Arc;

use crate::api::ComponentManager;
use crate::api::EntityTypeManager;
use crate::api::NamespaceManager;
use crate::api::RelationTypeManager;

#[derive(Clone)]
pub struct SchemaBuilderContext {
    pub namespace_manager: Arc<dyn NamespaceManager>,
    pub component_manager: Arc<dyn ComponentManager>,
    pub entity_type_manager: Arc<dyn EntityTypeManager>,
    pub relation_type_manager: Arc<dyn RelationTypeManager>,
}

impl SchemaBuilderContext {
    pub fn new(
        namespace_manager: Arc<dyn NamespaceManager>,
        component_manager: Arc<dyn ComponentManager>,
        entity_type_manager: Arc<dyn EntityTypeManager>,
        relation_type_manager: Arc<dyn RelationTypeManager>,
    ) -> SchemaBuilderContext {
        SchemaBuilderContext {
            namespace_manager,
            component_manager,
            entity_type_manager,
            relation_type_manager,
        }
    }
}
