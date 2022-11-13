use std::sync::Arc;

use crate::plugin_context::PluginContextDeinitializationError;
use crate::ComponentProvider;
use crate::ComponentProviderError;
use crate::EntityTypeProvider;
use crate::EntityTypeProviderError;
use crate::FlowInstanceProvider;
use crate::FlowInstanceProviderError;
use crate::FlowTypeProvider;
use crate::FlowTypeProviderError;
use crate::PluginContext;
use crate::PluginContextInitializationError;
use crate::RelationTypeProvider;
use crate::RelationTypeProviderError;
use crate::WebResourceProvider;
use crate::WebResourceProviderError;

#[derive(Debug)]
pub enum PluginLoadingError {
    LoadingDynamicLibraryFailed,
    CompilerVersionMismatch,
    PluginApiVersionMismatch,
    PluginContainerInitializationError,
    PluginDeclarationError { message: String },
}

impl<S: ToString> From<S> for PluginLoadingError {
    fn from(other: S) -> PluginLoadingError {
        PluginLoadingError::PluginDeclarationError { message: other.to_string() }
    }
}

#[derive(Debug)]
pub enum PluginUnloadingError {
    UnloadingFailed,
}

#[derive(Debug)]
pub enum PluginActivationError {
    // TODO: Add more specific error types
    ActivationFailed,
}

#[derive(Debug)]
pub enum PluginDeactivationError {
    // TODO: Add more specific error types
    DeactivationFailed,
}

pub trait Plugin: Send + Sync {
    /// Called on initialization of the plugin.
    fn activate(&self) -> Result<(), PluginActivationError> {
        Ok(())
    }

    /// Called on deactivation of the plugin.
    fn deactivate(&self) -> Result<(), PluginDeactivationError> {
        Ok(())
    }

    /// Injection setter for the plugin context.
    ///
    /// The plugin context provides access to the core services of the reactive graph flow.
    #[allow(unused_variables)]
    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
        Ok(())
    }

    fn remove_context(&self) -> Result<(), PluginContextDeinitializationError> {
        Ok(())
    }

    /// Returns a service which provides components.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: component_provider!(self.component_provider)
    fn get_component_provider(&self) -> Result<Option<Arc<dyn ComponentProvider>>, ComponentProviderError> {
        Ok(None)
    }

    /// Returns a service which provides entity types.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: entity_type_provider!(self.entity_type_provider)
    fn get_entity_type_provider(&self) -> Result<Option<Arc<dyn EntityTypeProvider>>, EntityTypeProviderError> {
        Ok(None)
    }

    /// Returns a service which provides relation types.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: relation_type_provider!(self.relation_type_provider)
    fn get_relation_type_provider(&self) -> Result<Option<Arc<dyn RelationTypeProvider>>, RelationTypeProviderError> {
        Ok(None)
    }

    /// Returns a service which provides flow types.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: flow_type_provider!(self.flow_type_provider)
    fn get_flow_type_provider(&self) -> Result<Option<Arc<dyn FlowTypeProvider>>, FlowTypeProviderError> {
        Ok(None)
    }

    /// Returns a service which provides flow instances.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: flow_instance_provider!(self.flow_instance_provider)
    fn get_flow_instance_provider(&self) -> Result<Option<Arc<dyn FlowInstanceProvider>>, FlowInstanceProviderError> {
        Ok(None)
    }

    /// Returns a provider service for web resources.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: web_resource_provider!(self.web_resource_provider)
    fn get_web_resource_provider(&self) -> Result<Option<Arc<dyn WebResourceProvider>>, WebResourceProviderError> {
        Ok(None)
    }
}
