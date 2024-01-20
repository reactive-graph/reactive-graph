#[derive(Clone, Debug, cynic::InputObject)]
#[cynic(schema_path = "schema_runtime.graphql", schema_module = "crate::schema_runtime::schema")]
pub struct InstanceAddress {
    pub hostname: String,
    pub port: i32,
    pub secure: bool,
    pub user_agent: Option<String>,
    pub endpoint_graphql: Option<String>,
    pub endpoint_dynamic_graph: Option<String>,
    pub endpoint_runtime: Option<String>,
    pub endpoint_plugin: Option<String>,
    pub bearer: Option<String>,
}

impl From<inexor_rgf_remotes_model::InstanceAddress> for InstanceAddress {
    fn from(address: inexor_rgf_remotes_model::InstanceAddress) -> Self {
        InstanceAddress {
            hostname: address.hostname,
            port: address.port as i32,
            secure: address.secure,
            user_agent: Some(address.user_agent),
            endpoint_graphql: Some(address.endpoint_graphql),
            endpoint_dynamic_graph: Some(address.endpoint_dynamic_graph),
            endpoint_runtime: Some(address.endpoint_runtime),
            endpoint_plugin: Some(address.endpoint_plugin),
            bearer: address.bearer,
        }
    }
}
