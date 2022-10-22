use std::sync::Arc;

use crate::ComponentManager;
use crate::EntityInstanceManager;
use crate::EntityTypeManager;
use crate::FlowInstanceManager;
use crate::FlowTypeManager;
use crate::GraphQLQueryService;
use crate::Plugin;
use crate::PluginContext;
use crate::RelationInstanceManager;
use crate::RelationTypeManager;
use crate::SystemEventManager;

/// Fake plugin
struct TestPlugin {}
impl Plugin for TestPlugin {}

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

    fn get_system_event_manager(&self) -> Arc<dyn SystemEventManager> {
        panic!();
    }
}

#[test]
fn plugin_api_default_trait_impl_test() {
    let plugin = TestPlugin {};
    let context = TestPluginContext {};
    assert_eq!(true, plugin.activate().is_ok());
    assert_eq!(true, plugin.deactivate().is_ok());
    assert_eq!(true, plugin.set_context(Arc::new(context)).is_ok());
    assert_eq!(true, plugin.remove_context().is_ok());
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
