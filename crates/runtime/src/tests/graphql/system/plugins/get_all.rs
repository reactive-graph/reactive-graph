use std::sync::Arc;

use serde::Deserialize;
use uuid::Uuid;

use crate::api::GraphQLQueryService;
use crate::runtime::Runtime;
use crate::runtime::RuntimeBuilder;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Plugin {
    id: Uuid,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct System {
    plugins: Vec<Plugin>,
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
            let plugin_container_manager = runtime.get_plugin_container_manager();
            let rt_plugins = plugin_container_manager.get_plugins();
            let gql_plugins: Vec<Uuid> = query_get_all_plugins(&query_service).await.iter().map(|plugin| plugin.id).collect();
            assert_eq!(rt_plugins, gql_plugins);
        })
        .await
        .stop()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}

const QUERY_GET_ALL_PLUGIN_IDS: &str = include_str!("../../../../../graphql/system/plugins/get_all_ids.graphql");

async fn query_get_all_plugins(query_service: &Arc<dyn GraphQLQueryService>) -> Vec<Plugin> {
    let response = query_service.query_response(QUERY_GET_ALL_PLUGIN_IDS).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.system.plugins
}
