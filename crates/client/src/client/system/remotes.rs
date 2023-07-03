#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod mapping {
    use crate::schema::system::instance::InstanceInfo;
    use typed_builder::TypedBuilder;

    #[derive(cynic::QueryVariables, Debug, TypedBuilder)]
    pub struct InstanceAddressVariables {
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
    #[cynic(graphql_type = "Mutation", variables = "InstanceAddressVariables")]
    pub struct AddRemote {
        pub system: AddRemoteMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem", variables = "InstanceAddressVariables")]
    pub struct AddRemoteMutationSystem {
        pub remotes: AddRemoteMutationRemotes,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationRemotes", variables = "InstanceAddressVariables")]
    pub struct AddRemoteMutationRemotes {
        #[arguments(hostname: $hostname, port: $port, secure: $secure)]
        pub add: InstanceInfo,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "InstanceAddressVariables")]
    pub struct RemoveRemote {
        pub system: RemoveRemoteMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem", variables = "InstanceAddressVariables")]
    pub struct RemoveRemoteMutationSystem {
        pub remotes: RemoveRemoteMutationRemotes,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationRemotes", variables = "InstanceAddressVariables")]
    pub struct RemoveRemoteMutationRemotes {
        #[arguments(hostname: $hostname, port: $port, secure: $secure)]
        pub remove: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation")]
    pub struct RemoveAllRemotes {
        pub system: RemoveAllRemotesMutationSystem,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationSystem")]
    pub struct RemoveAllRemotesMutationSystem {
        pub remotes: RemoveAllRemotesMutationRemotes,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "MutationRemotes")]
    pub struct RemoveAllRemotesMutationRemotes {
        pub remove_all: bool,
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
    use crate::system::remotes::mapping::InstanceAddressVariables;
    use crate::system::remotes::mapping::RemoveAllRemotes;
    use crate::system::remotes::mapping::RemoveRemote;

    pub fn add(vars: InstanceAddressVariables) -> cynic::Operation<AddRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        AddRemote::build(vars)
    }

    pub fn remove(vars: InstanceAddressVariables) -> cynic::Operation<RemoveRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        RemoveRemote::build(vars)
    }

    pub fn remove_all() -> cynic::Operation<RemoveAllRemotes, ()> {
        use cynic::MutationBuilder;
        RemoveAllRemotes::build(())
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
