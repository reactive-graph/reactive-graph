use reactive_graph_remotes_model::InstanceAddress;
use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_graphql_api::RuntimeQueryService;
use reactive_graph_runtime_impl::RuntimeBuilder;
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    remotes: Vec<InstanceAddress>,
}

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
            sleep(Duration::from_millis(2000)).await;

            let query_service = runtime.get_runtime_query_service();
            let instance_service = runtime.get_instance_service();
            let remotes_manager = runtime.get_remotes_manager();
            remotes_manager.remove_all();

            // GQL: Expect that no remotes exists
            assert_eq!(query_get_all_remotes(&query_service).await.len(), 0);

            // RT: Add self as remote
            remotes_manager
                .add(&instance_service.get_instance_info().into())
                .await
                .expect("Failed to add self to list of remotes");

            // GQL: Expect that one remote exist
            assert_eq!(query_get_all_remotes(&query_service).await.len(), 1);
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const QUERY_GET_ALL_REMOTES: &str = include_str!("get_all.graphql");

async fn query_get_all_remotes(query_service: &Arc<dyn RuntimeQueryService + Send + Sync>) -> Vec<InstanceAddress> {
    let response = query_service.query_response(QUERY_GET_ALL_REMOTES).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.remotes
}
