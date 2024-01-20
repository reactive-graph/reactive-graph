use std::sync::Arc;

use inexor_rgf_plugin_api::Plugin;
use inexor_rgf_plugin_api::PluginContext;

use crate::PluginProxy;

pub(crate) struct PluginRegistrar {
    pub(crate) plugin: Option<Arc<PluginProxy>>,
    pub(crate) context: Arc<dyn PluginContext + Send + Sync>,
}

impl PluginRegistrar {
    pub fn new(context: Arc<dyn PluginContext + Send + Sync>) -> PluginRegistrar {
        PluginRegistrar { plugin: None, context }
    }
}

impl inexor_rgf_plugin_api::PluginRegistrar for PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<Arc<dyn Plugin>>) {
        self.plugin = Some(Arc::new(PluginProxy { plugin }));
    }

    fn context(&self) -> Arc<dyn PluginContext + Send + Sync> {
        self.context.clone()
    }
}
