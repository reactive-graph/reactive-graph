use clap::Parser;

#[cfg(target_os = "linux")]
#[derive(Parser, Debug)]
pub struct PluginsArguments {
    // Plugins
    /// If true, all plugins will be disabled.
    #[arg(short = 'x', long, env = "REACTIVE_GRAPH_DISABLE_ALL_PLUGINS")]
    pub(crate) disable_all_plugins: Option<bool>,

    /// The list of plugins to disable.
    #[arg(short = 'p', long)]
    pub(crate) disabled_plugins: Option<Vec<String>>,

    /// The list of plugins to enable.
    #[arg(short = 'P', long)]
    pub(crate) enabled_plugins: Option<Vec<String>>,

    /// If true, hot deployment will be disabled.
    #[arg(long, env = "REACTIVE_GRAPH_DISABLE_HOT_DEPLOY")]
    pub(crate) disable_hot_deploy: Option<bool>,

    /// The folder which is watched for hot deployment.
    #[arg(long, env = "REACTIVE_GRAPH_HOT_DEPLOY_LOCATION")]
    pub(crate) hot_deploy_location: Option<String>,

    /// The folder which plugins are installed permanently.
    #[arg(long, env = "REACTIVE_GRAPH_INSTALL_LOCATION")]
    pub(crate) install_location: Option<String>,
}
