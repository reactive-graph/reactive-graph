use std::collections::HashMap;
use libloading::Library;
use std::sync::Arc;
use crate::plugin::proxy::PluginProxy;
use crate::plugins::Plugin;

pub(crate) struct PluginRegistrar {
    pub(crate) plugins: HashMap<String, PluginProxy>,
    pub(crate) lib: Arc<Library>,
}

impl PluginRegistrar {
    pub fn new(lib: Arc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            plugins: HashMap::default(),
        }
    }
}

impl inexor_rgf_core_plugins::PluginRegistrar for PluginRegistrar {
    fn register_plugin(&mut self, name: &str, plugin: Box<Arc<dyn Plugin>>) {
        let proxy = PluginProxy {
            plugin,
            lib: Arc::clone(&self.lib),
        };
        self.plugins.insert(name.to_string(), proxy);
    }
}
