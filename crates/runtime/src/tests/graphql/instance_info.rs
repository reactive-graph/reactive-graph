
use std::sync::Arc;

use serde::Deserialize;

use inexor_rgf_model_runtime::InstanceInfo;

use crate::runtime::Runtime;
use crate::runtime::RuntimeBuilder;

const QUERY_INSTANCE_INFO: &str = r#"
query InstanceInfo {
  system {
    instanceInfo {
      name
      description
      hostname
      port
      secure
      version
      buildDate
      gitBranch
      gitCommit
      rustcVersion
      pluginApiVersion
    }
  }
}
"#;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct System {
    instance_info: InstanceInfo,
}

#[derive(Deserialize, Debug)]
struct Data {
    system: System,
}

#[tokio::test(flavor = "multi_thread")]
async fn test_graphql() {
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
            let query_service = runtime.get_graphql_query_service();
            let response = query_service.query_response(QUERY_INSTANCE_INFO).await;
            assert!(response.errors.is_empty());
            let data = response.data.into_json().expect("Failed to get json data from graphql response");
            let data: Data = serde_json::from_value(data).expect("Failed to deserialize json into target data model");
            assert_eq!(data.system.instance_info.name, "iojasdf");
            assert_eq!(data.system.instance_info.description, "oijasdfnmei");
            assert_eq!(data.system.instance_info.port, 7839);
        })
        .await
        .do_not_run()
        .await
        .pre_shutdown()
        .await
        .shutdown()
        .await;
}
