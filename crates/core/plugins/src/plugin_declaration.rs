use std::sync::Arc;

use crate::Plugin;
use crate::PluginDependency;

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    /// The version of the rust compiler which has compiled the plugin. The version must match with the version the core application has been compiled with.
    pub rustc_version: &'static str,

    /// The version of plugin API. The version must match with the version of the plugin API used by the core application.
    pub plugin_api_version: &'static str,

    /// The name of the plugin.
    pub name: &'static str,

    /// The description of the plugin.
    pub description: &'static str,

    /// The version of the plugin.
    pub version: &'static str,

    /// The library registrar function.
    #[allow(improper_ctypes_definitions)]
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),

    /// Function to get the dependencies of the plugin.
    #[allow(improper_ctypes_definitions)]
    pub get_dependencies: unsafe extern "C" fn() -> Vec<PluginDependency>,
}

/// Contains the registration
pub trait PluginRegistrar {
    /// Registers the given plugin with the given name in the core application.
    fn register_plugin(&mut self, plugin: Box<Arc<dyn Plugin>>);
}
