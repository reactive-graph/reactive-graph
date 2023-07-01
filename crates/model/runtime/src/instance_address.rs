use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

pub const DEFAULT_HOSTNAME: &str = "localhost";
pub const DEFAULT_PORT: u16 = 31415;
pub const DEFAULT_ENDPOINT: &str = "/graphql";
pub const DEFAULT_USER_AGENT: &str = "inexor_rgf_client";

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InstanceAddress {
    /// The hostname of the GraphQL server.
    #[builder(default=DEFAULT_HOSTNAME.to_owned())]
    pub hostname: String,

    /// The port of the GraphQL server.
    #[builder(default=DEFAULT_PORT)]
    pub port: u16,

    /// If true, HTTPS will be used.
    #[builder(default = false)]
    pub secure: bool,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    #[builder(default = DEFAULT_ENDPOINT.to_owned())]
    #[serde(default = "default_endpoint")]
    pub endpoint: String,

    /// The user agent.
    #[builder(default = DEFAULT_USER_AGENT.to_owned())]
    #[serde(default = "default_user_agent")]
    pub user_agent: String,

    /// The authentication token.
    #[builder(default)]
    pub bearer: Option<String>,
}

impl InstanceAddress {
    pub fn new(hostname: String, port: u16, secure: bool) -> InstanceAddress {
        InstanceAddress {
            hostname,
            port,
            secure,
            endpoint: DEFAULT_ENDPOINT.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            bearer: None,
        }
    }

    pub fn protocol(&self) -> String {
        if self.secure {
            "https".to_string()
        } else {
            "http".to_string()
        }
    }

    pub fn url(&self) -> String {
        format!("{}://{}:{}{}", self.protocol(), self.hostname, self.port, self.endpoint)
    }
}

impl Default for InstanceAddress {
    fn default() -> Self {
        InstanceAddress {
            hostname: DEFAULT_HOSTNAME.to_string(),
            port: DEFAULT_PORT,
            secure: false,
            endpoint: DEFAULT_ENDPOINT.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            bearer: None,
        }
    }
}

fn default_endpoint() -> String {
    DEFAULT_ENDPOINT.to_owned()
}

fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_owned()
}
