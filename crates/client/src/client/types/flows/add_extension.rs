#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use crate::schema_graphql::types::flow_type::FlowType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddExtensionVariables {
        pub namespace: String,
        pub name: String,
        pub extension: ExtensionDefinition,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddExtensionVariables")]
    pub struct AddExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, extension: $extension)]
        pub add_extension: FlowType,
    }

    pub fn add_extension_mutation(
        ty: reactive_graph_graph::FlowTypeId,
        extension: reactive_graph_graph::Extension,
    ) -> Operation<AddExtension, AddExtensionVariables> {
        use cynic::MutationBuilder;
        let extension: ExtensionDefinition = extension.into();
        let vars = AddExtensionVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            extension,
        };
        AddExtension::build(vars)
    }

    pub fn add_extension_with_variables(variables: AddExtensionVariables) -> Operation<AddExtension, AddExtensionVariables> {
        use cynic::MutationBuilder;
        AddExtension::build(variables)
    }
}
