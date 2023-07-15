use std::sync::Arc;

use serde::Deserialize;

use crate::api::GraphQLQueryService;
use crate::config::InstanceAddress;
use crate::runtime::Runtime;
use crate::runtime::RuntimeBuilder;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct System {
    remotes: Vec<InstanceAddress>,
}

#[derive(Deserialize, Debug)]
struct Data {
    system: System,
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
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            let query_service = runtime.get_graphql_query_service();
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

const QUERY_GET_ALL_REMOTES: &str = include_str!("../../../../../graphql/system/remotes/get_all.graphql");

async fn query_get_all_remotes(query_service: &Arc<dyn GraphQLQueryService>) -> Vec<InstanceAddress> {
    let response = query_service.query_response(QUERY_GET_ALL_REMOTES).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.system.remotes
}
