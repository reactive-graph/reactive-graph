#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod mapping {
    use crate::schema::system::instance::InstanceInfo;
    use typed_builder::TypedBuilder;

    #[derive(cynic::QueryVariables, Debug, TypedBuilder)]
    pub struct AddRemoteVariables {
        pub hostname: String,
        pub port: i32,
        pub secure: Option<bool>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllRemotes {
        pub system: GetAllRemotesSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "System")]
    pub struct GetAllRemotesSystem {
        pub remotes: Vec<InstanceInfo>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddRemoteVariables")]
    pub struct AddRemote {
        pub system: AddRemoteMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem", variables = "AddRemoteVariables")]
    pub struct AddRemoteMutationSystem {
        pub remotes: AddRemoteMutationRemotes,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationRemotes", variables = "AddRemoteVariables")]
    pub struct AddRemoteMutationRemotes {
        #[arguments(hostname: $hostname, port: $port, secure: $secure)]
        pub add: InstanceInfo,
    }
}

pub mod queries {
    use crate::client::system::remotes::mapping::GetAllRemotes;

    pub fn get_all() -> cynic::Operation<GetAllRemotes, ()> {
        use cynic::QueryBuilder;
        GetAllRemotes::build(())
    }
}

pub mod operations {
    use crate::client::system::remotes::mapping::AddRemote;
    use crate::system::remotes::mapping::AddRemoteVariables;

    pub fn add(vars: AddRemoteVariables) -> cynic::Operation<AddRemote, AddRemoteVariables> {
        use cynic::MutationBuilder;
        AddRemote::build(vars)
    }
}

pub mod api {
    use std::sync::Arc;

    use crate::client::system::remotes::queries::get_all;
    use crate::model_runtime::InstanceInfo;
    use crate::InexorRgfClient;
    use crate::InexorRgfClientExecutionError;
    use crate::InstanceInfos;

    pub struct Remotes {
        client: Arc<InexorRgfClient>,
    }

    impl Remotes {
        pub fn new(client: Arc<InexorRgfClient>) -> Self {
            Self { client }
        }

        pub async fn get_all(&self) -> Result<Vec<InstanceInfo>, InexorRgfClientExecutionError> {
            self.client.run_graphql(get_all(), |data| InstanceInfos(data.system.remotes).into()).await
        }
    }
}
