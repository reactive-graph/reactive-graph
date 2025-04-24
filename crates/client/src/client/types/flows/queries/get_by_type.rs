#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::flow_type::FlowType;
    use reactive_graph_graph::FlowTypeId;

    use crate::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::types::common::variables::type_id::variables::TypeIdVariablesFields;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "TypeIdVariables")]
    pub struct GetFlowTypeByType {
        pub types: GetFlowTypeByTypeTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "TypeIdVariables")]
    pub struct GetFlowTypeByTypeTypes {
        #[arguments(
          type: {
            namespace: $namespace,
            name: $name
          }
        )]
        pub flows: Vec<FlowType>,
    }

    pub fn get_flow_type_by_type_query(ty: &FlowTypeId) -> Operation<GetFlowTypeByType, TypeIdVariables> {
        use cynic::QueryBuilder;
        GetFlowTypeByType::build(ty.clone().into())
    }

    #[cfg(all(test, feature = "integration-tests"))]
    mod tests {
        use reactive_graph_runtime_impl::RuntimeBuilder;

        use reactive_graph_graph::ComponentTypeIds;
        use reactive_graph_graph::Extensions;
        use reactive_graph_graph::FlowTypeId;
        use reactive_graph_graph::PropertyTypes;

        #[tokio::test(flavor = "multi_thread")]
        async fn test_get_flow_types_by_type() {
            let runtime = RuntimeBuilder::new()
                .ignore_config_files()
                .instance_name("Test Get Flow Type By Type")
                .pick_free_port()
                .disable_all_plugins(true)
                .get();
            let ty = FlowTypeId::new_from_type("test", "test");
            // let flow_type_manager = runtime.get_flow_type_manager();
            // let entity_type_manager = runtime.get_entity_type_manager();
            // let entity_instance_manager = runtime.get_entity_type_manager();
            //
            // let _flow_type = flow_type_manager
            //     .create_flow_type(&ty, "", ComponentTypeIds::new(), PropertyTypes::new(), Extensions::new())
            //     .expect("Failed to create flow_type");
            // // let inner_runtime = runtime.clone();
            let _port = runtime.get_config_manager().get_graphql_server_config().port();
        }
    }
}
