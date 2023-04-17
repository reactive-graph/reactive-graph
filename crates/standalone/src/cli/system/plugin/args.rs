use clap::Args;

use crate::cli::system::plugin::commands::PluginsCommands;
use crate::client::system::plugin::mapping::SearchPluginVariables;

#[derive(Args, Debug, Clone)]
pub(crate) struct PluginsArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<PluginsCommands>,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct PluginByNameArgs {
    /// The plugin name.
    pub name: String,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct SearchPluginsArgs {
    /// The plugin name.
    #[arg(long)]
    pub name: Option<String>,

    /// The plugin state.
    #[arg(long)]
    pub state: Option<String>,

    /// The plugin file stem.
    #[arg(long)]
    pub stem: Option<String>,
}

impl From<SearchPluginsArgs> for SearchPluginVariables {
    fn from(args: SearchPluginsArgs) -> Self {
        SearchPluginVariables {
            name: args.name,
            state: args.state,
            stem: args.stem,
        }
    }
}
