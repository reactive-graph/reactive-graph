use serde::Deserialize;
use serde::Serialize;

pub mod builder;

pub const DEFAULT_HOSTNAME: &str = "localhost";
pub const DEFAULT_PORT: u16 = 31415;
pub const DEFAULT_ENDPOINT: &str = "/graphql";
pub const DEFAULT_USER_AGENT: &str = "inexor_rgf_client";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InexorClientConfig {
    /// The hostname of the GraphQL server to connect to.
    pub hostname: String,

    /// The port of the GraphQL server to connect to.
    pub port: u16,

    /// If true, HTTPS will be used.
    pub secure: bool,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    pub endpoint: String,

    /// The user agent.
    pub user_agent: String,

    /// The authentication token.
    pub bearer: Option<String>,
}

impl Default for InexorClientConfig {
    fn default() -> Self {
        InexorClientConfig {
            hostname: DEFAULT_HOSTNAME.to_string(),
            port: DEFAULT_PORT,
            secure: false,
            endpoint: DEFAULT_ENDPOINT.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            bearer: None,
        }
    }
}
