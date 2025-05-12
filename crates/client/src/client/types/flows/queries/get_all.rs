#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::flow_type::FlowType;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllFlowTypes {
        pub types: GetAllFlowTypesTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types")]
    pub struct GetAllFlowTypesTypes {
        pub flows: Vec<FlowType>,
    }

    pub fn get_all_flow_types_query() -> Operation<GetAllFlowTypes, ()> {
        use cynic::QueryBuilder;
        GetAllFlowTypes::build(())
    }

    #[cfg(all(test, feature = "integration-tests"))]
    mod tests {
        use reactive_graph_runtime_impl::RuntimeBuilder;

        #[tokio::test(flavor = "multi_thread")]
        async fn test_get_flow_types_by_type() {
            let runtime = RuntimeBuilder::new()
                .ignore_config_files()
                .instance_name("Test Get All Flow Types")
                .pick_free_port()
                .disable_all_plugins(true)
                .get();
            // let ty = FlowTypeId::new_from_type("test", "test");
            // let flow_type_manager = runtime.get_flow_type_manager();
            // let _flow_type = flow_type_manager
            //     .create_flow_type(&ty, "", ComponentTypeIds::new(), PropertyTypes::new(), Extensions::new())
            //     .expect("Failed to create flow_type");
            // let inner_runtime = runtime.clone();
            let _port = runtime.get_config_manager().get_graphql_server_config().port();
        }
    }
}
