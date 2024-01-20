use std::sync::Arc;

use serde::Deserialize;

use inexor_rgf_runtime_api::Runtime;
use inexor_rgf_runtime_graphql_api::RuntimeQueryService;
use inexor_rgf_runtime_impl::RuntimeBuilder;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Mutation {
    remotes: MutationSystemRemoveAllRemotes,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystemRemoveAllRemotes {
    remove_all: bool,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_remove_all_remotes() {
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
            let _ = remotes_manager.add(&address).await.expect("Failed to add remote");

            // RT: Expect that one remote exist
            assert_eq!(remotes_manager.get_all().len(), 1);

            // GQL: Remove all remotes
            let has_been_removed = mutation_remove_all_remotes(&query_service).await;
            assert!(has_been_removed, "Expected that the remotes has been removed");

            // RT: Expect that no remotes exists
            assert_eq!(remotes_manager.get_all().len(), 0, "Expected that no remotes exists anymore");
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const MUTATION_REMOVE_ALL_REMOTES: &str = include_str!("./remove_all.graphql");

async fn mutation_remove_all_remotes(query_service: &Arc<dyn RuntimeQueryService + Send + Sync>) -> bool {
    let response = query_service.query_response(MUTATION_REMOVE_ALL_REMOTES).await;
    assert!(response.errors.is_empty(), "Expect no graphql errors");
    println!("{}", response.data);
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    let data: Mutation = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.remotes.remove_all
}
