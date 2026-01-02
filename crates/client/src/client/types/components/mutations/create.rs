#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;
    use crate::types::components::variables::create::variables::CreateComponentVariables;
    use crate::types::components::variables::create::variables::CreateComponentVariablesFields;

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
        #[arguments(type: $_type, description: $description, properties: $properties, extensions: $extensions
        )]
        pub create: Component,
    }

    pub fn create_component_mutation<C: Into<reactive_graph_graph::Component>>(component: C) -> Operation<CreateComponent, CreateComponentVariables> {
        use cynic::MutationBuilder;
        CreateComponent::build(CreateComponentVariables::new(component.into()))
    }
}
