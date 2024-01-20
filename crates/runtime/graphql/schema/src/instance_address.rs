use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use inexor_rgf_remotes_model::InstanceAddress;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_DYNAMIC_GRAPH;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_GRAPHQL;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_PLUGIN;
use inexor_rgf_remotes_model::DEFAULT_ENDPOINT_RUNTIME;
use inexor_rgf_remotes_model::DEFAULT_USER_AGENT;

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
#[graphql(name = "InstanceAddress")]
pub struct InstanceAddressDefinition {
    /// The hostname.
    pub hostname: String,

    /// The port.
    pub port: u16,

    /// Secure endpoint.
    pub secure: Option<bool>,

    /// The user agent.
    pub user_agent: Option<String>,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    pub endpoint_graphql: Option<String>,

    /// The relative URL of the dynamic graph endpoint, by default "/dynamic_graph".
    pub endpoint_dynamic_graph: Option<String>,

    /// The relative URL of the runtime endpoint, by default "/runtime/graphql".
    pub endpoint_runtime: Option<String>,

    /// The relative URL of the plugins endpoint, by default "/plugin/graphql".
    pub endpoint_plugin: Option<String>,

    /// The authentication token.
    pub bearer: Option<String>,
}

impl From<InstanceAddressDefinition> for InstanceAddress {
    fn from(address: InstanceAddressDefinition) -> Self {
        InstanceAddress {
            hostname: address.hostname,
            port: address.port,
            secure: address.secure.unwrap_or(false),
            user_agent: address.user_agent.unwrap_or(DEFAULT_USER_AGENT.to_owned()),
            endpoint_graphql: address.endpoint_graphql.unwrap_or(DEFAULT_ENDPOINT_GRAPHQL.to_owned()),
            endpoint_dynamic_graph: address.endpoint_dynamic_graph.unwrap_or(DEFAULT_ENDPOINT_DYNAMIC_GRAPH.to_owned()),
            endpoint_runtime: address.endpoint_runtime.unwrap_or(DEFAULT_ENDPOINT_RUNTIME.to_owned()),
            endpoint_plugin: address.endpoint_plugin.unwrap_or(DEFAULT_ENDPOINT_PLUGIN.to_owned()),
            bearer: address.bearer,
        }
    }
}
