use crate::client::introspection::commands::IntrospectionQueryCommands;
use clap::Args;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct IntrospectionQueryArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<IntrospectionQueryCommands>,
}
