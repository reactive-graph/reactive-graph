use crate::Plugin;

/// Fake plugin
struct TestPlugin {}
impl Plugin for TestPlugin {}

#[tokio::test(flavor = "multi_thread")]
async fn plugin_api_default_trait_impl_test() {
    let plugin = TestPlugin {};
    assert_eq!(true, plugin.activate().await.is_ok());
    assert_eq!(true, plugin.deactivate().await.is_ok());
}
