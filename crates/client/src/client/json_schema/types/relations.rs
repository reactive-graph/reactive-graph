#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use serde_json::Value;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct JsonSchemaRelationTypes {
        pub json_schema: JsonSchema,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "JsonSchema")]
    pub struct JsonSchema {
        pub types: JsonSchemaTypeSystem,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "JsonSchemaTypeSystem")]
    pub struct JsonSchemaTypeSystem {
        pub relations: Value,
    }

    pub fn get_json_schema_for_relation_types() -> Operation<JsonSchemaRelationTypes, ()> {
        use cynic::QueryBuilder;
        JsonSchemaRelationTypes::build(())
    }

    #[cfg(all(test, feature = "integration-tests"))]
    mod tests {
        use reactive_graph_runtime_impl::RuntimeBuilder;
        use std::sync::Arc;

        use crate::ReactiveGraphClient;
        use reactive_graph_runtime_api::Runtime;

        #[tokio::test(flavor = "multi_thread")]
        async fn test_get_json_schema_for_relation_types() {
            RuntimeBuilder::new()
                .ignore_config_files()
                .instance_name("test_get_json_schema_for_relation_types")
                .pick_free_port()
                .disable_all_plugins(true)
                .init()
                .await
                .post_init()
                .await
                .spawn()
                .await
                .with_runtime(|runtime: Arc<dyn Runtime + Send + Sync>| async move {
                    let client = ReactiveGraphClient::new_from_runtime(runtime.clone()).expect("Failed to get client from runtime");
                    let json_schema = client
                        .json_schema()
                        .types()
                        .relations()
                        .await
                        .expect("Failed to get JSON Schema for relation types");
                    let json_schema = json_schema.expect("No JSON Schema for relation types was returned");
                    let json_schema = serde_json::to_string_pretty(&json_schema).expect("Failed to serialize JSON Schema for relation types");
                    println!("{}", json_schema);
                })
                .await
                .stop()
                .await
                .pre_shutdown()
                .await
                .shutdown()
                .await;
        }
    }
}
