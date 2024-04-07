use std::sync::Arc;
use std::time::Duration;

use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_impl::get_runtime;

const QUERY_GET_SHUTDOWN_ENTITY: &str = include_str!("./get_shutdown_entity.graphql");

#[tokio::test(flavor = "multi_thread")]
async fn test_graphql_query() {
    let rt: Arc<dyn Runtime + Send + Sync> = get_runtime();
    let runtime = rt.clone();
    tokio::spawn(async move {
        let runtime = runtime;
        runtime.init().await;
        runtime.post_init().await;
        runtime.run().await;
        runtime.pre_shutdown().await;
        runtime.shutdown().await;
    });
    rt.wait_for_started(Duration::from_secs(5)).await.expect("Runtime didn't came up");
    let result = rt
        .get_graphql_query_service()
        .query(QUERY_GET_SHUTDOWN_ENTITY)
        .await
        .expect("Failed to query shutdown entity");
    println!("{result}");
    // TODO: assert_eq(snapshot, result);
    rt.stop();
}
