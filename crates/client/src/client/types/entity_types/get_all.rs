#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::entity_type::EntityType;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllEntityTypes {
        pub types: GetAllEntityTypesTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types")]
    pub struct GetAllEntityTypesTypes {
        pub entities: Vec<EntityType>,
    }

    pub fn get_all_entity_types_query() -> Operation<GetAllEntityTypes, ()> {
        use cynic::QueryBuilder;
        GetAllEntityTypes::build(())
    }

    #[cfg(test)]
    mod tests {
        use reactive_graph_runtime_impl::RuntimeBuilder;

        use reactive_graph_graph::ComponentTypeIds;
        use reactive_graph_graph::EntityTypeId;
        use reactive_graph_graph::Extensions;
        use reactive_graph_graph::PropertyTypes;

        #[tokio::test(flavor = "multi_thread")]
        async fn test_get_entity_types_by_type() {
            let runtime = RuntimeBuilder::new()
                .ignore_config_files()
                .instance_name("Test Runtime Builder Get")
                .pick_free_port()
                .disable_all_plugins(true)
                .get();
            let ty = EntityTypeId::new_from_type("test", "test");
            let entity_type_manager = runtime.get_entity_type_manager();
            let _entity_type = entity_type_manager
                .create_entity_type(&ty, "", ComponentTypeIds::new(), PropertyTypes::new(), Extensions::new())
                .expect("Failed to create entity_type");
            // let inner_runtime = runtime.clone();
            let _port = runtime.get_config_manager().get_graphql_server_config().port();
        }
    }
}

pub mod api {
    use std::sync::Arc;

    use cynic::http::ReqwestExt;

    use crate::client::InexorRgfClient;
    use crate::client::InexorRgfClientExecutionError;
    use crate::schema_graphql::types::entity_type::EntityTypes as EntityTypesVec;
    use crate::types::entity_types::get_all::queries::get_all_entity_types_query;
    use reactive_graph_graph::EntityType;

    pub struct EntityTypes {
        client: Arc<InexorRgfClient>,
    }

    impl EntityTypes {
        pub fn new(client: Arc<InexorRgfClient>) -> Self {
            Self { client }
        }

        pub async fn get_all_entity_types(&self) -> Result<Option<Vec<EntityType>>, InexorRgfClientExecutionError> {
            let entity_types = self
                .client
                .client
                .post(self.client.url_graphql())
                .run_graphql(get_all_entity_types_query())
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .map(|data| EntityTypesVec(data.types.entities))
                .map(From::from);
            Ok(entity_types)
        }
    }
}
