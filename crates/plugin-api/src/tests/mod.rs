use std::sync::Arc;

use crate::CommandManager;
use crate::ComponentImportExportManager;
use crate::ComponentManager;
use crate::ComponentProviderRegistry;
use crate::ConfigManager;
use crate::EntityBehaviourRegistry;
use crate::EntityComponentBehaviourRegistry;
use crate::EntityInstanceManager;
use crate::EntityTypeImportExportManager;
use crate::EntityTypeManager;
use crate::EntityTypeProviderRegistry;
use crate::FlowInstanceManager;
use crate::FlowTypeImportExportManager;
use crate::FlowTypeManager;
use crate::FlowTypeProviderRegistry;
use crate::GraphQLQueryService;
use crate::Plugin;
use crate::PluginContext;
use crate::RelationBehaviourRegistry;
use crate::RelationComponentBehaviourRegistry;
use crate::RelationInstanceManager;
use crate::RelationTypeImportExportManager;
use crate::RelationTypeManager;
use crate::RelationTypeProviderRegistry;
use crate::SystemEventManager;
use crate::WebResourceManager;

/// Fake plugin
struct TestPlugin {}
impl Plugin for TestPlugin {}

struct TestPluginContext {}
impl PluginContext for TestPluginContext {
    fn get_component_manager(&self) -> Arc<dyn ComponentManager + Send + Sync> {
        panic!();
    }

    fn get_component_import_export_manager(&self) -> Arc<dyn ComponentImportExportManager + Send + Sync> {
        panic!();
    }

    fn get_component_provider_registry(&self) -> Arc<dyn ComponentProviderRegistry + Send + Sync> {
        panic!();
    }

    fn get_entity_type_manager(&self) -> Arc<dyn EntityTypeManager + Send + Sync> {
        panic!();
    }

    fn get_entity_type_import_export_manager(&self) -> Arc<dyn EntityTypeImportExportManager + Send + Sync> {
        panic!();
    }

    fn get_entity_type_provider_registry(&self) -> Arc<dyn EntityTypeProviderRegistry + Send + Sync> {
        panic!();
    }

    fn get_relation_type_manager(&self) -> Arc<dyn RelationTypeManager + Send + Sync> {
        panic!();
    }

    fn get_relation_type_import_export_manager(&self) -> Arc<dyn RelationTypeImportExportManager + Send + Sync> {
        panic!();
    }

    fn get_relation_type_provider_registry(&self) -> Arc<dyn RelationTypeProviderRegistry + Send + Sync> {
        panic!();
    }

    fn get_flow_type_manager(&self) -> Arc<dyn FlowTypeManager + Send + Sync> {
        panic!();
    }

    fn get_flow_type_import_export_manager(&self) -> Arc<dyn FlowTypeImportExportManager + Send + Sync> {
        panic!();
    }

    fn get_flow_type_provider_registry(&self) -> Arc<dyn FlowTypeProviderRegistry + Send + Sync> {
        panic!();
    }

    fn get_entity_instance_manager(&self) -> Arc<dyn EntityInstanceManager + Send + Sync> {
        panic!();
    }

    fn get_relation_instance_manager(&self) -> Arc<dyn RelationInstanceManager + Send + Sync> {
        panic!();
    }

    fn get_flow_instance_manager(&self) -> Arc<dyn FlowInstanceManager + Send + Sync> {
        panic!();
    }

    fn get_entity_behaviour_registry(&self) -> Arc<dyn EntityBehaviourRegistry + Send + Sync> {
        panic!();
    }

    fn get_entity_component_behaviour_registry(&self) -> Arc<dyn EntityComponentBehaviourRegistry + Send + Sync> {
        panic!();
    }

    fn get_relation_behaviour_registry(&self) -> Arc<dyn RelationBehaviourRegistry + Send + Sync> {
        panic!();
    }

    fn get_relation_component_behaviour_registry(&self) -> Arc<dyn RelationComponentBehaviourRegistry + Send + Sync> {
        panic!();
    }

    fn get_config_manager(&self) -> Arc<dyn ConfigManager + Send + Sync> {
        panic!();
    }

    fn get_graphql_query_service(&self) -> Arc<dyn GraphQLQueryService + Send + Sync> {
        panic!();
    }

    fn get_system_event_manager(&self) -> Arc<dyn SystemEventManager + Send + Sync> {
        panic!();
    }

    fn get_command_manager(&self) -> Arc<dyn CommandManager + Send + Sync> {
        panic!();
    }

    fn get_web_resource_manager(&self) -> Arc<dyn WebResourceManager + Send + Sync> {
        panic!();
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn plugin_api_default_trait_impl_test() {
    let plugin = TestPlugin {};
    assert_eq!(true, plugin.activate().await.is_ok());
    assert_eq!(true, plugin.deactivate().await.is_ok());
}
