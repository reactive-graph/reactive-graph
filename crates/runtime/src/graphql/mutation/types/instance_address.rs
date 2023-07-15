use async_graphql::InputObject;
use serde::Deserialize;
use serde::Serialize;

use crate::config::InstanceAddress;

#[derive(Clone, Debug, Serialize, Deserialize, InputObject)]
#[graphql(name = "InstanceAddress")]
pub struct InstanceAddressDefinition {
    /// The hostname.
    pub hostname: String,

    /// The port.
    pub port: u16,

    /// Secure endpoint.
    pub secure: Option<bool>,
}

impl From<InstanceAddressDefinition> for InstanceAddress {
    fn from(address: InstanceAddressDefinition) -> Self {
        InstanceAddress::new(address.hostname, address.port, address.secure.unwrap_or(false))
    }
}
