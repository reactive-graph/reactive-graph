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
    use reactive_graph_graph::NamespacedTypeGetter;

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

    #[derive(cynic::QueryFragment, Debug)]
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
        #[arguments(type: { name: $name, namespace: $namespace }, description: $description, properties: $properties, extensions: $extensions
        )]
        pub create: Component,
    }

    pub fn create_component_mutation(component: reactive_graph_graph::Component) -> Operation<CreateComponent, CreateComponentVariables> {
        use cynic::MutationBuilder;
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

    pub fn create_component_with_variables(variables: CreateComponentVariables) -> Operation<CreateComponent, CreateComponentVariables> {
        use cynic::MutationBuilder;
        CreateComponent::build(variables)
    }
}
