use inexor_rgf_core_builder::EntityTypeBuilder;
use inexor_rgf_core_builder::ReactiveEntityInstanceBuilder;
use inexor_rgf_core_model::DataType;
use inexor_rgf_core_model::EntityTypeId;
use inexor_rgf_rt::runtime::Runtime;
use inexor_rgf_rt::runtime::RuntimeBuilder;
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ()> {
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
        // The runtime can be used using the (rust) APIs and using the GraphQL API
        //
        .with_runtime(|runtime: Arc<dyn Runtime>| async move {
            // The entity type manager manages the entity types
            let entity_type_manager = runtime.get_entity_type_manager();

            // Define the type id (the name of the type plus the namespace)
            let ty = EntityTypeId::new_from_type("example", "example");

            // Now use a builder to construct an entity type for the type id
            let entity_type = EntityTypeBuilder::new(ty)
                .description("An example entity type")
                .input_property("your_name", DataType::String)
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
            let entity_instance_manager = runtime.get_reactive_entity_instance_manager();

            // Based on the entity type we can use the builder pattern to initialize the entity instance
            let entity_instance = ReactiveEntityInstanceBuilder::from(entity_type)
                .property("your_name", json!("Peter Penacka"))
                .build();
            // The entity instance is reactive but not yet registered and unknown to the system.
            // Therefore we have to register it
            let entity_instance = entity_instance_manager
                .register_reactive_instance(entity_instance)
                .expect("Failed to register reactive entity instance");

            // Check if it has been registered
            if entity_instance_manager.has(entity_instance.id) {
                println!("Successfully registered entity instance!");
            }

            // Ok, lets simulate a GraphQL query!
            // For example we want to query for the entity type and the entity instance we just created:
            let query = r#"
query {
  types {
    entities(type: { namespace: "example", name: "example" }) {
      namespace
      name
    }
  }
  instances {
    entities(type: { namespace: "example", name: "example" }) {
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
