#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::properties::variables::container::variables::PropertyContainerVariables;
    use crate::client::types::properties::variables::container::variables::PropertyContainerVariablesFields;
    use crate::schema_graphql::types::entity_type::EntityType;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PropertyContainerVariables")]
    pub struct RemoveProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, propertyName: $property_name)]
        pub remove_property: EntityType,
    }

    pub fn remove_property_mutation(ty: reactive_graph_graph::EntityTypeId, property_name: String) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(PropertyContainerVariables::new(ty, property_name))
    }

    pub fn remove_property_with_variables(variables: PropertyContainerVariables) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(variables)
    }
}
