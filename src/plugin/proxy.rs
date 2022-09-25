use inexor_rgf_core_plugins::plugin::{
    PluginInitializationError, PluginMetadataError, PluginPostInitializationError, PluginPreShutdownError, PluginShutdownError,
};
use inexor_rgf_core_plugins::{
    ComponentBehaviourProviderError, ComponentProviderError, EntityBehaviourProviderError, EntityTypeProviderError, FlowInstanceProviderError,
    FlowTypeProvider, FlowTypeProviderError, PluginContextInitializationError, RelationBehaviourProviderError, RelationTypeProviderError,
    WebResourceProviderError,
};
use std::sync::Arc;

use libloading::Library;

use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::{
    ComponentBehaviourProvider, ComponentProvider, EntityBehaviourProvider, EntityTypeProvider, FlowInstanceProvider, Plugin, RelationBehaviourProvider,
    RelationTypeProvider, WebResourceProvider,
};

/// A proxy object which wraps a [`Plugin`] and makes sure it can't outlive
/// the library it came from.
pub struct PluginProxy {
    pub(crate) plugin: Box<Arc<dyn Plugin>>,
    #[allow(dead_code)]
    pub(crate) lib: Arc<Library>,
}

impl Plugin for PluginProxy {
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError> {
        self.plugin.metadata()
    }

    fn init(&self) -> Result<(), PluginInitializationError> {
        self.plugin.init()
    }

    fn post_init(&self) -> Result<(), PluginPostInitializationError> {
        self.plugin.post_init()
    }

    fn pre_shutdown(&self) -> Result<(), PluginPreShutdownError> {
        self.plugin.pre_shutdown()
    }

    fn shutdown(&self) -> Result<(), PluginShutdownError> {
        self.plugin.shutdown()
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.plugin.set_context(context.clone())
    }

    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        self.plugin.get_component_provider()
    }

    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        self.plugin.get_entity_type_provider()
    }

    fn get_relation_type_provider(&self) -> Result<Option<Arc<dyn RelationTypeProvider>>, RelationTypeProviderError> {
        self.plugin.get_relation_type_provider()
    }

    fn get_flow_type_provider(&self) -> Result<Option<Arc<dyn FlowTypeProvider>>, FlowTypeProviderError> {
        self.plugin.get_flow_type_provider()
    }

    fn get_component_behaviour_provider(&self) -> Result<Option<Arc<dyn ComponentBehaviourProvider>>, ComponentBehaviourProviderError> {
        self.plugin.get_component_behaviour_provider()
    }

    fn get_entity_behaviour_provider(&self) -> Result<Option<Arc<dyn EntityBehaviourProvider>>, EntityBehaviourProviderError> {
        self.plugin.get_entity_behaviour_provider()
    }

    fn get_relation_behaviour_provider(&self) -> Result<Option<Arc<dyn RelationBehaviourProvider>>, RelationBehaviourProviderError> {
        self.plugin.get_relation_behaviour_provider()
    }

    fn get_flow_instance_provider(&self) -> Result<Option<Arc<dyn FlowInstanceProvider>>, FlowInstanceProviderError> {
        self.plugin.get_flow_instance_provider()
    }

    fn get_web_resource_provider(&self) -> Result<Option<Arc<dyn WebResourceProvider>>, WebResourceProviderError> {
        self.plugin.get_web_resource_provider()
    }
}
