use clap::Parser;

#[derive(Parser, Debug)]
pub struct UninstallPluginArgs {
    /// The name of the plugin.
    pub plugin_name: String,
}
