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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub depends_on: Vec<String>,
}

#[macro_export]
macro_rules! plugin_metadata {
    ($( $dependency:expr ),*) => {{
        let mut depends_on = Vec::new();
        $(
            depends_on.push(String::from($dependency));
        )*
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            depends_on,
        })
    }};
}

#[macro_export]
macro_rules! component_provider {
    ($component_provider:expr) => {{
        let component_provider = $component_provider.clone();
        let component_provider: Result<Arc<dyn ComponentProvider>, _> = <dyn query_interface::Object>::query_arc(component_provider);
        if component_provider.is_err() {
            return Err(ComponentProviderError::InitializationError);
        }
        Ok(component_provider.ok())
    }};
}

#[macro_export]
macro_rules! entity_type_provider {
    ($entity_type_provider:expr) => {{
        let entity_type_provider = $entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> = <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(EntityTypeProviderError::InitializationError);
        }
        Ok(entity_type_provider.ok())
    }};
}

#[macro_export]
macro_rules! relation_type_provider {
    ($relation_type_provider:expr) => {{
        let relation_type_provider = $relation_type_provider.clone();
        let relation_type_provider: Result<Arc<dyn RelationTypeProvider>, _> = <dyn query_interface::Object>::query_arc(relation_type_provider);
        if relation_type_provider.is_err() {
            return Err(RelationTypeProviderError::InitializationError);
        }
        Ok(relation_type_provider.ok())
    }};
}

#[macro_export]
macro_rules! flow_type_provider {
    ($flow_type_provider:expr) => {{
        let flow_type_provider = $flow_type_provider.clone();
        let flow_type_provider: Result<Arc<dyn FlowTypeProvider>, _> = <dyn query_interface::Object>::query_arc(flow_type_provider);
        if flow_type_provider.is_err() {
            return Err(FlowTypeProviderError::InitializationError);
        }
        Ok(flow_type_provider.ok())
    }};
}

#[macro_export]
macro_rules! component_behaviour_provider {
    ($component_behaviour_provider:expr) => {{
        let component_behaviour_provider = $component_behaviour_provider.clone();
        let component_behaviour_provider: Result<Arc<dyn ComponentBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(component_behaviour_provider);
        if component_behaviour_provider.is_err() {
            return Err(ComponentBehaviourProviderError::InitializationError);
        }
        Ok(component_behaviour_provider.ok())
    }};
}

#[macro_export]
macro_rules! entity_behaviour_provider {
    ($entity_behaviour_provider:expr) => {{
        let entity_behaviour_provider = $entity_behaviour_provider.clone();
        let entity_behaviour_provider: Result<Arc<dyn EntityBehaviourProvider>, _> = <dyn query_interface::Object>::query_arc(entity_behaviour_provider);
        if entity_behaviour_provider.is_err() {
            return Err(EntityBehaviourProviderError::InitializationError);
        }
        Ok(entity_behaviour_provider.ok())
    }};
}

#[macro_export]
macro_rules! relation_behaviour_provider {
    ($relation_behaviour_provider:expr) => {{
        let relation_behaviour_provider = $relation_behaviour_provider.clone();
        let relation_behaviour_provider: Result<Arc<dyn RelationBehaviourProvider>, _> = <dyn query_interface::Object>::query_arc(relation_behaviour_provider);
        if relation_behaviour_provider.is_err() {
            return Err(RelationBehaviourProviderError::InitializationError);
        }
        Ok(relation_behaviour_provider.ok())
    }};
}

#[macro_export]
macro_rules! flow_instance_provider {
    ($flow_instance_provider:expr) => {{
        let flow_instance_provider = $flow_instance_provider.clone();
        let flow_instance_provider: Result<Arc<dyn FlowInstanceProvider>, _> = <dyn query_interface::Object>::query_arc(flow_instance_provider);
        if flow_instance_provider.is_err() {
            return Err(FlowInstanceProviderError::InitializationError);
        }
        Ok(flow_instance_provider.ok())
    }};
}

#[macro_export]
macro_rules! web_resource_provider {
    ($web_resource_provider:expr) => {{
        let web_resource_provider = $web_resource_provider.clone();
        let web_resource_provider: Result<Arc<dyn WebResourceProvider>, _> = <dyn query_interface::Object>::query_arc(web_resource_provider);
        if web_resource_provider.is_err() {
            return Err(WebResourceProviderError::InitializationError);
        }
        Ok(web_resource_provider.ok())
    }};
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
