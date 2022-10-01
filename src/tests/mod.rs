use crate::plugin::PluginMetadataError;
use crate::PluginMetadata;
use crate::{
    ComponentManager, EntityInstanceManager, EntityTypeManager, FlowInstanceManager, FlowTypeManager, GraphQLQueryService, Plugin, PluginContext,
    RelationInstanceManager, RelationTypeManager,
};
use std::sync::Arc;

/// Fake plugin
struct TestPlugin {}
impl Plugin for TestPlugin {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        Err(PluginMetadataError::InvalidMetadataError)
    }
}

struct TestPluginContext {}
impl PluginContext for TestPluginContext {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager> {
        panic!();
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager> {
        panic!();
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager> {
        panic!();
    }

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager> {
        panic!();
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager> {
        panic!();
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager> {
        panic!();
    }

    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager> {
        panic!();
    }

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService> {
        panic!();
    }
}

#[test]
fn plugin_api_default_trait_impl_test() {
    let plugin = TestPlugin {};
    let context = TestPluginContext {};
    assert_eq!(true, plugin.init().is_ok());
    assert_eq!(true, plugin.post_init().is_ok());
    assert_eq!(true, plugin.pre_shutdown().is_ok());
    assert_eq!(true, plugin.shutdown().is_ok());
    assert_eq!(true, plugin.set_context(Arc::new(context)).is_ok());
    assert_eq!(true, plugin.get_component_behaviour_provider().is_ok());
    assert_eq!(true, plugin.get_entity_type_provider().is_ok());
    assert_eq!(true, plugin.get_relation_type_provider().is_ok());
    assert_eq!(true, plugin.get_flow_type_provider().is_ok());
    assert_eq!(true, plugin.get_component_behaviour_provider().is_ok());
    assert_eq!(true, plugin.get_entity_behaviour_provider().is_ok());
    assert_eq!(true, plugin.get_relation_behaviour_provider().is_ok());
    assert_eq!(true, plugin.get_flow_instance_provider().is_ok());
    assert_eq!(true, plugin.get_web_resource_provider().is_ok());
}
