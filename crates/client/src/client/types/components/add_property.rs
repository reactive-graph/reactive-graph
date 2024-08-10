#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::component::Component;
    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddPropertyVariables {
        pub namespace: String,
        pub name: String,
        pub property: PropertyTypeDefinition,
    }

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
        #[arguments(type: { name: $name, namespace: $namespace }, property: $property)]
        pub add_property: Component,
    }

    pub fn add_property_mutation(
        ty: reactive_graph_graph::ComponentTypeId,
        property_type: reactive_graph_graph::PropertyType,
    ) -> Operation<AddProperty, AddPropertyVariables> {
        use cynic::MutationBuilder;
        let extensions: ExtensionDefinitions = property_type.extensions.into();
        let vars = AddPropertyVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            property: PropertyTypeDefinition {
                name: property_type.name,
                data_type: property_type.data_type.into(),
                description: property_type.description,
                socket_type: property_type.socket_type.into(),
                mutability: property_type.mutability.into(),
                extensions: extensions.0,
            },
        };
        AddProperty::build(vars)
    }

    pub fn add_property_with_variables(variables: AddPropertyVariables) -> Operation<AddProperty, AddPropertyVariables> {
        use cynic::MutationBuilder;
        AddProperty::build(variables)
    }
}
