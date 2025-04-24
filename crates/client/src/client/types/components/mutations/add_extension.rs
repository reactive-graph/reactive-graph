#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariables;
    use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariablesFields;
    use crate::schema_graphql::types::component::Component;
    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddExtensionVariables")]
    pub struct AddExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationComponents {
        #[arguments(type: { name: $name, namespace: $namespace }, extension: $extension)]
        pub add_extension: Component,
    }

    pub fn add_extension_mutation(
        ty: reactive_graph_graph::ComponentTypeId,
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
