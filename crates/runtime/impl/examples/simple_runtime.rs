use std::str::FromStr;
use std::sync::Arc;

use reactive_graph_graph::EntityType;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_runtime_api::Runtime;
use reactive_graph_runtime_impl::RuntimeBuilder;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Define the type id (the name of the type plus the namespace)
    let ty = EntityTypeId::from_str("example::example").expect("Failed to construct the entity type id");

    RuntimeBuilder::new()
        // For this example we dont want to use the configuration files
        .ignore_config_files()
        // For this example we dont want to use any plugins
        .disable_all_plugins(true)
        // Don't disturb any network service
        .pick_free_port()
        // Initialize the runtime gracefully
        .init()
        .await
        .post_init()
        .await
        // The runtime is initialized at this point and can be used.
        // The GraphQL HTTP-Server is not started yet (and we won't start it in this example)
        //
        // Using the rust APIs and / or the GraphQL API
        //
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            // The entity type manager manages the entity types
            let entity_type_manager = runtime.get_entity_type_manager();

            // Now use a builder to construct an entity type for the type id
            let entity_type = EntityType::builder()
                .ty(ty)
                .description("An example entity type")
                .properties(PropertyTypes::new().property(PropertyType::string_input("your_name")))
                .build();

            // Before we can use the entity type it has to be registered on the entity type manager.
            // This makes the entity type known to the whole system
            let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type!");

            // Check if it has been registered
            if entity_type_manager.has(&entity_type.ty) {
                println!("Successfully registered entity type!");
            }

            // Now we want to create an entity instance based on the entity type.

            // First: we need the REACTIVE entity instance manager:
            let reactive_entity_manager = runtime.get_reactive_entity_manager();

            // Based on the entity type we can use the builder pattern to initialize the entity instance
            let reactive_entity = ReactiveEntity::builder_from_entity_type(&entity_type).build();

            // The entity instance is reactive but not yet registered and unknown to the runtime.
            // Therefore we have to register it
            let reactive_entity = reactive_entity_manager
                .register_reactive_instance(reactive_entity)
                .expect("Failed to register reactive entity instance");

            // Check if it has been registered
            if reactive_entity_manager.has(reactive_entity.id) {
                println!("Successfully registered entity instance!");
            }

            // Ok, lets simulate a GraphQL query!
            // For example, we want to query for the entity type and the entity instance we just created:
            let query = r#"
query {
  types {
    entities(type: "example::example") {
      namespace
      name
    }
  }
  instances {
    entities(type: "example::example") {
      id
      properties {
        name
        value
      }
    }
  }
}
            "#;
            println!("\n===== GraphQL Query =====\n{}", query);

            // We need the GraphQL query service
            let query_service = runtime.get_graphql_query_service();

            let query = async_graphql::Request::new(query);

            let response = query_service.execute(query).await;

            let data = serde_json::to_string_pretty(&response.data).expect("Failed to parse JSON");
            println!("\n===== GraphQL Response =====\n{}", data);
            response.errors.iter().for_each(|e| println!("Error: {}", e));
        }) // Going out of scope -> continue
        .await
        // Let's tell the builder that we don't want to start the HTTP/GraphQL server.
        .do_not_run()
        .await
        // Shutdown gracefully
        .pre_shutdown()
        .await
        .shutdown()
        .await;
    Ok(())
}
