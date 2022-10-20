use std::sync::Arc;

use crate::ComponentBehaviourProviderError;
use serde::Deserialize;
use serde::Serialize;

use crate::ComponentBehaviourProvider;
use crate::ComponentProvider;
use crate::ComponentProviderError;
use crate::EntityBehaviourProvider;
use crate::EntityBehaviourProviderError;
use crate::EntityTypeProvider;
use crate::EntityTypeProviderError;
use crate::FlowInstanceProvider;
use crate::FlowInstanceProviderError;
use crate::FlowTypeProvider;
use crate::FlowTypeProviderError;
use crate::PluginContext;
use crate::PluginContextInitializationError;
use crate::RelationBehaviourProvider;
use crate::RelationBehaviourProviderError;
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
pub enum PluginMetadataError {
    InvalidMetadataError,
}

#[derive(Debug)]
pub enum PluginInitializationError {
    InitializationFailed,
}

#[derive(Debug)]
pub enum PluginPostInitializationError {
    PostInitializationFailed,
}

#[derive(Debug)]
pub enum PluginPreShutdownError {
    PreShutdownFailed,
}

#[derive(Debug)]
pub enum PluginShutdownError {
    ShutdownFailed,
}

pub trait Plugin: Send + Sync {
    /// Returns the metadata of the plugin.
    fn metadata(&self) -> Result<PluginMetadata, PluginMetadataError>;

    /// Called on initialization of the plugin.
    fn init(&self) -> Result<(), PluginInitializationError> {
        Ok(())
    }

    /// Called after initialization of the plugin.
    fn post_init(&self) -> Result<(), PluginPostInitializationError> {
        Ok(())
    }

    /// Called before shutdown of the plugin.
    fn pre_shutdown(&self) -> Result<(), PluginPreShutdownError> {
        Ok(())
    }

    /// Called on shutdown of the plugin.
    fn shutdown(&self) -> Result<(), PluginShutdownError> {
        Ok(())
    }

    /// Injection setter for the plugin context.
    ///
    /// The plugin context provides access to the core services of the reactive graph flow.
    #[allow(unused_variables)]
    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginContextInitializationError> {
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

    /// Returns a service which provides behaviours for reactive instances having a component.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: component_behaviour_provider!(self.component_behaviour_provider)
    fn get_component_behaviour_provider(&self) -> Result<Option<Arc<dyn ComponentBehaviourProvider>>, ComponentBehaviourProviderError> {
        Ok(None)
    }

    /// Returns a service which provides behaviours for reactive entity instances.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: entity_behaviour_provider!(self.entity_behaviour_provider)
    fn get_entity_behaviour_provider(&self) -> Result<Option<Arc<dyn EntityBehaviourProvider>>, EntityBehaviourProviderError> {
        Ok(None)
    }

    /// Returns a service which provides behaviours for reactive relation instances.
    ///
    /// This is not mandatory.
    /// A macro exists which reduces boilerplate code: relation_behaviour_provider!(self.relation_behaviour_provider)
    fn get_relation_behaviour_provider(&self) -> Result<Option<Arc<dyn RelationBehaviourProvider>>, RelationBehaviourProviderError> {
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

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    /// The version of the rust compiler which has compiled the plugin. The version must match with the version the core application has been compiled with.
    pub rustc_version: &'static str,

    /// The version of plugin API. The version must match with the version of the plugin API used by the core application.
    pub inexor_rgf_plugin_version: &'static str,

    /// The library registrar function.
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    /// Registers the given plugin with the given name in the core application.
    fn register_plugin(&mut self, name: &str, plugin: Box<Arc<dyn Plugin>>);
}
