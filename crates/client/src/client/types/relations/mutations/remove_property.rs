#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::properties::variables::container::variables::PropertyContainerVariables;
    use crate::client::types::properties::variables::container::variables::PropertyContainerVariablesFields;
    use crate::schema_graphql::types::relation_type::RelationType;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PropertyContainerVariables")]
    pub struct RemoveProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, propertyName: $property_name)]
        pub remove_property: RelationType,
    }

    pub fn remove_property_mutation(ty: reactive_graph_graph::RelationTypeId, property_name: String) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(PropertyContainerVariables::new(ty, property_name))
    }

    pub fn remove_property_with_variables(variables: PropertyContainerVariables) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(variables)
    }
}
