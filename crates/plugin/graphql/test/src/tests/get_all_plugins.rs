use std::sync::Arc;

use serde::Deserialize;
use uuid::Uuid;

use inexor_rgf_plugin_graphql_api::PluginQueryService;
use inexor_rgf_runtime_api::Runtime;
use inexor_rgf_runtime_impl::RuntimeBuilder;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    plugins: Vec<Plugin>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Plugin {
    id: Uuid,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_all_plugins() {
    inexor_rgf_test_utils::init_logger();
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
            let query_service = runtime.plugin_graphql_system().get_plugin_query_service();
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

const QUERY_GET_ALL_PLUGIN_IDS: &str = include_str!("./get_all_ids.graphql");

async fn query_get_all_plugins(query_service: &Arc<dyn PluginQueryService + Send + Sync>) -> Vec<Plugin> {
    let response = query_service.query_response(QUERY_GET_ALL_PLUGIN_IDS).await;
    assert!(response.errors.is_empty());
    let data = response.data.into_json().expect("Failed to get json data from graphql response");
    let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
    data.plugins
}
