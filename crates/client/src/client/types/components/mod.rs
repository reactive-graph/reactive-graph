use std::sync::Arc;

use cynic::http::ReqwestExt;
use inexor_rgf_core_model::ComponentTypeId;

use crate::client::types::components::queries::create_component;
use crate::client::types::components::queries::create_component_mutation;
use crate::client::types::components::queries::get_all_components_query;
use crate::client::types::components::queries::get_component_by_type_query;
use crate::client::types::components::queries::CreateComponentVariables;
use crate::client::InexorRgfClient;
use crate::client::InexorRgfClientExecutionError;
use crate::schema::component::Component;

#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "crate::schema::schema")]
pub mod queries {
    use typed_builder::TypedBuilder;

    use crate::model::ComponentTypeId;
    use crate::model::NamespacedTypeGetter;
    use crate::schema::component::Component;
    use crate::schema::extension::ExtensionDefinition;
    use crate::schema::extension::ExtensionDefinitions;
    use crate::schema::property_type::PropertyTypeDefinition;
    use crate::schema::property_type::PropertyTypeDefinitions;

    #[derive(cynic::QueryVariables, Debug, TypedBuilder)]
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

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetAllComponents {
        pub types: GetAllComponentsTypes,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Types")]
    pub struct GetAllComponentsTypes {
        pub components: Vec<Component>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "ComponentTypeIdVariables")]
    pub struct GetComponentByType {
        pub types: GetComponentByTypeTypes,
    }

    #[derive(cynic::QueryFragment, Debug)]
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

    #[derive(cynic::QueryVariables, Debug, TypedBuilder)]
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

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateComponentVariables")]
    pub struct CreateComponent {
        pub types: MutationTypes,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "CreateComponentVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(variables = "CreateComponentVariables")]
    pub struct MutationComponents {
        #[arguments(type: { name: $name, namespace: $namespace }, description: $description, properties: $properties, extensions: $extensions)]
        pub create: Component,
    }

    pub fn get_all_components_query() -> cynic::Operation<GetAllComponents, ()> {
        use cynic::QueryBuilder;
        GetAllComponents::build(())
    }

    pub fn get_component_by_type_query(ty: &ComponentTypeId) -> cynic::Operation<GetComponentByType, ComponentTypeIdVariables> {
        use cynic::QueryBuilder;
        GetComponentByType::build(ty.clone().into())
    }

    pub fn create_component_mutation(component: crate::model::Component) -> cynic::Operation<CreateComponent, CreateComponentVariables> {
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

    pub fn create_component(variables: CreateComponentVariables) -> cynic::Operation<CreateComponent, CreateComponentVariables> {
        use cynic::MutationBuilder;
        CreateComponent::build(variables)
    }

    #[cfg(test)]
    mod tests {
        use inexor_rgf_rt::runtime::RuntimeBuilder;

        use crate::model::ComponentTypeId;

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
            let component = component_manager.create(&ty, "", Vec::new(), Vec::new()).expect("Failed to create component");
            // let inner_runtime = runtime.clone();
            let port = runtime.get_config_manager().get_graphql_server_config().port();
            test_get_components_by_type()
        }
    }
}

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
            .post(self.client.url())
            .run_graphql(get_all_components_query())
            .await
            .map_err(|e| InexorRgfClientExecutionError::FailedToSendRequest(e))?
            .data
            .map(|data| data.types.components);
        Ok(components)
    }

    pub async fn get_component_by_type<C: Into<ComponentTypeId>>(&self, ty: C) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url())
            .run_graphql(get_component_by_type_query(&ty.into()))
            .await
            .map_err(|e| InexorRgfClientExecutionError::FailedToSendRequest(e))?
            .data
            .and_then(|data| data.types.components.first().cloned());
        Ok(component)
    }

    pub async fn create_component(&self, component: crate::model::Component) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url())
            .run_graphql(create_component_mutation(component))
            .await
            .map_err(|e| InexorRgfClientExecutionError::FailedToSendRequest(e))?
            .data
            .map(|data| data.types.components.create);
        Ok(component)
    }

    pub async fn create_component_2(&self, variables: CreateComponentVariables) -> Result<Option<Component>, InexorRgfClientExecutionError> {
        let component = self
            .client
            .client
            .post(self.client.url())
            .run_graphql(create_component(variables))
            .await
            .map_err(|e| InexorRgfClientExecutionError::FailedToSendRequest(e))?
            .data
            .map(|data| data.types.components.create.into());
        Ok(component)
    }
}
