use std::sync::Arc;

use crate::component_provider::ComponentProvider;
use crate::entity_behaviour_provider::EntityBehaviourProvider;
use crate::entity_type_provider::EntityTypeProvider;
use crate::flow_provider::FlowProvider;
use crate::relation_behaviour_provider::RelationBehaviourProvider;
use crate::relation_type_provider::RelationTypeProvider;

#[derive(Debug)]
pub enum PluginError {
    NoComponentProvider,
    NoEntityTypeProvider,
    NoRelationTypeProvider,
    NoEntityBehaviourProvider,
    NoRelationBehaviourProvider,
    NoFlowProvider,
    PluginCreationError,
    InitializationError,
    PostInitializationError,
    PreShutdownError,
    ShutdownError,
    Other { message: String },
}

impl<S: ToString> From<S> for PluginError {
    fn from(other: S) -> PluginError {
        PluginError::Other {
            message: other.to_string(),
        }
    }
}

// pub struct PluginMetadata {
//     name: String,
//     description: String,
//     version: String,
// }

pub trait Plugin: Send + Sync {
    // TODO: Additional metadata
    // fn metadata(&self) -> Result<Arc<dyn PluginMetadata>, PluginError>;
    fn init(&self) -> Result<(), PluginError>;

    fn post_init(&self) -> Result<(), PluginError>;

    fn pre_shutdown(&self) -> Result<(), PluginError>;

    fn shutdown(&self) -> Result<(), PluginError>;

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError>;

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError>;

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError>;

    fn get_entity_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError>;

    fn get_relation_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError>;

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError>;
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
