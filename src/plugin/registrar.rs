use std::sync::Arc;

use crate::plugin::proxy::PluginProxy;
use crate::plugins::Plugin;

pub(crate) struct PluginRegistrar {
    pub(crate) plugin: Option<Arc<PluginProxy>>,
}

impl PluginRegistrar {
    pub fn new() -> PluginRegistrar {
        PluginRegistrar { plugin: None }
    }
}

impl inexor_rgf_core_plugins::PluginRegistrar for PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<Arc<dyn Plugin>>) {
        self.plugin = Some(Arc::new(PluginProxy { plugin }));
    }
}
