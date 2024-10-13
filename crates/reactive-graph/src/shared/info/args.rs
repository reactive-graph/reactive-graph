use crate::shared::info::commands::InfoCommands;
use clap::Parser;
use serde::Serialize;

#[derive(Parser, Debug)]
pub struct InfoArgs {
    #[command(subcommand)]
    pub commands: Option<InfoCommands>,
}

#[derive(Parser, Debug)]
pub struct InfoCommandArgs {
    #[arg(long)]
    pub output_format: Option<OutputFormatArgs>,
}

#[derive(clap::ValueEnum, Default, Debug, Clone, Serialize)]
pub enum OutputFormatArgs {
    #[default]
    Default,
    Json,
    #[cfg(feature = "toml")]
    Toml,
}
