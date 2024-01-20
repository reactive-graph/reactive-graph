use async_graphql::Request;
use std::sync::Arc;

use serde::Deserialize;

use inexor_rgf_remotes_model::InstanceInfo;
use inexor_rgf_runtime_api::Runtime;
use inexor_rgf_runtime_graphql_api::RuntimeQueryService;
use inexor_rgf_runtime_impl::RuntimeBuilder;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Mutation {
    remotes: MutationSystemUpdateAllRemotes,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystemUpdateAllRemotes {
    update_all: Vec<InstanceInfo>,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update_all_remotes() {
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
            let instance_info = remotes_manager.add(&address).await.expect("Failed to add remote");

            // RT: Expect that one remote exists
            assert_eq!(remotes_manager.get_all().len(), 1);

            // GQL: Update all remote
            let updated_instance_info = mutation_update_all_remotes(&query_service).await;

            // RT: Expect that one remote was updated
            assert_eq!(updated_instance_info.len(), 1);

            // Expect that the updated remote matches the remote in the remotes manager
            assert_eq!(remotes_manager.get(&instance_info.address()), Some(instance_info));
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const MUTATION_UPDATE_ALL_REMOTES: &str = include_str!("update_all.graphql");

async fn mutation_update_all_remotes(query_service: &Arc<dyn RuntimeQueryService + Send + Sync>) -> Vec<InstanceInfo> {
    let request = Request::new(MUTATION_UPDATE_ALL_REMOTES);
    let response = query_service.execute(request).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    println!("{}", data);
    let data: Mutation = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.remotes.update_all
}
