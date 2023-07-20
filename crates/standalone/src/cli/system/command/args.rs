use clap::Args;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct ExecuteCommandArgs {
    /// The command name.
    pub command_name: String,

    /// The command arguments.
    #[arg(trailing_var_arg = true)]
    pub command_arguments: Vec<String>,
}
