#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;
    use crate::types::properties::variables::container::variables::PropertyContainerVariables;
    use crate::types::properties::variables::container::variables::PropertyContainerVariablesFields;

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
        #[arguments(type: $_type, propertyName: $property_name)]
        pub remove_property: Component,
    }

    pub fn remove_property_mutation<C: Into<reactive_graph_graph::ComponentTypeId>>(
        ty: C,
        property_name: String,
    ) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(PropertyContainerVariables::new(ty.into(), property_name))
    }
}
