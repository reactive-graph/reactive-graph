#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation")]
    pub struct RemoveAllRemotes {
        pub remotes: RemoveAllRemotesMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes")]
    pub struct RemoveAllRemotesMutationRemotes {
        pub remove_all: bool,
    }

    pub fn remove_all() -> cynic::Operation<RemoveAllRemotes, ()> {
        use cynic::MutationBuilder;
        RemoveAllRemotes::build(())
    }
}
