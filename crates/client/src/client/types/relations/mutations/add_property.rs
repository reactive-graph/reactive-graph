#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::properties::variables::add_property::variables::AddPropertyVariables;
    use crate::client::types::properties::variables::add_property::variables::AddPropertyVariablesFields;
    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use crate::schema_graphql::types::relation_type::RelationType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddPropertyVariables")]
    pub struct AddProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertyVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertyVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, property: $property)]
        pub add_property: RelationType,
    }

    pub fn add_property_mutation(
        ty: reactive_graph_graph::RelationTypeId,
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
