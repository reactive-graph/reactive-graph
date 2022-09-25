use crate::tests::utils::application::rw_application;
use futures_await_test::async_test;

use crate::application::Application;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
// use async_std::task;

// #[test]
#[async_test]
async fn test_dependency_injection_multi_threaded() {
    let application = rw_application();
    let _rw_app = application.read().unwrap();

    // assert!(!component_manager.has(String::from("named")));
    //
    // assert!(!rw_app.is_running());

    let r_application = application.clone();
    let _runner = async move {
        thread::sleep(Duration::from_millis(100));
        r_application.read().unwrap().init();
        // "run" is the only writer
        r_application.write().unwrap().run().await;
    };
    // run(application.clone());

    // let w_application = ;
    // let waiter = wait(application.clone());

    // let handle = thread::spawn(async move || {
    //     t_application.read().unwrap().init();
    //     // "run" is the only writer
    //     t_application.write().unwrap().run().await;
    //     // run(t_application);
    // });

    // assert!(!rw_app.is_running());
    //
    // let runner = async_std::task::spawn(runner);
    // let control = async_std::task::spawn(waiter);
    // async_std::task::block_on(control);
    // async_std::task::block_on(runner);
    // // futures::join!(runner, waiter);
    //
    // // handle.join().unwrap();
    // assert!(!rw_app.is_running());
}

async fn run(application: Arc<RwLock<dyn Application>>) {
    thread::sleep(Duration::from_millis(100));
    application.read().unwrap().init();
    // "run" is the only writer
    application.write().unwrap().run().await;
}

async fn wait(application: Arc<RwLock<dyn Application>>) {
    assert!(false);
    let application = application.read().unwrap();
    let component_manager = application.get_component_manager();
    assert!(!component_manager.has("named"));
    assert!(!application.is_running());
    thread::sleep(Duration::from_millis(200));
    assert!(component_manager.has("named"));
    assert!(application.is_running());
    application.stop();
    assert!(!application.is_running());
}
