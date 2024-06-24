#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::component::Component;
    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinitions;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct ComponentTypeIdVariables {
        namespace: String,
        name: String,
    }

    impl From<ComponentTypeId> for ComponentTypeIdVariables {
        fn from(ty: ComponentTypeId) -> Self {
            ComponentTypeIdVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
            }
        }
    }

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

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateComponentVariables {
        pub namespace: String,
        pub name: String,
        #[builder(default)]
        pub description: Option<String>,
        #[builder(default)]
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        #[builder(default)]
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateComponentVariables")]
    pub struct CreateComponent {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateComponentVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateComponentVariables")]
    pub struct MutationComponents {
        #[arguments(type: { name: $name, namespace: $namespace }, description: $description, properties: $properties, extensions: $extensions)]
        pub create: Component,
    }

    pub fn get_all_components_query() -> Operation<GetAllComponents, ()> {
        use cynic::QueryBuilder;
        GetAllComponents::build(())
    }

    pub fn get_component_by_type_query(ty: &ComponentTypeId) -> Operation<GetComponentByType, ComponentTypeIdVariables> {
        use cynic::QueryBuilder;
        GetComponentByType::build(ty.clone().into())
    }

    pub fn create_component_mutation(component: reactive_graph_graph::Component) -> Operation<CreateComponent, CreateComponentVariables> {
        use cynic::MutationBuilder;
        // let component = component.into();
        let namespace = component.namespace();
        let name = component.type_name();
        let description = component.description;
        let property_types: PropertyTypeDefinitions = component.properties.into();
        let extensions: ExtensionDefinitions = component.extensions.into();
        let vars = CreateComponentVariables {
            namespace,
            name,
            description: Some(description),
            properties: Some(property_types.0),
            extensions: Some(extensions.0),
        };
        CreateComponent::build(vars)
    }

    pub fn create_component_with_variables(variables: CreateComponentVariables) -> cynic::Operation<CreateComponent, CreateComponentVariables> {
        use cynic::MutationBuilder;
        CreateComponent::build(variables)
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

pub mod api {
    use std::sync::Arc;

    use cynic::http::ReqwestExt;

    use crate::client::types::components::queries::create_component_mutation;
    use crate::client::types::components::queries::create_component_with_variables;
    use crate::client::types::components::queries::get_all_components_query;
    use crate::client::types::components::queries::get_component_by_type_query;
    use crate::client::types::components::queries::CreateComponentVariables;
    use crate::client::InexorRgfClient;
    use crate::client::InexorRgfClientExecutionError;
    use crate::schema_graphql::types::component::Components as ComponentsVec;
    use reactive_graph_graph::Component;
    use reactive_graph_graph::ComponentTypeId;

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

        pub async fn get_component_by_type<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<Component>, InexorRgfClientExecutionError> {
            let component = self
                .client
                .client
                .post(self.client.url_graphql())
                .run_graphql(get_component_by_type_query(&ty.into()))
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .and_then(|data| data.types.components.first().cloned())
                .map(From::from);
            Ok(component)
        }

        pub async fn create_component(&self, component: Component) -> Result<Option<Component>, InexorRgfClientExecutionError> {
            let component = self
                .client
                .client
                .post(self.client.url_graphql())
                .run_graphql(create_component_mutation(component))
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .map(|data| data.types.components.create)
                .map(From::from);
            Ok(component)
        }

        pub async fn create_component_with_variables(&self, variables: CreateComponentVariables) -> Result<Option<Component>, InexorRgfClientExecutionError> {
            let component = self
                .client
                .client
                .post(self.client.url_graphql())
                .run_graphql(create_component_with_variables(variables))
                .await
                .map_err(InexorRgfClientExecutionError::FailedToSendRequest)?
                .data
                .map(|data| data.types.components.create)
                .map(From::from);
            Ok(component)
        }
    }
}
