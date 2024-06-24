#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::entity_type::EntityType;
    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinitions;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct EntityTypeTypeIdVariables {
        namespace: String,
        name: String,
    }

    impl From<EntityTypeId> for EntityTypeTypeIdVariables {
        fn from(ty: EntityTypeId) -> Self {
            EntityTypeTypeIdVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
            }
        }
    }

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

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "EntityTypeTypeIdVariables")]
    pub struct GetEntityTypeByType {
        pub types: GetEntityTypeByTypeTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Types", variables = "EntityTypeTypeIdVariables")]
    pub struct GetEntityTypeByTypeTypes {
        #[arguments(
          type: {
            namespace: $namespace,
            name: $name
          }
        )]
        pub entities: Vec<EntityType>,
    }

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateEntityTypeVariables {
        pub namespace: String,
        pub name: String,
        #[builder(default)]
        pub description: Option<String>,
        #[builder(default)]
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        #[builder(default)]
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateEntityTypeVariables")]
    pub struct CreateEntityType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateEntityTypeVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateEntityTypeVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, description: $description, properties: $properties, extensions: $extensions)]
        pub create: EntityType,
    }

    pub fn get_all_entity_types_query() -> Operation<GetAllEntityTypes, ()> {
        use cynic::QueryBuilder;
        GetAllEntityTypes::build(())
    }

    pub fn get_entity_type_by_type_query(ty: &EntityTypeId) -> Operation<GetEntityTypeByType, EntityTypeTypeIdVariables> {
        use cynic::QueryBuilder;
        GetEntityTypeByType::build(ty.clone().into())
    }

    pub fn create_entity_type_mutation(entity_type: reactive_graph_graph::EntityType) -> Operation<CreateEntityType, CreateEntityTypeVariables> {
        use cynic::MutationBuilder;
        // let entity_type = entity_type.into();
        let namespace = entity_type.namespace();
        let name = entity_type.type_name();
        let description = entity_type.description;
        let property_types: PropertyTypeDefinitions = entity_type.properties.into();
        let extensions: ExtensionDefinitions = entity_type.extensions.into();
        let vars = CreateEntityTypeVariables {
            namespace,
            name,
            description: Some(description),
            properties: Some(property_types.0),
            extensions: Some(extensions.0),
        };
        CreateEntityType::build(vars)
    }

    pub fn create_entity_type_with_variables(variables: CreateEntityTypeVariables) -> Operation<CreateEntityType, CreateEntityTypeVariables> {
        use cynic::MutationBuilder;
        CreateEntityType::build(variables)
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

    use crate::client::types::entity_types::queries::create_entity_type_mutation;
    use crate::client::types::entity_types::queries::create_entity_type_with_variables;
    use crate::client::types::entity_types::queries::get_all_entity_types_query;
    use crate::client::types::entity_types::queries::get_entity_type_by_type_query;
    use crate::client::types::entity_types::queries::CreateEntityTypeVariables;
    use crate::client::InexorRgfClient;
    use crate::client::InexorRgfClientExecutionError;
    use crate::schema_graphql::types::entity_type::EntityTypes as EntityTypesVec;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::EntityTypeId;

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

        pub async fn get_entity_type_by_type<C: Into<EntityTypeId>>(&self, ty: C) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
            let entity_type = self
                .client
                .client
                .post(self.client.url_graphql())
                .run_graphql(get_entity_type_by_type_query(&ty.into()))
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .and_then(|data| data.types.entities.first().cloned())
                .map(From::from);
            Ok(entity_type)
        }

        pub async fn create_entity_type(&self, entity_type: EntityType) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
            let entity_type = self
                .client
                .client
                .post(self.client.url_graphql())
                .run_graphql(create_entity_type_mutation(entity_type))
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .map(|data| data.types.entities.create)
                .map(From::from);
            Ok(entity_type)
        }

        pub async fn create_entity_type_with_variables(
            &self,
            variables: CreateEntityTypeVariables,
        ) -> Result<Option<EntityType>, InexorRgfClientExecutionError> {
            let entity_type = self
                .client
                .client
                .post(self.client.url_graphql())
                .run_graphql(create_entity_type_with_variables(variables))
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .map(|data| data.types.entities.create)
                .map(From::from);
            Ok(entity_type)
        }
    }
}
