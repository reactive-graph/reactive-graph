use std::sync::Arc;

use crate::plugin::proxy::PluginProxy;
use crate::plugins::Plugin;
use crate::plugins::PluginContext;

pub(crate) struct PluginRegistrar {
    pub(crate) plugin: Option<Arc<PluginProxy>>,
    pub(crate) context: Arc<dyn PluginContext + Send + Sync>,
}

impl PluginRegistrar {
    pub fn new(context: Arc<dyn PluginContext + Send + Sync>) -> PluginRegistrar {
        PluginRegistrar { plugin: None, context }
    }
}

impl crate::plugins::PluginRegistrar for PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<Arc<dyn Plugin>>) {
        self.plugin = Some(Arc::new(PluginProxy { plugin }));
    }

    fn context(&self) -> Arc<dyn PluginContext + Send + Sync> {
        self.context.clone()
    }
}
