use crate::shared::info::commands::InfoCommands;
use crate::shared::output_format::OutputFormatArgsOptional;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct InfoArgs {
    #[command(subcommand)]
    pub commands: Option<InfoCommands>,
}

#[derive(Parser, Debug)]
pub struct InfoCommandArgs {
    #[clap(flatten)]
    pub output_format: OutputFormatArgsOptional,
}
