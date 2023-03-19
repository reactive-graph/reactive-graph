use crate::get_runtime;
use std::env;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_graphql_query() {
    env::set_current_dir("../..").expect("Cant change directory to repository root dir");
    if let Err(error) = log4rs::init_file("./config/logging.toml", Default::default()) {
        eprintln!("Failed to configure logger: {}", error);
    }
    let rt = get_runtime();
    let runtime = rt.clone();
    tokio::spawn(async move {
        let runtime = runtime;
        runtime.init().await;
        runtime.post_init().await;
        runtime.run().await;
        runtime.pre_shutdown().await;
        runtime.shutdown().await;
    });
    rt.wait_for(Duration::from_secs(5)).await.expect("Runtime didn't came up");
    let query_string = String::from("query { instances { entities(label: \"/org/inexor/system/shutdown\") { id } } }");
    let result = rt
        .get_graphql_query_service()
        .query(query_string)
        .await
        .expect("failed to query shutdown event");
    // TODO: assert_eq(snapshot, result);
    rt.stop();
}
