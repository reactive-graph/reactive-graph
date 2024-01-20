use async_graphql::Request;
use std::sync::Arc;

use serde::Deserialize;

use crate::tests::util::address_to_vars;
use inexor_rgf_remotes_model::InstanceAddress;
use inexor_rgf_remotes_model::InstanceInfo;
use inexor_rgf_runtime_api::Runtime;
use inexor_rgf_runtime_graphql_api::RuntimeQueryService;
use inexor_rgf_runtime_impl::RuntimeBuilder;

#[derive(Deserialize, Debug)]
struct Mutation {
    remotes: MutationFetchRemotesFromRemote,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationFetchRemotesFromRemote {
    fetch_remotes_from_remote: Vec<InstanceInfo>,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fetch_remotes_from_remote() {
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
            let query_service = runtime.get_runtime_query_service();
            let instance_service = runtime.get_instance_service();
            let remotes_manager = runtime.get_remotes_manager();
            remotes_manager.remove_all();

            // RT: Expect that no remotes exists
            assert_eq!(remotes_manager.get_all().len(), 0);

            // RT: Add self as remote
            let address = instance_service.get_instance_info().address();
            println!("{:?}", address);
            let _instance_info = remotes_manager.add(&address).await.expect("Failed to fetch remotes from remote");
            println!("{:?}", _instance_info);

            // GQL: Fetch remote of the given remote
            let _fetched_remotes_from_remote = mutation_fetch_remotes_from_remote(&query_service, &address).await;
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const MUTATION_FETCH_REMOTES_FROM_REMOTE: &str = include_str!("fetch_remotes_from_remote.graphql");

async fn mutation_fetch_remotes_from_remote(query_service: &Arc<dyn RuntimeQueryService + Send + Sync>, address: &InstanceAddress) -> Vec<InstanceInfo> {
    println!("{:?}", address);
    println!("{:?}", address_to_vars(address));
    let request = Request::new(MUTATION_FETCH_REMOTES_FROM_REMOTE).variables(address_to_vars(address));
    let response = query_service.execute(request).await;
    assert!(response.errors.is_empty(), "{:?}", response.errors);
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    println!("{}", data);
    let data: Mutation = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.remotes.fetch_remotes_from_remote
}
