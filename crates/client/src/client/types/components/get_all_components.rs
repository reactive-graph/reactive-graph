#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {

    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllComponents {
        pub types: GetAllComponentsTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types")]
    pub struct GetAllComponentsTypes {
        pub components: Vec<Component>,
    }

    pub fn get_all_components_query() -> Operation<GetAllComponents, ()> {
        use cynic::QueryBuilder;
        GetAllComponents::build(())
    }

    pub mod api {
        use std::sync::Arc;

        use cynic::http::ReqwestExt;

        // use crate::client::types::components::queries::create_component_mutation;
        // use crate::client::types::components::queries::create_component_with_variables;
        use crate::client::types::components::get_all_components::queries::get_all_components_query;
        // use crate::client::types::components::queries::get_component_by_type_query;
        // use crate::client::types::components::queries::CreateComponentVariables;
        use crate::client::InexorRgfClient;
        use crate::client::InexorRgfClientExecutionError;
        use crate::schema_graphql::types::component::Components as ComponentsVec;
        use reactive_graph_graph::Component;
        // use reactive_graph_graph::ComponentTypeId;

        pub struct Components {
            client: Arc<InexorRgfClient>,
        }

        impl Components {
            pub fn new(client: Arc<InexorRgfClient>) -> Self {
                Self { client }
            }

            pub async fn get_all_components(&self) -> Result<Option<Vec<Component>>, InexorRgfClientExecutionError> {
                let components = self
                    .client
                    .client
                    .post(self.client.url_graphql())
                    .run_graphql(get_all_components_query())
                    .await
                    .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                    .data
                    .map(|data| ComponentsVec(data.types.components))
                    .map(From::from);
                Ok(components)
            }
        }
    }
}
