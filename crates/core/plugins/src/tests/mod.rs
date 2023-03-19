use std::sync::Arc;

use crate::ComponentManager;
use crate::ConfigManager;
use crate::EntityBehaviourRegistry;
use crate::EntityComponentBehaviourRegistry;
use crate::EntityInstanceManager;
use crate::EntityTypeManager;
use crate::FlowInstanceManager;
use crate::FlowTypeManager;
use crate::GraphQLQueryService;
use crate::Plugin;
use crate::PluginContext;
use crate::RelationBehaviourRegistry;
use crate::RelationComponentBehaviourRegistry;
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

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry> {
        panic!();
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry> {
        panic!();
    }

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry> {
        panic!();
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry> {
        panic!();
    }

    fn get_config_manager(&self) -> Arc<dyn ConfigManager> {
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
    // Type providers
    assert_eq!(true, plugin.get_component_provider().is_ok());
    assert_eq!(true, plugin.get_entity_type_provider().is_ok());
    assert_eq!(true, plugin.get_relation_type_provider().is_ok());
    assert_eq!(true, plugin.get_flow_type_provider().is_ok());
    // Instance providers
    assert_eq!(true, plugin.get_flow_instance_provider().is_ok());
    assert_eq!(true, plugin.get_web_resource_provider().is_ok());
}
