use clap::Parser;

#[derive(Parser, Debug)]
pub struct PluginsArguments {
    // Plugins
    /// If true, all plugins will be disabled.
    #[arg(short = 'x', long, env = "REACTIVE_GRAPH_DISABLE_ALL_PLUGINS")]
    pub disable_all_plugins: Option<bool>,

    /// The list of plugins to disable.
    #[arg(short = 'p', long)]
    pub disabled_plugins: Option<Vec<String>>,

    /// The list of plugins to enable.
    #[arg(short = 'P', long)]
    pub enabled_plugins: Option<Vec<String>>,

    /// If true, hot deployment will be disabled.
    #[arg(long, env = "REACTIVE_GRAPH_DISABLE_HOT_DEPLOY")]
    pub disable_hot_deploy: Option<bool>,

    /// The folder which is watched for hot deployment.
    #[arg(long, env = "REACTIVE_GRAPH_HOT_DEPLOY_LOCATION")]
    pub hot_deploy_location: Option<String>,

    /// The folder which plugins are installed permanently.
    #[arg(long, env = "REACTIVE_GRAPH_INSTALL_LOCATION")]
    pub install_location: Option<String>,
}
