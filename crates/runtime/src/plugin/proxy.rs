use std::sync::Arc;

use async_trait::async_trait;

use crate::plugins::plugin_context::PluginContext;
use crate::plugins::ComponentProvider;
use crate::plugins::ComponentProviderError;
use crate::plugins::EntityTypeProvider;
use crate::plugins::EntityTypeProviderError;
use crate::plugins::FlowInstanceProvider;
use crate::plugins::FlowInstanceProviderError;
use crate::plugins::FlowTypeProvider;
use crate::plugins::FlowTypeProviderError;
use crate::plugins::Plugin;
use crate::plugins::PluginActivationError;
use crate::plugins::PluginContextDeinitializationError;
use crate::plugins::PluginContextInitializationError;
use crate::plugins::PluginDeactivationError;
use crate::plugins::RelationTypeProvider;
use crate::plugins::RelationTypeProviderError;
use crate::plugins::WebResourceProvider;
use crate::plugins::WebResourceProviderError;

/// A proxy object which wraps a [`Plugin`] and makes sure it can't outlive
/// the library it came from.
pub struct PluginProxy {
    pub(crate) plugin: Box<Arc<dyn Plugin>>,
}

#[async_trait]
impl Plugin for PluginProxy {
    async fn activate(&self) -> Result<(), PluginActivationError> {
        self.plugin.activate().await
    }

    async fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        self.plugin.deactivate().await
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        self.plugin.set_context(context.clone())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        self.plugin.remove_context()
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

    fn get_flow_instance_provider(&self) -> Result<Option<Arc<dyn FlowInstanceProvider>>, FlowInstanceProviderError> {
        self.plugin.get_flow_instance_provider()
    }

    fn get_web_resource_provider(&self) -> Result<Option<Arc<dyn WebResourceProvider>>, WebResourceProviderError> {
        self.plugin.get_web_resource_provider()
    }
}
