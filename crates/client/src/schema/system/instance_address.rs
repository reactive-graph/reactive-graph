#[derive(Clone, Debug, cynic::InputObject)]
#[cynic(schema_path = "schema.graphql", schema_module = "crate::schema::schema")]
pub struct InstanceAddress {
    pub hostname: String,
    pub port: i32,
    pub secure: bool,
    pub endpoint: Option<String>,
    pub user_agent: Option<String>,
    pub bearer: Option<String>,
}

impl From<crate::config::InstanceAddress> for InstanceAddress {
    fn from(address: crate::config::InstanceAddress) -> Self {
        InstanceAddress {
            hostname: address.hostname,
            port: address.port as i32,
            secure: address.secure,
            endpoint: Some(address.endpoint),
            user_agent: Some(address.user_agent),
            bearer: address.bearer,
        }
    }
}
