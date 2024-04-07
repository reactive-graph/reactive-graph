use std::sync::Arc;

use async_graphql::Request;
use serde::Deserialize;

use reactive_graph_remotes_model::InstanceAddress;
use reactive_graph_remotes_model::InstanceInfo;
use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_graphql_api::RuntimeQueryService;
use reactive_graph_runtime_impl::RuntimeBuilder;

use crate::tests::util::address_to_vars;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Mutation {
    remotes: MutationSystemUpdateRemote,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MutationSystemUpdateRemote {
    update: InstanceInfo,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_update_remote() {
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

            // GQL: Update remote
            let updated_instance_info = mutation_update_remote(&query_service, &address).await;

            // Compare local instance info with returned instance info
            assert_eq!(updated_instance_info, instance_info);

            // RT: Expect that one remote exist
            assert_eq!(remotes_manager.get_all().len(), 1);
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const MUTATION_UPDATE_REMOTE: &str = include_str!("update.graphql");

async fn mutation_update_remote(query_service: &Arc<dyn RuntimeQueryService + Send + Sync>, address: &InstanceAddress) -> InstanceInfo {
    let request = Request::new(MUTATION_UPDATE_REMOTE).variables(address_to_vars(address));
    let response = query_service.execute(request).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    println!("{}", data);
    let data: Mutation = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.remotes.update
}
