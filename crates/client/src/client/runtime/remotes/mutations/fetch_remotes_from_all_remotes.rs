#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-runtime-schema.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    use crate::schema_runtime::InstanceInfo;

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation")]
    pub struct FetchRemotesFromAllRemotes {
        pub remotes: FetchRemotesFromAllRemotesMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes")]
    pub struct FetchRemotesFromAllRemotesMutationRemotes {
        pub fetch_remotes_from_all_remotes: Vec<InstanceInfo>,
    }

    pub fn fetch_remotes_from_all_remotes() -> cynic::Operation<FetchRemotesFromAllRemotes, ()> {
        use cynic::MutationBuilder;
        FetchRemotesFromAllRemotes::build(())
    }
}
