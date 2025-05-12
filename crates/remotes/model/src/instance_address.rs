use serde::Deserialize;
use serde::Serialize;
use std::hash::Hash;
use std::hash::Hasher;
use typed_builder::TypedBuilder;

pub const DEFAULT_HOSTNAME: &str = "localhost";
pub const DEFAULT_PORT: u16 = 31415;
pub const DEFAULT_USER_AGENT: &str = "reactive_graph_client";
pub const DEFAULT_ENDPOINT_GRAPHQL: &str = "/graphql";
pub const DEFAULT_ENDPOINT_DYNAMIC_GRAPH: &str = "/dynamic_graph";
pub const DEFAULT_ENDPOINT_RUNTIME: &str = "/runtime/graphql";
pub const DEFAULT_ENDPOINT_PLUGIN: &str = "/plugin/graphql";

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

    /// The user agent.
    #[builder(default = DEFAULT_USER_AGENT.to_owned())]
    #[serde(default = "default_user_agent", skip_serializing_if = "is_default_user_agent")]
    pub user_agent: String,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    #[builder(default = DEFAULT_ENDPOINT_GRAPHQL.to_owned())]
    #[serde(default = "default_endpoint_graphql", skip_serializing_if = "is_default_endpoint_graphql")]
    pub endpoint_graphql: String,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    #[builder(default = DEFAULT_ENDPOINT_DYNAMIC_GRAPH.to_owned())]
    #[serde(default = "default_endpoint_dynamic_graph", skip_serializing_if = "is_default_endpoint_dynamic_graph")]
    pub endpoint_dynamic_graph: String,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    #[builder(default = DEFAULT_ENDPOINT_RUNTIME.to_owned())]
    #[serde(default = "default_endpoint_runtime", skip_serializing_if = "is_default_endpoint_runtime")]
    pub endpoint_runtime: String,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    #[builder(default = DEFAULT_ENDPOINT_PLUGIN.to_owned())]
    #[serde(default = "default_endpoint_plugin", skip_serializing_if = "is_default_endpoint_plugin")]
    pub endpoint_plugin: String,

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
            user_agent: DEFAULT_USER_AGENT.to_string(),
            endpoint_graphql: DEFAULT_ENDPOINT_GRAPHQL.to_string(),
            endpoint_dynamic_graph: DEFAULT_ENDPOINT_DYNAMIC_GRAPH.to_string(),
            endpoint_runtime: DEFAULT_ENDPOINT_RUNTIME.to_string(),
            endpoint_plugin: DEFAULT_ENDPOINT_PLUGIN.to_string(),
            bearer: None,
        }
    }

    pub fn protocol(&self) -> String {
        if self.secure { "https".to_string() } else { "http".to_string() }
    }

    pub fn base_url(&self) -> String {
        format!("{}://{}:{}", self.protocol(), self.hostname, self.port)
    }

    pub fn url_reactive_graph(&self) -> String {
        format!("{}{}", self.base_url(), self.endpoint_graphql)
    }

    pub fn url_dynamic_graph(&self) -> String {
        format!("{}{}", self.base_url(), self.endpoint_dynamic_graph)
    }

    pub fn url_reactive_graph_runtime(&self) -> String {
        format!("{}{}", self.base_url(), self.endpoint_runtime)
    }

    pub fn url_reactive_graph_plugins(&self) -> String {
        format!("{}{}", self.base_url(), self.endpoint_plugin)
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
            user_agent: DEFAULT_USER_AGENT.to_string(),
            endpoint_graphql: DEFAULT_ENDPOINT_GRAPHQL.to_string(),
            endpoint_dynamic_graph: DEFAULT_ENDPOINT_DYNAMIC_GRAPH.to_string(),
            endpoint_runtime: DEFAULT_ENDPOINT_RUNTIME.to_string(),
            endpoint_plugin: DEFAULT_ENDPOINT_PLUGIN.to_string(),
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

fn default_user_agent() -> String {
    DEFAULT_USER_AGENT.to_owned()
}

fn is_default_user_agent(user_agent: &String) -> bool {
    DEFAULT_USER_AGENT == user_agent
}

fn default_endpoint_graphql() -> String {
    DEFAULT_ENDPOINT_GRAPHQL.to_owned()
}

fn is_default_endpoint_graphql(endpoint: &String) -> bool {
    DEFAULT_ENDPOINT_GRAPHQL == endpoint
}

fn default_endpoint_dynamic_graph() -> String {
    DEFAULT_ENDPOINT_DYNAMIC_GRAPH.to_owned()
}

fn is_default_endpoint_dynamic_graph(endpoint: &String) -> bool {
    DEFAULT_ENDPOINT_DYNAMIC_GRAPH == endpoint
}

fn default_endpoint_runtime() -> String {
    DEFAULT_ENDPOINT_RUNTIME.to_owned()
}

fn is_default_endpoint_runtime(endpoint: &String) -> bool {
    DEFAULT_ENDPOINT_RUNTIME == endpoint
}

fn default_endpoint_plugin() -> String {
    DEFAULT_ENDPOINT_PLUGIN.to_owned()
}

fn is_default_endpoint_plugin(endpoint: &String) -> bool {
    DEFAULT_ENDPOINT_PLUGIN == endpoint
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
