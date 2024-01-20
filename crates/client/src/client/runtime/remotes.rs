#[cynic::schema_for_derives(file = r#"schema_runtime.graphql"#, module = "crate::schema_runtime::schema")]
pub mod mapping {
    use crate::schema_runtime::InstanceAddress;
    use crate::schema_runtime::InstanceInfo;

    #[derive(Debug, cynic::QueryVariables)]
    pub struct InstanceAddressVariables {
        pub address: InstanceAddress,
    }

    impl From<&inexor_rgf_remotes_model::InstanceAddress> for InstanceAddressVariables {
        fn from(address: &inexor_rgf_remotes_model::InstanceAddress) -> Self {
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

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllRemotes {
        pub remotes: Vec<InstanceInfo>,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "Mutation", variables = "InstanceAddressVariables")]
    pub struct AddRemote {
        pub remotes: AddRemoteMutationRemotes,
    }

    #[derive(Debug, cynic::QueryFragment)]
    #[cynic(graphql_type = "MutationRemotes", variables = "InstanceAddressVariables")]
    pub struct AddRemoteMutationRemotes {
        #[arguments(address: $address)]
        pub add: InstanceInfo,
    }

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
}

pub mod queries {
    use crate::client::runtime::remotes::mapping::GetAllRemotes;

    pub fn get_all() -> cynic::Operation<GetAllRemotes, ()> {
        use cynic::QueryBuilder;
        GetAllRemotes::build(())
    }
}

pub mod operations {
    use crate::client::runtime::remotes::mapping::AddRemote;
    use crate::client::runtime::remotes::mapping::FetchRemotesFromAllRemotes;
    use crate::client::runtime::remotes::mapping::FetchRemotesFromRemote;
    use crate::client::runtime::remotes::mapping::InstanceAddressVariables;
    use crate::client::runtime::remotes::mapping::RemoveAllRemotes;
    use crate::client::runtime::remotes::mapping::RemoveRemote;
    use crate::client::runtime::remotes::mapping::UpdateAllRemotes;
    use crate::client::runtime::remotes::mapping::UpdateRemote;

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

    pub fn update(vars: InstanceAddressVariables) -> cynic::Operation<UpdateRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        UpdateRemote::build(vars)
    }

    pub fn update_all() -> cynic::Operation<UpdateAllRemotes, ()> {
        use cynic::MutationBuilder;
        UpdateAllRemotes::build(())
    }

    pub fn fetch_remotes_from_remote(vars: InstanceAddressVariables) -> cynic::Operation<FetchRemotesFromRemote, InstanceAddressVariables> {
        use cynic::MutationBuilder;
        FetchRemotesFromRemote::build(vars)
    }

    pub fn fetch_remotes_from_all_remotes() -> cynic::Operation<FetchRemotesFromAllRemotes, ()> {
        use cynic::MutationBuilder;
        FetchRemotesFromAllRemotes::build(())
    }
}

pub mod api {
    use std::sync::Arc;

    use crate::client::runtime::remotes::operations::add;
    use crate::client::runtime::remotes::operations::fetch_remotes_from_all_remotes;
    use crate::client::runtime::remotes::operations::fetch_remotes_from_remote;
    use crate::client::runtime::remotes::operations::remove;
    use crate::client::runtime::remotes::operations::remove_all;
    use crate::client::runtime::remotes::operations::update;
    use crate::client::runtime::remotes::operations::update_all;
    use crate::client::runtime::remotes::queries::get_all;
    use crate::schema_runtime::InstanceInfos;
    use crate::InexorRgfClient;
    use crate::InexorRgfClientExecutionError;
    use inexor_rgf_remotes_model::InstanceAddress;
    use inexor_rgf_remotes_model::InstanceInfo;

    pub struct Remotes {
        client: Arc<InexorRgfClient>,
    }

    impl Remotes {
        pub fn new(client: Arc<InexorRgfClient>) -> Self {
            Self { client }
        }

        pub async fn get_all(&self) -> Result<Vec<InstanceInfo>, InexorRgfClientExecutionError> {
            self.client.execute_runtime(get_all(), |data| InstanceInfos(data.remotes).into()).await
        }

