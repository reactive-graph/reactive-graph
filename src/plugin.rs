use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::plugin_context::PluginContext;
use crate::ComponentBehaviourProvider;
use crate::ComponentProvider;
use crate::EntityBehaviourProvider;
use crate::EntityTypeProvider;
use crate::FlowProvider;
use crate::RelationBehaviourProvider;
use crate::RelationTypeProvider;
use crate::WebResourceProvider;

#[derive(Debug)]
pub enum PluginError {
    NoComponentProvider,
    NoEntityTypeProvider,
    NoRelationTypeProvider,
    NoComponentBehaviourProvider,
    NoEntityBehaviourProvider,
    NoRelationBehaviourProvider,
    NoFlowProvider,
    NoWebResourceProvider,
    PluginCreationError,
    InitializationError,
    PostInitializationError,
    PreShutdownError,
    ShutdownError,
    Other { message: String },
}

impl<S: ToString> From<S> for PluginError {
    fn from(other: S) -> PluginError {
        PluginError::Other { message: other.to_string() }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
}

pub trait Plugin: Send + Sync {
    fn metadata(&self) -> Result<PluginMetadata, PluginError>;

    fn init(&self) -> Result<(), PluginError>;

    fn post_init(&self) -> Result<(), PluginError>;

    fn pre_shutdown(&self) -> Result<(), PluginError>;

    fn shutdown(&self) -> Result<(), PluginError>;

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError>;

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError>;

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError>;

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError>;

    fn get_component_behaviour_provider(&self) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError>;

    fn get_entity_behaviour_provider(&self) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError>;

    fn get_relation_behaviour_provider(&self) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError>;

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError>;

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError>;
}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub inexor_rgf_plugin_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn register_plugin(&mut self, name: &str, plugin: Box<Arc<dyn Plugin>>);
}
