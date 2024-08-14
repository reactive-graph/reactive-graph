#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::entity_type::EntityType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct RemovePropertyVariables {
        pub namespace: String,
        pub name: String,
        pub property_name: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RemovePropertyVariables")]
    pub struct RemoveProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemovePropertyVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemovePropertyVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, propertyName: $property_name)]
        pub remove_property: EntityType,
    }

    pub fn remove_property_mutation(ty: reactive_graph_graph::EntityTypeId, property_name: String) -> Operation<RemoveProperty, RemovePropertyVariables> {
        use cynic::MutationBuilder;
        let vars = RemovePropertyVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            property_name,
        };
        RemoveProperty::build(vars)
    }

    pub fn remove_property_with_variables(variables: RemovePropertyVariables) -> Operation<RemoveProperty, RemovePropertyVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(variables)
    }
}
