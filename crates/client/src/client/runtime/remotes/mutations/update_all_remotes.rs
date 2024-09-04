#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mutations {
    use crate::schema_runtime::InstanceInfo;

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation")]
    pub struct UpdateAllRemotes {
        pub remotes: UpdateAllRemotesMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes")]
    pub struct UpdateAllRemotesMutationRemotes {
        pub update_all: Vec<InstanceInfo>,
    }

    pub fn update_all() -> cynic::Operation<UpdateAllRemotes, ()> {
        use cynic::MutationBuilder;
        UpdateAllRemotes::build(())
    }
}
