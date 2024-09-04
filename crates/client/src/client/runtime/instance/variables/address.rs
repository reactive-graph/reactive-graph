#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod variables {
    use crate::schema_runtime::InstanceAddress;

    #[derive(Debug, cynic::QueryVariables)]
    pub struct InstanceAddressVariables {
        pub address: InstanceAddress,
    }

    impl From<&reactive_graph_remotes_model::InstanceAddress> for InstanceAddressVariables {
        fn from(address: &reactive_graph_remotes_model::InstanceAddress) -> Self {
            InstanceAddressVariables {
                address: InstanceAddress {
                    hostname: address.hostname.clone(),
                    port: address.port as i32,
                    secure: address.secure,
                    user_agent: Some(address.user_agent.clone()),
                    endpoint_graphql: Some(address.endpoint_graphql.clone()),
                    endpoint_dynamic_graph: Some(address.endpoint_dynamic_graph.clone()),
                    endpoint_runtime: Some(address.endpoint_runtime.clone()),
                    endpoint_plugin: Some(address.endpoint_plugin.clone()),
                    bearer: address.bearer.clone(),
                },
            }
        }
    }
}
