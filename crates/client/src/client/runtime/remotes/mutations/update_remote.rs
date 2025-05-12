#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-runtime-schema.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    use crate::InstanceInfo;
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariables;
    use crate::client::runtime::instance::variables::address::variables::InstanceAddressVariablesFields;

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation", variables = "InstanceAddressVariables")]
    pub struct UpdateRemote {
        pub remotes: UpdateRemoteMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes", variables = "InstanceAddressVariables")]
    pub struct UpdateRemoteMutationRemotes {
        #[arguments(address: $address)]
        pub update: InstanceInfo,
    }

    pub fn update(vars: InstanceAddressVariables) -> cynic::Operation<UpdateRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        UpdateRemote::build(vars)
    }
}
