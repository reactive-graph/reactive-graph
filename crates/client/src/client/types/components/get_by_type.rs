#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use reactive_graph_graph::ComponentTypeId;

    use crate::client::types::components::type_id::queries::ComponentTypeIdVariables;
    use crate::client::types::components::type_id::queries::ComponentTypeIdVariablesFields;
    use crate::schema_graphql::types::component::Component;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "ComponentTypeIdVariables")]
    pub struct GetComponentByType {
        pub types: GetComponentByTypeTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "ComponentTypeIdVariables")]
    pub struct GetComponentByTypeTypes {
        #[arguments(
            type: {
            namespace: $namespace,
            name: $name
            }
        )]
        pub components: Vec<Component>,
    }

    pub fn get_component_by_type_query(ty: &ComponentTypeId) -> Operation<GetComponentByType, ComponentTypeIdVariables> {
        use cynic::QueryBuilder;
        GetComponentByType::build(ty.clone().into())
    }

    #[cfg(test)]
    mod tests {
        use reactive_graph_runtime_impl::RuntimeBuilder;

        use reactive_graph_graph::ComponentTypeId;
        use reactive_graph_graph::Extensions;
        use reactive_graph_graph::PropertyTypes;

        #[tokio::test(flavor = "multi_thread")]
        async fn test_get_components_by_type() {
            let runtime = RuntimeBuilder::new()
                .ignore_config_files()
                .instance_name("Test Runtime Builder Get")
                .pick_free_port()
                .disable_all_plugins(true)
                .get();
            let ty = ComponentTypeId::new_from_type("test", "test");
            let component_manager = runtime.get_component_manager();
            let _component = component_manager
                .create_component(&ty, "", PropertyTypes::new(), Extensions::new())
                .expect("Failed to create component");
            // let inner_runtime = runtime.clone();
            let _port = runtime.get_config_manager().get_graphql_server_config().port();
        }
    }
}
