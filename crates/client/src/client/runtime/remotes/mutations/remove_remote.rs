#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariables;
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariablesFields;
    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation", variables = "InstanceAddressVariables")]
    pub struct RemoveRemote {
        pub remotes: RemoveRemoteMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes", variables = "InstanceAddressVariables")]
    pub struct RemoveRemoteMutationRemotes {
        #[arguments(address: $address)]
        pub remove: bool,
    }

    pub fn remove(vars: InstanceAddressVariables) -> cynic::Operation<RemoveRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        RemoveRemote::build(vars)
    }
}
