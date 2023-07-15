use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use crate::config::InstanceAddress;
use crate::config::DEFAULT_ENDPOINT;
use crate::config::DEFAULT_USER_AGENT;

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
#[graphql(name = "InstanceAddress")]
pub struct InstanceAddressDefinition {
    /// The hostname.
    pub hostname: String,

    /// The port.
    pub port: u16,

    /// Secure endpoint.
    pub secure: Option<bool>,

    /// The relative URL of the GraphQL endpoint, by default "/graphql".
    pub endpoint: Option<String>,

    /// The user agent.
    pub user_agent: Option<String>,

    /// The authentication token.
    pub bearer: Option<String>,
}

impl From<InstanceAddressDefinition> for InstanceAddress {
    fn from(address: InstanceAddressDefinition) -> Self {
        InstanceAddress {
            hostname: address.hostname,
            port: address.port,
            secure: address.secure.unwrap_or(false),
            endpoint: address.endpoint.unwrap_or(DEFAULT_ENDPOINT.to_owned()),
            user_agent: address.user_agent.unwrap_or(DEFAULT_USER_AGENT.to_owned()),
            bearer: address.bearer,
        }
    }
}
