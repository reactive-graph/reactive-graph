use clap::ArgAction::SetTrue;
use clap::Args;

use crate::cli::commands::ClientCommands;
use crate::client::InexorClientConfig;
use crate::client::DEFAULT_ENDPOINT;
use crate::client::DEFAULT_HOSTNAME;
use crate::client::DEFAULT_PORT;
use crate::client::DEFAULT_USER_AGENT;

#[derive(Args, Debug, Clone)]
pub struct ClientArgs {
    /// The hostname to connect to.
    #[arg(long)]
    hostname: Option<String>,

    /// The port to connect to.
    #[arg(long)]
    port: Option<u16>,

    /// If true, connects via HTTPS.
    #[arg(long, action = SetTrue)]
    secure: Option<bool>,

    /// The endpoint to use.
    #[arg(long)]
    endpoint: Option<String>,

    /// The authentication token.
    #[arg(long)]
    bearer: Option<String>,

    #[command(subcommand)]
    pub(crate) commands: Option<ClientCommands>,
}

impl From<&ClientArgs> for InexorClientConfig {
    fn from(client_args: &ClientArgs) -> Self {
        InexorClientConfig {
            hostname: client_args.hostname.clone().unwrap_or(DEFAULT_HOSTNAME.to_string()),
            port: client_args.port.unwrap_or(DEFAULT_PORT),
            secure: client_args.secure.unwrap_or(false),
            endpoint: client_args.endpoint.clone().unwrap_or(DEFAULT_ENDPOINT.to_string()),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            bearer: client_args.bearer.clone(),
        }
    }
}
