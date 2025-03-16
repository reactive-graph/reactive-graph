#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    use crate::InstanceInfo;
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariables;
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariablesFields;

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation", variables = "InstanceAddressVariables")]
    pub struct FetchRemotesFromRemote {
        pub remotes: FetchRemotesFromRemoteMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes", variables = "InstanceAddressVariables")]
    pub struct FetchRemotesFromRemoteMutationRemotes {
        #[arguments(address: $address)]
        pub fetch_remotes_from_remote: Vec<InstanceInfo>,
    }

    pub fn fetch_remotes_from_remote(vars: InstanceAddressVariables) -> cynic::Operation<FetchRemotesFromRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        FetchRemotesFromRemote::build(vars)
    }
}
