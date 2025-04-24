#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::common::variables::type_id::variables::TypeIdVariables;
    use crate::client::types::common::variables::type_id::variables::TypeIdVariablesFields;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "TypeIdVariables")]
    pub struct DeleteComponent {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "TypeIdVariables")]
    pub struct MutationComponents {
        #[arguments(type: { name: $name, namespace: $namespace })]
        pub delete: bool,
    }

    pub fn delete_component_mutation<C: Into<reactive_graph_graph::ComponentTypeId>>(ty: C) -> Operation<DeleteComponent, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteComponent::build(ty.into().into())
    }

    pub fn delete_component_with_variables(variables: TypeIdVariables) -> Operation<DeleteComponent, TypeIdVariables> {
        use cynic::MutationBuilder;
        DeleteComponent::build(variables)
    }
}
