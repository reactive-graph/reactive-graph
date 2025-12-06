#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::properties::variables::add_property::variables::AddPropertyVariables;
    use crate::client::types::properties::variables::add_property::variables::AddPropertyVariablesFields;
    use crate::schema_graphql::types::component::Component;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::PropertyType;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddPropertyVariables")]
    pub struct AddProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertyVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertyVariables")]
    pub struct MutationComponents {
        #[arguments(type: $_type, property: $property)]
        pub add_property: Component,
    }

    pub fn add_property_mutation<C: Into<ComponentTypeId>, PT: Into<PropertyType>>(ty: C, property_type: PT) -> Operation<AddProperty, AddPropertyVariables> {
        use cynic::MutationBuilder;
        AddProperty::build(AddPropertyVariables::new(ty.into(), property_type.into()))
    }
}
