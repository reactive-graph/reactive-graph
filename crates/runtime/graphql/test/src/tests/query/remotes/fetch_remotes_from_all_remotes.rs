use async_graphql::Request;
use reactive_graph_remotes_model::InstanceInfo;
use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_graphql_api::RuntimeQueryService;
use reactive_graph_runtime_impl::RuntimeBuilder;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Mutation {
    remotes: MutationSystemFetchRemotesFromAllRemotes,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystemFetchRemotesFromAllRemotes {
    fetch_remotes_from_all_remotes: Vec<InstanceInfo>,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fetch_remotes_from_all_remotes() {
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
            sleep(Duration::from_millis(2000)).await;

            let query_service = runtime.get_runtime_query_service();
            let instance_service = runtime.get_instance_service();
            let remotes_manager = runtime.get_remotes_manager();
            remotes_manager.remove_all();

            // RT: Expect that no remotes exists
            assert_eq!(remotes_manager.get_all().len(), 0);

            // RT: Add self as remote
            let address = instance_service.get_instance_info().address();
            let _instance_info = remotes_manager.add(&address).await.expect("Failed to add remote");

            // GQL: Fetch remotes of all remotes
            let _fetched_remotes_from_all_remotes = mutation_fetch_remotes_from_all_remotes(&query_service).await;
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const MUTATION_FETCH_REMOTES_FROM_ALL_REMOTES: &str = include_str!("fetch_remotes_from_all_remotes.graphql");

async fn mutation_fetch_remotes_from_all_remotes(query_service: &Arc<dyn RuntimeQueryService + Send + Sync>) -> Vec<InstanceInfo> {
    let request = Request::new(MUTATION_FETCH_REMOTES_FROM_ALL_REMOTES);
    let response = query_service.execute(request).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    println!("{}", data);
    let data: Mutation = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.remotes.fetch_remotes_from_all_remotes
}
