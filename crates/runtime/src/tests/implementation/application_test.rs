use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use crate::get_rw_runtime;
use futures_await_test::async_test;

use crate::runtime::Runtime;

#[async_test]
async fn test_dependency_injection_multi_threaded() {
    let runtime = get_rw_runtime();
    let _rw_app = runtime.read().unwrap();

    let r_application = runtime.clone();
    let _runner = async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        r_application.read().unwrap().init();
        // "run" is the only writer
        r_application.write().unwrap().run().await;
    };
}

async fn run(runtime: Arc<RwLock<dyn Runtime>>) {
    tokio::time::sleep(Duration::from_millis(100)).await;
    runtime.read().unwrap().init();
    // "run" is the only writer
    runtime.write().unwrap().run().await;
}

async fn wait(runtime: Arc<RwLock<dyn Runtime>>) {
    assert!(false);
    let rt = runtime.read().unwrap();
    let component_manager = rt.get_component_manager();
    assert!(!component_manager.has_by_type("core", "named"));
    assert!(!rt.is_running());
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(component_manager.has_by_type("core", "named"));
    assert!(rt.is_running());
    rt.stop();
    assert!(!rt.is_running());
}
