use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;
use typed_builder::TypedBuilder;

pub const DEFAULT_HOSTNAME: &str = "localhost";
pub const DEFAULT_PORT: u16 = 31415;
pub const DEFAULT_ENDPOINT: &str = "/graphql";
pub const DEFAULT_USER_AGENT: &str = "inexor_rgf_client";

#[derive(Clone, Debug, Eq, Deserialize, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InstanceAddress {
    /// The hostname of the GraphQL server.
    #[builder(default = DEFAULT_HOSTNAME.to_owned())]
    pub hostname: String,

    /// The port of the GraphQL server.
    #[builder(default = DEFAULT_PORT)]
    #[serde(default = "default_port", skip_serializing_if = "is_default_port")]
    pub port: u16,

    /// If true, HTTPS will be used.
    #[builder(default = false)]
    #[serde(default = "bool::default", skip_serializing_if = "is_default")]
    pub secure: bool,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    #[builder(default = DEFAULT_ENDPOINT.to_owned())]
    #[serde(default = "default_endpoint", skip_serializing_if = "is_default_endpoint")]
    pub endpoint: String,

    /// The user agent.
    #[builder(default = DEFAULT_USER_AGENT.to_owned())]
    #[serde(default = "default_user_agent", skip_serializing_if = "is_default_user_agent")]
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

// An InstanceAddress is equals if hostname, port and secure are equal
impl PartialEq<InstanceAddress> for InstanceAddress {
    fn eq(&self, other: &InstanceAddress) -> bool {
        self.hostname == other.hostname && self.port == other.port && self.secure == other.secure
    }
}

impl Hash for InstanceAddress {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hostname.hash(state);
        self.port.hash(state);
        self.secure.hash(state);
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

fn default_port() -> u16 {
    DEFAULT_PORT
}

fn is_default_port(port: &u16) -> bool {
    DEFAULT_PORT == *port
}

fn default_endpoint() -> String {
    DEFAULT_ENDPOINT.to_owned()
}

fn is_default_endpoint(endpoint: &String) -> bool {
    DEFAULT_ENDPOINT == endpoint
}

fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_owned()
}

fn is_default_user_agent(user_agent: &String) -> bool {
    DEFAULT_USER_AGENT == user_agent
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
