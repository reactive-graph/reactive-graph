#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;
    use crate::types::properties::container::queries::PropertyContainerVariables;
    use crate::types::properties::container::queries::PropertyContainerVariablesFields;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PropertyContainerVariables")]
    pub struct RemoveProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationComponents {
        #[arguments(type: { name: $name, namespace: $namespace }, propertyName: $property_name)]
        pub remove_property: Component,
    }

    pub fn remove_property_mutation(ty: reactive_graph_graph::ComponentTypeId, property_name: String) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        let vars = PropertyContainerVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            property_name,
        };
        RemoveProperty::build(vars)
    }

    pub fn remove_property_with_variables(variables: PropertyContainerVariables) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(variables)
    }
}