        pub async fn add(&self, address: &InstanceAddress) -> Result<InstanceInfo, InexorRgfClientExecutionError> {
            self.client.execute_runtime(add(address.into()), |data| data.remotes.add.into()).await
        }

        pub async fn remove(&self, address: &InstanceAddress) -> Result<bool, InexorRgfClientExecutionError> {
            self.client.execute_runtime(remove(address.into()), |data| data.remotes.remove).await
        }

        pub async fn remove_all(&self) -> Result<bool, InexorRgfClientExecutionError> {
            self.client.execute_runtime(remove_all(), |data| data.remotes.remove_all).await
        }

        pub async fn update(&self, address: &InstanceAddress) -> Result<InstanceInfo, InexorRgfClientExecutionError> {
            self.client.execute_runtime(update(address.into()), |data| data.remotes.update.into()).await
        }

        pub async fn update_all(&self) -> Result<Vec<InstanceInfo>, InexorRgfClientExecutionError> {
            self.client
                .execute_runtime(update_all(), |data| InstanceInfos(data.remotes.update_all).into())
                .await
        }

        pub async fn fetch_remotes_from_remote(&self, address: &InstanceAddress) -> Result<Vec<InstanceInfo>, InexorRgfClientExecutionError> {
            self.client
                .execute_runtime(fetch_remotes_from_remote(address.into()), |data| InstanceInfos(data.remotes.fetch_remotes_from_remote).into())
                .await
        }

        pub async fn fetch_remotes_from_all_remotes(&self) -> Result<Vec<InstanceInfo>, InexorRgfClientExecutionError> {
            self.client
                .execute_runtime(fetch_remotes_from_all_remotes(), |data| InstanceInfos(data.remotes.fetch_remotes_from_all_remotes).into())
                .await
        }
    }
}

#[cfg(test)]
pub mod test {

    use crate::InexorRgfClient;
    use inexor_rgf_runtime_api::Runtime;
    use inexor_rgf_runtime_impl::RuntimeBuilder;
    use std::sync::Arc;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_all_remotes() {
        RuntimeBuilder::new()
            .ignore_config_files()
            .disable_all_plugins(true)
            .pick_free_port()
            .init()
            .await
            .post_init()
            .await
            .spawn()
            .await
            .with_runtime(|runtime: Arc<dyn Runtime + Send + Sync>| async move {
                let remotes_manager = runtime.get_remotes_manager();

                let rt_address = runtime.address();

                // RT: Create remote
                remotes_manager.add(&rt_address).await.expect("Failed to add self to list of remotes");

                let rt_remotes = remotes_manager.get_all();

                // Client: Connect to self and get all remotes
                let client = InexorRgfClient::new(rt_address).expect("Cannot create client");
                let remotes = client.runtime().remotes().get_all().await.expect("Failed to get all remotes");

                // Expect that the remotes of the runtime are the same
                assert_eq!(remotes.len(), 1);
                assert_eq!(remotes, rt_remotes);
            })
            .await
            .stop()
            .await
            .pre_shutdown()
            .await
            .shutdown()
            .await;
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_add_remote() {
        RuntimeBuilder::new()
            .ignore_config_files()
            .disable_all_plugins(true)
            .pick_free_port()
            .init()
            .await
            .post_init()
            .await
            .spawn()
            .await
            .with_runtime(|runtime: Arc<dyn Runtime + Send + Sync>| async move {
                let remotes_manager = runtime.get_remotes_manager();

                // Get instance info from the runtime
                let rt_address = runtime.address();

                // Check that there are no remotes
                assert_eq!(remotes_manager.get_all().len(), 0);

                // Client: Connect to self and get all remotes
                let client = InexorRgfClient::new(rt_address.clone()).expect("Cannot create client");
                let remote = client.runtime().remotes().add(&rt_address).await.expect("Failed to add remote");

                // Check that there is a new remote
                assert_eq!(remotes_manager.get_all().len(), 1);
                assert_eq!(rt_address, remote.address);
            })
            .await
            .stop()
            .await
            .pre_shutdown()
            .await
            .shutdown()
            .await;
    }
}
