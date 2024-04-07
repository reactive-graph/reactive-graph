use convert_case::Case::Camel;
use convert_case::Casing;
use default_test::DefaultTest;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Root;
use log4rs::Config;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::NamespacedTypeGetter;
use std::sync::Arc;
use std::time::Duration;

use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_impl::get_runtime;

#[tokio::test(flavor = "multi_thread")]
async fn test_dynamic_graph_query() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .expect("Failed to create logger");
    if let Err(error) = log4rs::init_config(config) {
        eprintln!("Failed to configure logger: {}", error);
    }

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

    let entity_type_manager = rt.get_entity_type_manager();
    println!("Entity Types: {}", entity_type_manager.count());
    let entity_type = EntityType::default_test();
    println!("{}", entity_type.ty);
    let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");

    let sdl = rt.get_dynamic_graph_schema_manager().get_dynamic_schema().await.unwrap().sdl();
    println!("{sdl}");

    let query = format!(
        r#"
            query GetShutdownEntity {{
              {} {{
                {} {{
                  id
                }}
              }}
            }}
        "#,
        entity_type.namespace().to_case(Camel),
        entity_type.type_name().to_case(Camel)
    );
    println!("{query}");
    let result = rt
        .get_dynamic_graph_query_service()
        .query(query) // QUERY_GET_SHUTDOWN_ENTITY.to_string()
        .await
        .expect("failed to query shutdown event");
    println!("{result}");
    // TODO: assert_eq(snapshot, result);
    rt.stop();
}
