use std::sync::Arc;

use serde::Deserialize;

use inexor_rgf_remotes_model::InstanceInfo;
use inexor_rgf_remotes_model::DEFAULT_HOSTNAME;
use inexor_rgf_runtime_api::Runtime;
use inexor_rgf_runtime_impl::RuntimeBuilder;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    instance_info: InstanceInfo,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_instance_info() {
    RuntimeBuilder::new()
        .ignore_config_files()
        .disable_all_plugins(true)
        .instance_name("iojasdf")
        .instance_description("oijasdfnmei")
        .port(7839)
        .init()
        .await
        .post_init()
        .await
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            // let query_service = runtime.get_graphql_query_service();
            let query_service = runtime.get_runtime_query_service();

            const QUERY_INSTANCE_INFO: &str = include_str!("get_instance_info.graphql");

            let response = query_service.query_response(QUERY_INSTANCE_INFO).await;
            assert!(response.errors.is_empty());
            let data = response.data.into_json().expect("Failed to get json data from graphql response");
            let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
            assert_eq!(data.instance_info.name, "iojasdf");
            assert_eq!(data.instance_info.description, "oijasdfnmei");
            assert_eq!(data.instance_info.address.hostname, DEFAULT_HOSTNAME);
            assert_eq!(data.instance_info.address.port, 7839);
            assert_eq!(data.instance_info.address.secure, false);
        })
        .await
        .do_not_run()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}
